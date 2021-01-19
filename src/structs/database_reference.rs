#![allow(dead_code)]

#[derive(Debug)]
/// The position of the sequence for a cross-reference of sequences.
pub struct SequencePosition {
    /// The starting position
    pub start: usize,
    /// Initial insertion code of the PDB sequence segment
    pub start_insert: char,
    /// The ending position
    pub end: usize,
    /// Ending insertion code of the PDB sequence segment
    pub end_insert: char,
}

impl SequencePosition {
    /// Create a new SequencePosition
    pub fn new(start: usize, start_insert: char, end: usize, end_insert: char) -> Self {
        SequencePosition {
            start,
            start_insert,
            end,
            end_insert,
        }
    }

    /// Create a new SequencePosition, from a tuple
    pub fn from_tuple((start, start_insert, end, end_insert): (usize, char, usize, char)) -> Self {
        SequencePosition {
            start,
            start_insert,
            end,
            end_insert,
        }
    }
}

#[derive(Debug)]
/// A DatabaseReference containing the cross-reference to a corresponding database sequence for a Chain.
pub struct DatabaseReference {
    /// The information about the database, (name, accession code, identification code), see DBREF documentation wwPDB v3.30
    pub database: (String, String, String),
    /// The position of the sequence as present in the PDB
    pub pdb_position: SequencePosition,
    /// The position of the sequence as present in the database
    pub database_position: SequencePosition,
    /// The differences between residues in the database and in the pdb file
    pub differences: Vec<SequenceDifference>,
}

impl DatabaseReference {
    /// Create a new DatabaseReference
    pub fn new(
        database: (String, String, String),
        pdb_position: SequencePosition,
        database_position: SequencePosition,
    ) -> Self {
        DatabaseReference {
            database,
            pdb_position,
            database_position,
            differences: Vec::new(),
        }
    }
}

#[derive(Debug)]
/// A difference between the sequence of the database and the pdb file
pub struct SequenceDifference {
    /// The residue in the PDB file
    pub residue: ([char; 3], usize),
    /// The residue in the database
    pub database_residue: Option<([char; 3], usize)>,
    /// The comment to explain the difference
    pub comment: String,
}

impl SequenceDifference {
    /// Create a new DatabaseReference
    pub fn new(
        residue: ([char; 3], usize),
        database_residue: Option<([char; 3], usize)>,
        comment: String,
    ) -> Self {
        SequenceDifference {
            residue,
            database_residue,
            comment,
        }
    }
}