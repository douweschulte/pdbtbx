#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// The position of the sequence for a cross-reference of sequences.
pub struct SequencePosition {
    /// The starting position
    pub start: isize,
    /// Initial insertion code of the PDB sequence segment
    pub start_insert: char,
    /// The ending position
    pub end: isize,
    /// Ending insertion code of the PDB sequence segment
    pub end_insert: char,
}

impl SequencePosition {
    /// Create a new SequencePosition
    pub fn new(start: isize, start_insert: char, end: isize, end_insert: char) -> Self {
        SequencePosition {
            start,
            start_insert,
            end,
            end_insert,
        }
    }

    /// Create a new SequencePosition, from a tuple
    pub fn from_tuple((start, start_insert, end, end_insert): (isize, char, isize, char)) -> Self {
        SequencePosition {
            start,
            start_insert,
            end,
            end_insert,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// A difference between the sequence of the database and the pdb file
pub struct SequenceDifference {
    /// The residue in the PDB file
    pub residue: (String, isize, Option<String>),
    /// The residue in the database
    pub database_residue: Option<(String, isize)>,
    /// The comment to explain the difference
    pub comment: String,
}

impl SequenceDifference {
    /// Create a new DatabaseReference
    pub fn new(
        residue: (String, isize, Option<String>),
        database_residue: Option<(String, isize)>,
        comment: String,
    ) -> Self {
        SequenceDifference {
            residue,
            database_residue,
            comment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_database_reference() {
        let pos_db = SequencePosition::new(10, ' ', 12, ' ');
        let pos_seq = SequencePosition::new(10, ' ', 13, 'A');
        let a = DatabaseReference::new(
            ("DB".to_string(), "ACC".to_string(), "ID".to_string()),
            pos_seq.clone(),
            pos_db.clone(),
        );
        let c = DatabaseReference::new(
            ("Z".to_string(), "ACC".to_string(), "ID".to_string()),
            pos_seq.clone(),
            pos_db.clone(),
        );
        assert_ne!(a, c);
        assert_eq!(a.database_position, pos_db);
        assert_eq!(a.pdb_position, pos_seq);
        assert_eq!(a.differences, Vec::new());
        format!("{:?}", a);
        assert!(a < c);
    }

    #[test]
    fn check_sequence_position() {
        let a = SequencePosition::new(10, ' ', 12, ' ');
        let b = SequencePosition::from_tuple((10, ' ', 12, ' '));
        let c = SequencePosition::from_tuple((11, ' ', 12, ' '));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.start, 10);
        assert_eq!(a.start_insert, ' ');
        assert_eq!(a.end, 12);
        assert_eq!(a.end_insert, ' ');
        format!("{:?}", a);
        assert!(a < c);
    }
    #[test]
    fn check_sequence_difference() {
        let a = SequenceDifference::new(
            ("ALA".to_string(), 10, None),
            Some(("PHE".to_string(), 10)),
            "Added phenyl group".to_string(),
        );
        let b = SequenceDifference::new(
            ("ALA".to_string(), 10, None),
            Some(("PHE".to_string(), 13)),
            "Added phenyl group".to_string(),
        );
        assert_ne!(a, b);
        assert!(a < b);
        format!("{:?}", a);
    }
}
