use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use indexmap::IndexMap;

use crate::error::*;
use crate::structs::*;
use crate::validate::*;
use crate::ReadOptions;
use crate::StrictnessLevel;

use super::lexer::*;
use super::lexitem::*;
use super::temporary_structs::*;
use super::validate::*;

/// Parse the given file into a PDB struct.
/// Returns a PDBError if a BreakingError is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory see [`ReadOptions::read_raw`]. There is also a function to open a mmCIF file directly
/// see [`crate::open_mmcif`]. If you want to open a general file with no knowledge about the file type see [`crate::open`].
#[deprecated(
    since = "0.12.0",
    note = "Please use `ReadOptions::default().set_format(Format::Pdb).read(filename)` instead"
)]
pub fn open_pdb(
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    open_pdb_with_options(filename, ReadOptions::default().set_level(level))
}

/// Parse the given file into a PDB struct with [`ReadOptions`].
pub(crate) fn open_pdb_with_options(
    filename: impl AsRef<str>,
    options: &ReadOptions,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    let filename = filename.as_ref();
    // Open a file a use a buffered reader to minimise memory use while immediately lexing the line followed by adding it to the current PDB
    let file = if let Ok(f) = File::open(filename) {
        f
    } else {
        return Err(vec![PDBError::new(ErrorLevel::BreakingError, "Could not open file", "Could not open the specified file, make sure the path is correct, you have permission, and that it is not open in another program.", Context::show(filename))]);
    };
    let reader = BufReader::new(file);
    open_pdb_raw_with_options(reader, Context::show(filename), options)
}

