use std::collections::HashMap;
use std::fmt::Write;

use context_error::{BoxedError, Context, CreateError};

use crate::{structs::*, ErrorLevel};

/// Validate the SEQRES data found, if there is any
#[allow(
    clippy::comparison_chain,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub(crate) fn validate_seqres(
    pdb: &mut PDB,
    sequence: HashMap<String, Vec<(usize, usize, Vec<String>)>>,
    lines: &[String],
    start_linenumber: usize,
    context: &Context<'static>,
) -> Vec<BoxedError<'static, ErrorLevel>> {
    let mut errors = Vec::new();
    for (chain_id, data) in sequence {
        if let Some(chain) = pdb.chains_mut().find(|c| c.id() == chain_id) {
            let mut chain_sequence = Vec::new();
            let mut serial = 1;
            let mut residues = 0;
            for (line, (ser_num, res_num, seq)) in data.into_iter().enumerate() {
                if serial != ser_num {
                    errors.push(BoxedError::new(
                        ErrorLevel::StrictWarning,
                        "SEQRES serial number invalid",
                        format!("The serial number for SEQRES chain \"{chain_id}\" with number \"{ser_num}\" does not follow sequentially from the previous row."),
                        context.clone()
                    ));
                }
                serial += 1;
                if residues == 0 {
                    residues = res_num;
                } else if residues != res_num {
                    errors.push(BoxedError::new(
                        ErrorLevel::StrictWarning,
                        "SEQRES residue total invalid",
                        format!("The residue total for SEQRES chain \"{chain_id}\" with number \"{ser_num}\" does not match the total on the first row for this chain."),
                        context.clone()
                    ));
                }
                chain_sequence.extend(
                    seq.into_iter()
                        .enumerate()
                        .map(|(column, item)| (item, (line, column))),
                );
            }
            if chain_sequence.len() != residues {
                errors.push(BoxedError::new(
                    ErrorLevel::LooseWarning,
                    "SEQRES residue total invalid",
                    format!("The residue total for SEQRES chain \"{chain_id}\" does not match the total residues found in the seqres records."),
                    context.clone()
                ));
            }
            let mut offset = 0;
            if let Some(db_ref) = chain.database_reference() {
                offset = db_ref.pdb_position.start;
                for dif in &db_ref.differences {
                    if dif.database_residue.is_none() && dif.residue.1 < db_ref.pdb_position.start {
                        // If there is a residue in front of the db sequence
                        offset -= 1;
                    }
                }
                if db_ref.pdb_position.end - offset + 1 != residues as isize {
                    errors.push(BoxedError::new(
                        ErrorLevel::LooseWarning,
                        "SEQRES residue total invalid",
                        format!("The residue total ({}) for SEQRES chain \"{}\" does not match the total residues found in the dbref record ({}).", residues, chain_id, db_ref.pdb_position.end - offset + 1),
                        context.clone()
                    ));
                }
            }

            let copy = chain.clone();
            let mut chain_res = copy.residues();
            let mut next = chain_res.next();
            // Contain all inconsistencies between the SEQRES and found residues, group all by `chain_id` and keep a list of index, seqres item position, and chain_sequence.
            let mut seqres_inconsistent = Vec::new();

            for (raw_index, (seq, position)) in chain_sequence.iter().enumerate() {
                let index = raw_index as isize + offset;
                if let Some(n) = next {
                    if index == n.serial_number() {
                        if let Some(name) = n.name() {
                            if *seq != name {
                                #[allow(clippy::type_complexity)]
                                if let Some(item) = seqres_inconsistent.iter_mut().find(
                                    |item: &&mut (&str, Vec<(isize, (usize, usize), &str)>)| {
                                        item.0 == chain_id
                                    },
                                ) {
                                    item.1.push((index, *position, name));
                                } else {
                                    seqres_inconsistent
                                        .push((&chain_id, vec![(index, *position, name)]));
                                }
                            }
                        } else {
                            errors.push(BoxedError::new(
                                ErrorLevel::StrictWarning,
                                "Multiple residues in SEQRES validation",
                                format!("The residue index {index} in chain {chain_id} has no conformers or multiple with different names. The program cannot validate the SEQRES record in this way."),
                                context.clone()
                            )); // TODO: show found residues
                        }
                        next = chain_res.next();
                    } else if index < n.serial_number() {
                        chain.add_residue(
                            Residue::new(
                                index,
                                None,
                                Some(
                                    Conformer::new(seq, None, None)
                                        .expect("Invalid characters in Conformer generation"),
                                ),
                            )
                            .expect("Invalid characters in Residue generation"),
                        );
                        chain.sort();
                    } else {
                        errors.push(BoxedError::new(
                            ErrorLevel::LooseWarning,
                            "Chain residue invalid",
                            format!("The residue index {} value \"{:?}\" for Chain \"{}\" is not sequentially increasing, value expected: {}.", n.serial_number(), n.name(), chain_id, index),
                            context.clone()
                        ));
                        #[allow(clippy::while_let_on_iterator)]
                        while let Some(n) = chain_res.next() {
                            if n.serial_number() == index {
                                next = chain_res.next();
                                break;
                            }
                        }
                    }
                } else {
                    chain.add_residue(
                        Residue::new(
                            index,
                            None,
                            Some(
                                Conformer::new(seq, None, None)
                                    .expect("Invalid characters in Conformer generation"),
                            ),
                        )
                        .expect("Invalid characters in Residue generation"),
                    );
                    chain.sort();
                }
            }

            if !seqres_inconsistent.is_empty() {
                for (_chain, inconsistencies) in seqres_inconsistent {
                    // Find the extremes of the used lines from all SEQRES lines, assuming that all lines are continuous.
                    let used_lines = inconsistencies
                        .iter()
                        .fold((usize::MAX, usize::MIN), |a, v| {
                            (v.1 .0.min(a.0), v.1 .0.max(a.1))
                        });
                    let context_lines = &lines[used_lines.0..=used_lines.1];
                    let mut highlights = Vec::new();
                    let mut found_residues = Vec::new();

                    // Create all highlights in the original SEQRES definition.
                    for inconsistency in inconsistencies {
                        let line_offset = inconsistency.1 .0 - used_lines.0;
                        let offset = 19 + 4 * inconsistency.1 .1;
                        highlights.push((line_offset, offset, 3)); // Calculate the correct character position for the highlight (is index right now)

                        // Add the found residue to the list of found residues, in the correct line
                        if let Some(line) = found_residues
                            .iter_mut()
                            .find(|v: &&mut (usize, Vec<(usize, &str)>)| v.0 == line_offset)
                        {
                            line.1.push((inconsistency.1 .1, inconsistency.2));
                        } else {
                            found_residues
                                .push((line_offset, vec![(inconsistency.1 .1, inconsistency.2)]));
                        }
                    }
                    // Add all found residues into lines matching the SEQRES definition
                    let found_residues = found_residues
                        .iter()
                        .map(|line| {
                            line.1
                                .iter()
                                .enumerate()
                                .fold((String::new(), 0), |acc, v| {
                                    (
                                        acc.0
                                            + if v.1 .0.saturating_sub(1) == acc.1 || v.0 == 0 {
                                                " "
                                            } else {
                                                " ... "
                                            }
                                            + v.1 .1,
                                        v.1 .0,
                                    )
                                })
                                .0
                        })
                        .fold(String::new(), |mut acc, item| {
                            if acc.is_empty() {
                                item
                            } else {
                                write!(&mut acc, "\n{item}").unwrap();
                                acc
                            }
                        });

                    // Create the final error message. See example:
                    // LooseWarning: SEQRES inconsistent residues
                    //
                    //      |
                    // 1    | SEQRES 1   B 475   GLY PRO ASN ILE CYS THR THR ARG GLY VAL SER SER CYS (SEQRES definition)
                    //      |                                ^^^ ^^^ ^^^ ^^^
                    //      |
                    //
                    //      |
                    // 1    |                     HOH HOH HOH HOH (found residues)
                    //      |
                    // The residues as defined in the SEQRES records do not match with the found residues, see above for details.
                    errors.push(BoxedError::new(
                        ErrorLevel::LooseWarning,
                        "SEQRES inconsistent residues",
                        "The residues as defined in the SEQRES records do not match with the found residues, see above for details.",
                            Context::default().line_index(start_linenumber as u32).lines(0, context_lines.join("\n")).add_highlights(highlights)
                        ).add_context( Context::default().lines(0,found_residues))
                    );
                }
            }

            let total_found = chain
                .residues()
                .filter(|r| !r.atoms().any(Atom::hetero)) // TODO: It filters out all residues with at least one HETATM, this should be changed to include in the total residues defined in HETNAM.
                .count();
            if chain_sequence.len() != total_found {
                errors.push(BoxedError::new(
                    ErrorLevel::LooseWarning,
                    "SEQRES residue total invalid",
                    format!("The residue total ({}) for SEQRES chain \"{}\" does not match the total residues found in the chain ({}).", chain_sequence.len(), chain_id, total_found),
                    context.clone()
                ));
            }
        }
    }
    errors
}