/// Parse the input stream into a PDB struct. To allow for direct streaming from sources, like from RCSB.org.
/// Returns a PDBError if a BreakingError is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// ## Arguments
/// * `input` - the input stream
/// * `context` - the context of the full stream, to place error messages correctly, for files this is `Context::show(filename)`.
/// * `level` - the strictness level to operate in. If errors are generated which are breaking in the given level the parsing will fail.
///
/// # Related
/// If you want to open a file see [`open_pdb`]. There is also a function to open a mmCIF file directly
/// see [`crate::open_mmcif`] and [`crate::open_mmcif_raw`]. If you want to open a general file
/// with no knowledge about the file type see [`crate::open`] and [`crate::open_raw`].
#[deprecated(
    since = "0.12.0",
    note = "Please use `ReadOptions::default().read_raw(input)` instead"
)]
pub fn open_pdb_raw<T>(
    input: std::io::BufReader<T>,
    context: Context,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>>
where
    T: std::io::Read,
{
    open_pdb_raw_with_options(input, context, ReadOptions::default().set_level(level))
}

/// Parse the input stream into a [`PDB`] struct.
///
/// # Related
/// See [`ReadOptions::read_raw`] for a version of this function with sane defaults.
/// Note that the file type should be set explicitly with [`ReadOptions::set_format`].
pub(crate) fn open_pdb_raw_with_options<T>(
    input: std::io::BufReader<T>,
    context: Context,
    options: &ReadOptions,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>>
where
    T: std::io::Read,
{
    let mut errors = Vec::new();
    let mut pdb = PDB::new();
    let mut current_model_number = 0;
    let mut current_model: IndexMap<String, IndexMap<(isize, Option<String>), Residue>> =
        IndexMap::new();
    let mut sequence: HashMap<String, Vec<(usize, usize, Vec<String>)>> = HashMap::new();
    let mut seqres_lines = Vec::new();
    let mut seqres_start_linenumber = usize::MAX;
    let mut database_references = Vec::new();
    let mut modifications = Vec::new();
    let mut bonds = Vec::new();
    let mut temp_scale = BuildUpMatrix::empty();
    let mut temp_origx = BuildUpMatrix::empty();
    let mut temp_mtrix: Vec<(usize, BuildUpMatrix, bool)> = Vec::new();
    let mut last_residue_serial_number = 0;
    let mut residue_serial_addition = 0;
    let mut last_atom_serial_number = 0;
    let mut atom_serial_addition = 0;
    let mut chain_iter = ('A'..='Z').cycle();
    // Initialize chain_id value
    let mut chain_id_new = chain_iter.next();

    'all_lines: for (mut linenumber, read_line) in input.lines().enumerate() {
        linenumber += 1; // 1 based indexing in files

        let line = if let Ok(l) = read_line {
            l
        } else {
            return Err(vec![PDBError::new(
                ErrorLevel::BreakingError,
                "Could read line",
                format!("Could not read line {linenumber} while parsing the input file."),
                context,
            )]);
        };
        let line_result = lex_line(&line, linenumber, options);
        let line_context = Context::FullLine {
            linenumber,
            line: line.clone(),
        };

        // Then immediately add this lines information to the final PDB struct
        match line_result {
            Ok((result, line_errors)) => {
                errors.extend(line_errors);
                match result {
                    LexItem::Header(_, _, identifier) => pdb.identifier = Some(identifier),
                    LexItem::Remark(num, text) => {
                        let _ = pdb.add_remark(num, text.to_string()); // Better error messages are created downstream
                    }
                    LexItem::Atom(
                        hetero,
                        serial_number,
                        name,
                        alt_loc,
                        residue_name,
                        mut chain_id,
                        residue_serial_number,
                        insertion_code,
                        x,
                        y,
                        z,
                        occ,
                        b,
                        _,
                        element,
                        charge,
                        autodock_type,
                    ) => {
                        if options.discard_hydrogens & (element == "H") {
                            continue;
                        }
                        if serial_number == 0 && last_atom_serial_number == 99_999 {
                            atom_serial_addition += 100_000
                        }

                        if residue_serial_number == 0 && last_residue_serial_number == 9999 {
                            residue_serial_addition += 10000;
                        }

                        if chain_id.trim().is_empty() {
                            chain_id = chain_id_new
                                .expect("Chain ID iterator is exhausted")
                                .to_string();
                        }

                        let atom = Atom::new(
                            hetero,
                            serial_number + atom_serial_addition,
                            name,
                            x,
                            y,
                            z,
                            occ,
                            b,
                            element,
                            charge,
                            autodock_type,
                        )
                        .expect("Invalid characters in atom creation");
                        let conformer_id = (residue_name.as_str(), alt_loc.as_deref());

                        let current_chain = if let Some(chain) = current_model.get_mut(&chain_id) {
                            chain
                        } else {
                            current_model.insert(chain_id.clone(), IndexMap::new());
                            current_model.get_mut(&chain_id).expect("Element that was just inserted into this IndexMap was not found in this IndexMap.")
                        };

                        if let Some(residue) = current_chain.get_mut(&(
                            residue_serial_number + residue_serial_addition,
                            insertion_code.clone(),
                        )) {
                            residue.add_atom(atom, conformer_id);
                        } else {
                            current_chain.insert(
                                (
                                    residue_serial_number + residue_serial_addition,
                                    insertion_code.clone(),
                                ),
                                Residue::new(
                                    residue_serial_number + residue_serial_addition,
                                    insertion_code.as_deref(),
                                    Some(
                                        Conformer::new(
                                            residue_name.as_str(),
                                            alt_loc.as_deref(),
                                            Some(atom),
                                        )
                                        .expect("Invalid characters in Conformer creation"),
                                    ),
                                )
                                .expect("Invalid characters in Residue creation"),
                            );
                        }

                        last_residue_serial_number = residue_serial_number;
                        last_atom_serial_number = serial_number;
                    }
                    LexItem::Anisou(s, n, _, _r, _c, _rs, _, factors, _, _e, _ch) => {
                        let mut found = false;
                        for atom in current_model
                            .values_mut()
                            .rev()
                            .flat_map(|residues| residues.values_mut().flat_map(Residue::atoms_mut))
                        {
                            if atom.serial_number() == s {
                                atom.set_anisotropic_temperature_factors(factors);
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            eprintln!("Could not find atom for temperature factors, coupled to atom {s} {n}")
                        }
                    }
                    LexItem::Model(number) => {
                        if !current_model.is_empty() {
                            pdb.add_model(Model::from_iter(
                                current_model_number,
                                current_model.into_iter().map(|(id, residues)| {
                                    Chain::from_iter(id, residues.into_values())
                                        .expect("Invalid characters in Chain definition")
                                }),
                            ));

                            if options.only_first_model {
                                current_model = IndexMap::new();
                                break 'all_lines;
                            }
                        }
                        current_model_number = number;
                        current_model = IndexMap::new();
                    }
                    LexItem::Scale(n, row) => {
                        temp_scale.set_row(n, row);
                    }
                    LexItem::OrigX(n, row) => {
                        temp_origx.set_row(n, row);
                    }
                    LexItem::MtriX(n, ser, row, given) => {
                        let mut found = false;
                        for (index, matrix, contained) in &mut temp_mtrix {
                            if *index == ser {
                                matrix.set_row(n, row);
                                *contained = given;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            let mut matrix = BuildUpMatrix::empty();
                            matrix.set_row(n, row);
                            temp_mtrix.push((ser, matrix, given))
                        }
                    }
                    LexItem::Crystal(a, b, c, alpha, beta, gamma, spacegroup, _z) => {
                        pdb.unit_cell = Some(UnitCell::new(a, b, c, alpha, beta, gamma));
                        pdb.symmetry =
                            Some(Symmetry::new(&spacegroup).unwrap_or_else(|| {
                                panic!("Invalid space group: \"{spacegroup}\"")
                            }));
                    }
                    LexItem::Seqres(ser_num, chain_id, num_res, values) => {
                        seqres_start_linenumber = seqres_start_linenumber.min(linenumber);
                        if let Some(data) = sequence.get_mut(&chain_id) {
                            data.push((ser_num, num_res, values));
                        } else {
                            sequence.insert(chain_id, vec![(ser_num, num_res, values)]);
                        }
                        seqres_lines.push(line);
                    }
                    LexItem::Dbref(_pdb_id, chain_id, local_pos, db, db_acc, db_id, db_pos) => {
                        database_references.push((
                            chain_id,
                            DatabaseReference::new(
                                (db, db_acc, db_id),
                                SequencePosition::from_tuple(local_pos),
                                SequencePosition::from_tuple(db_pos),
                            ),
                            true,
                        ));
                    }
                    LexItem::Dbref1(_pdb_id, chain_id, local_pos, db, db_id) => {
                        database_references.push((
                            chain_id,
                            DatabaseReference::new(
                                (db, "".to_string(), db_id),
                                SequencePosition::from_tuple(local_pos),
                                SequencePosition::new(0, ' ', 0, ' '),
                            ),
                            false,
                        ));
                    }
                    LexItem::Dbref2(_pdb_id, chain_id, db_acc, db_start, db_end) => {
                        let mut found = false;
                        for dbref in database_references.iter_mut() {
                            if dbref.0 == chain_id {
                                dbref.1.database.acc = db_acc;
                                dbref.1.database_position =
                                    SequencePosition::new(db_start, ' ', db_end, ' ');
                                dbref.2 = true;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            errors.push(PDBError::new(ErrorLevel::BreakingError, "Solitary DBREF2", format!("Could not find the DBREF1 record fitting to this DBREF2 with chain id '{chain_id}'"), line_context.clone()))
                        }
                    }
                    LexItem::Seqadv(
                        _id_code,
                        chain_id,
                        res_name,
                        seq_num,
                        insert,
                        _database,
                        _database_accession,
                        db_pos,
                        comment,
                    ) => {
                        if let Some((_, db_ref, _)) =
                            database_references.iter_mut().find(|a| a.0 == chain_id)
                        {
                            db_ref.differences.push(SequenceDifference::new(
                                (res_name, seq_num, insert),
                                db_pos,
                                comment,
                            ))
                        } else {
                            errors.push(PDBError::new(
                            ErrorLevel::StrictWarning,
                            "Sequence Difference Database not found",
                            format!("For this sequence difference (chain: {chain_id}) the corresponding database definition (DBREF) was not found, make sure the DBREF is located before the SEQADV"),
                            line_context.clone()
                        ))
                        }
                    }
                    item @ LexItem::Modres(..) => modifications.push((line_context.clone(), item)),
                    item @ LexItem::SSBond(..) => bonds.push((line_context.clone(), item)),
                    LexItem::Master(
                        num_remark,
                        num_empty,
                        _num_het,
                        _num_helix,
                        _num_sheet,
                        _num_turn,
                        _num_site,
                        num_xform,
                        num_coord,
                        _num_ter,
                        _num_connect,
                        _num_seq,
                    ) => {
                        // The last atoms need to be added to make the MASTER checksum work out
                        if !current_model.is_empty() {
                            pdb.add_model(Model::from_iter(
                                current_model_number,
                                current_model.into_iter().map(|(id, residues)| {
                                    Chain::from_iter(id, residues.into_values())
                                        .expect("Invalid characters in Chain definition")
                                }),
                            ));
                            current_model = IndexMap::new();
                        }
                        // The for now forgotten numbers will have to be added when the appropriate records are added to the parser
                        if num_remark != pdb.remark_count() {
                            errors.push(
                            PDBError::new(
                                ErrorLevel::StrictWarning,
                                "MASTER checksum failed",
                                format!("The number of REMARKS ({}) is different then posed in the MASTER Record ({})", pdb.remark_count(), num_remark),
                                line_context.clone()
                            )
                        );
                        }
                        if num_empty != 0 {
                            errors.push(
                            PDBError::new(
                                ErrorLevel::LooseWarning,
                                "MASTER checksum failed",
                                format!("The empty checksum number is not empty (value: {num_empty}) while it is defined to be empty."),
                                line_context.clone()
                            )
                        );
                        }
                        let mut xform = 0;
                        if temp_origx.is_set() {
                            xform += 3;
                        }
                        if temp_scale.is_set() {
                            xform += 3;
                        }
                        for (_, mtrix, _) in &temp_mtrix {
                            if mtrix.is_set() {
                                xform += 3;
                            }
                        }
                        if num_xform != xform {
                            errors.push(
                            PDBError::new(
                                ErrorLevel::StrictWarning,
                                "MASTER checksum failed",
                                format!("The number of coordinate transformation records ({xform}) is different then posed in the MASTER Record ({num_xform})"),
                                line_context.clone()
                            )
                        );
                        }
                        if num_coord != pdb.total_atom_count() {
                            errors.push(
                            PDBError::new(
                                ErrorLevel::LooseWarning,
                                "MASTER checksum failed",
                                format!("The number of Atoms ({}) is different then posed in the MASTER Record ({})", pdb.total_atom_count(), num_coord),
                                line_context.clone()
                            )
                        );
                        }
                    }
                    LexItem::TER() => chain_id_new = chain_iter.next(),
                    _ => (),
                }
            }
            Err(e) => errors.push(e),
        }
    }
    if !current_model.is_empty() {
        pdb.add_model(Model::from_iter(
            current_model_number,
            current_model.into_iter().map(|(id, residues)| {
                Chain::from_iter(id, residues.into_values())
                    .expect("Invalid characters in Chain definition")
            }),
        ));
    }

    for (chain_id, reference, complete) in database_references {
        if !complete {
            errors.push(PDBError::new(
                ErrorLevel::StrictWarning,
                "Solitary DBREF1 definition",
                format!("The complementary DBREF2 was not found for this DBREF1 definition. For chain id '{}'. For database '{}' with ID code '{}'.", chain_id, reference.database.name, reference.database.id),
                Context::None,
            ))
        } else if let Some(chain) = pdb.chains_mut().find(|a| a.id() == chain_id) {
            chain.set_database_reference(reference);
        }
    }

    if let Some(scale) = temp_scale.get_matrix() {
        pdb.scale = Some(scale);
    } else if temp_scale.is_partly_set() {
        errors.push(PDBError::new(
            ErrorLevel::StrictWarning,
            "Invalid SCALE definition",
            "Not all rows are set in the scale definition",
            context.clone(),
        ))
    }

    if let Some(origx) = temp_origx.get_matrix() {
        pdb.origx = Some(origx);
    } else if temp_origx.is_partly_set() {
        errors.push(PDBError::new(
            ErrorLevel::StrictWarning,
            "Invalid ORIGX definition",
            "Not all rows are set in the ORIGX definition",
            context.clone(),
        ))
    }

    for (index, matrix, given) in temp_mtrix {
        if let Some(m) = matrix.get_matrix() {
            pdb.add_mtrix(MtriX::new(index, m, given))
        } else {
            errors.push(PDBError::new(
                ErrorLevel::StrictWarning,
                "Invalid MATRIX definition",
                format!("Not all rows are set in the MtriX definition, number: {index}",),
                context.clone(),
            ))
        }
    }

    reshuffle_conformers(&mut pdb);

    merge_long_remark_warnings(&mut errors);
    errors.extend(validate_seqres(
        &mut pdb,
        sequence,
        seqres_lines,
        seqres_start_linenumber - 1, // Convert from 1 based to 0 based numbering
        &context,
    ));
    errors.extend(add_modifications(&mut pdb, modifications));
    errors.extend(add_bonds(&mut pdb, bonds));
    errors.extend(validate(&pdb));

    if errors.iter().any(|e| e.fails(options.level)) {
        Err(errors)
    } else {
        Ok((pdb, errors))
    }
}

/// Merge all warnings about long REMARK definitions into a single warning
fn merge_long_remark_warnings(errors: &mut Vec<PDBError>) {
    // Weed out all remark too long warnings
    let mut remark_too_long = Vec::new();
    errors.retain(|error| {
        if error.short_description() == "Remark too long" {
            remark_too_long.push(error.context().clone());
            false
        } else {
            true
        }
    });
    // Merge consecutive warnings into a single context to take up less vertical space
    let mut contexts = Vec::new();
    let mut lines = Vec::new();
    let mut highlights = Vec::new();
    let mut last = usize::MAX;
    let mut index = 0;
    for context in remark_too_long {
        if let Context::Line {
            linenumber,
            line,
            offset,
            length,
        } = context
        {
            if last == usize::MAX || linenumber - 1 == last {
                lines.push(line);
                highlights.push((index, offset, length));
                last = linenumber;
                index += 1;
            } else {
                if !lines.is_empty() {
                    contexts.push((
                        None,
                        Context::RangeHighlights {
                            start_linenumber: last - index,
                            lines,
                            highlights,
                        },
                    ));
                    index = 0;
                }
                lines = vec![line];
                highlights = vec![(index, offset, length)];
                last = linenumber;
            }
        }
    }
    if !lines.is_empty() {
        contexts.push((
            None,
            Context::RangeHighlights {
                start_linenumber: last - index,
                lines,
                highlights,
            },
        ));
    }
    if !contexts.is_empty() {
        // Generate the final error message
        errors.push(PDBError::new(
            ErrorLevel::GeneralWarning,
            "Remark too long",
            "The above REMARK definitions are too long, the max is 80 characters.",
            Context::Multiple { contexts },
        ));
    }
}

/// Adds all MODRES records to the Atoms
fn add_modifications(pdb: &mut PDB, modifications: Vec<(Context, LexItem)>) -> Vec<PDBError> {
    let mut errors = Vec::new();
    for (context, item) in modifications {
        match item {
            LexItem::Modres(_, res_name, chain_id, seq_num, insertion_code, std_name, comment) => {
                if let Some(chain) = pdb.chains_mut().find(|c| c.id() == chain_id) {
                    if let Some(residue) = chain
                        .residues_mut()
                        .find(|r| r.id() == (seq_num, insertion_code.as_deref()))
                    {
                        if let Some(conformer) =
                            residue.conformers_mut().find(|c| c.name() == res_name)
                        {
                            if let Err(e) = conformer.set_modification((std_name, comment)) {
                                errors.push(PDBError::new(
                                    ErrorLevel::InvalidatingError,
                                    "Invalid characters",
                                    e,
                                    context,
                                ));
                            }
                        } else {
                            errors.push(PDBError::new(ErrorLevel::InvalidatingError, "Modified residue could not be found", "The residue presented in this MODRES record could not be found in the specified residue in the PDB file.", context));
                        }
                    } else {
                        errors.push(PDBError::new(ErrorLevel::InvalidatingError, "Modified residue could not be found", "The residue presented in this MODRES record could not be found in the specified chain in the PDB file.", context));
                    }
                } else {
                    errors.push(PDBError::new(ErrorLevel::InvalidatingError, "Modified residue could not be found", "The chain presented in this MODRES record could not be found in the PDB file.", context));
                }
            }
            _ => {
                panic!("Found an invalid element in the modifications list, it is not a LexItem::Modres");
            }
        }
    }
    errors
}

/// Adds all bonds to the PDB, has to be done after all Atoms are already in place
#[allow(clippy::unwrap_used)]
fn add_bonds(pdb: &mut PDB, bonds: Vec<(Context, LexItem)>) -> Vec<PDBError> {
    let mut errors = Vec::new();
    for (context, bond) in bonds {
        match bond {
            LexItem::SSBond(atom1, atom2, ..) => {
                let find = |atom: (String, isize, Option<String>, String)| {
                    pdb.chains()
                        .find(|c| c.id() == atom.3)
                        .and_then(|c| {
                            c.residues()
                                .find(|r| {
                                    r.serial_number() == atom.1
                                        && r.insertion_code() == atom.2.as_deref()
                                })
                                .map(|r| {
                                    r.conformers().find(|c| c.name() == atom.0).map(|c| {
                                        c.atoms().find(|a| a.name() == "SG").map(Atom::counter)
                                    })
                                })
                        })
                        .flatten()
                        .flatten()
                };
                let ref1 = find(atom1);
                let ref2 = find(atom2);

                if let (Some(counter1), Some(counter2)) = (ref1, ref2) {
                    pdb.add_bond_counters(counter1, counter2, Bond::Disulfide);
                } else {
                    errors.push(PDBError::new(
                        ErrorLevel::InvalidatingError,
                        "Could not find a bond partner",
                        "One of the atoms could not be found while parsing a disulfide bond.",
                        context,
                    ));
                }
            }
            _ => {
                panic!(
                    "Found an invalid element in the bonds list, it is not a valid bond LexItem"
                );
            }
        }
    }
    errors
}
