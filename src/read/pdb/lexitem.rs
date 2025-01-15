/// A definition of all lines that a PDB file can contain (and can be parsed by this program)
/// with all properties saved as primitive data types.
///
/// See wwPDB v3.30 for detailed explanation of the meaning of all fields
#[derive(Debug, Clone)]
pub enum LexItem {
    /// A HEADER in a PDB file
    /// * classification
    /// * date of deposition
    /// * identification
    Header(String, String, String),
    /// A REMARK saved as the remark-type-number and the remark line itself
    Remark(usize, String),
    /// An Atom with all its information, including the deprecated and rarely used fields.
    /// * hetatom (true) or atom (false)
    /// * serial number
    /// * name
    /// * alternate location
    /// * residue name
    /// * chain id
    /// * residue serial number
    /// * insertion
    /// * x
    /// * y
    /// * z
    /// * occupancy
    /// * b_factor
    /// * segment id
    /// * element
    /// * charge
    /// * autodock type
    Atom(
        bool,
        usize,
        String,
        Option<String>,
        String,
        String,
        isize,
        Option<String>,
        f64,
        f64,
        f64,
        f64,
        f64,
        String,
        String,
        f32,
        String,
    ),
    /// An Anisou record with all its information, including the deprecated and rarely used fields.
    /// * serial number
    /// * name
    /// * alternate location
    /// * residue name
    /// * chain id
    /// * residue serial number
    /// * insertion
    /// * temperature factors
    /// * segment id
    /// * element
    /// * charge
    Anisou(
        usize,
        String,
        Option<String>,
        String,
        String,
        isize,
        Option<String>,
        [[f64; 3]; 3],
        String,
        String,
        isize,
    ),
    /// A SCALEn line, as the row (1/2/3) and data
    Scale(usize, [f64; 4]),
    /// A ORIGXn line, as the row (1/2/3) and data
    OrigX(usize, [f64; 4]),
    /// A MTRIXn line, as the row (1/2/3), serial number, data, and contained fields
    MtriX(usize, usize, [f64; 4], bool),
    /// A CRYST1 line, containing: a, b, c, alpha, beta, gamma, space group character, and space group symbols as numbers
    Crystal(f64, f64, f64, f64, f64, f64, String, usize),
    /// A MODEL with its serial number
    Model(usize),
    /// The Master record, having a checksum of the number of selected record types, used for verification
    Master(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ),
    /// A SEQRES row
    ///  * SerNum - Serial number of the SEQRES record for the current chain
    ///  * ChainID - The Chain, will be a single character, can be blank (it then selects the only chain available)
    ///  * NumRes - The number of residues in the chain (repeated every row)
    ///  * ResidueNames - All residues in the chain
    Seqres(usize, String, usize, Vec<String>),
    /// A DBREF row in the original/standard format
    ///  * IDCode
    ///  * ChainID
    ///  * (SeqBegin, InsertBegin, SeqEnd, InsertEnd)
    ///  * Database - Sequence database name
    ///  * DBAccession - Sequence database accession code
    ///  * DBIDCode - Sequence database identification code
    ///  * (DBSeqBegin, DBInsertBegin, DBSeqEnd, DBInsertEnd)
    Dbref(
        String,
        String,
        (isize, char, isize, char),
        String,
        String,
        String,
        (isize, char, isize, char),
    ),
    /// A DBREF1 row in the DBREF extended format
    ///  * IDCode
    ///  * ChainID
    ///  * (SeqBegin, InsertBegin, SeqEnd, InsertEnd)
    ///  * Database - Sequence database name
    ///  * DBIDCode - Sequence database identification code
    Dbref1(String, String, (isize, char, isize, char), String, String),
    /// A DBREF2 row in the DBREF extended format
    ///  * IDCode
    ///  * ChainID
    ///  * DBAccession - Sequence database accession code
    ///  * DBSeqBegin - Initial sequence number of the Database segment
    ///  * DBSeqEnd -  Ending sequence number of the Database segment
    Dbref2(String, String, String, isize, isize),
    /// A SEQADV row
    ///  * IDCode
    ///  * ResName - Name of the PDB residue in conflict
    ///  * ChainID
    ///  * SeqNum
    ///  * InsertionCode
    ///  * Database
    ///  * DBAccession
    ///  * (DBRes, DBSeq) - Sequence database residue name and sequence number
    ///  * Conflict comment
    Seqadv(
        String,
        String,
        String,
        isize,
        Option<String>,
        String,
        String,
        Option<(String, isize)>,
        String,
    ),
    /// A MODRES record, having information about modifications of atoms
    ///  * IDCode
    ///  * ResName
    ///  * ChainID
    ///  * SeqNum
    ///  * InsertionCode
    ///  * Standard residue name
    ///  * Comment
    Modres(
        String,
        String,
        String,
        isize,
        Option<String>,
        String,
        String,
    ),
    /// A disulfide bond
    /// * Residue name 1 (CYS)
    /// * Residue serial number 1
    /// * Insertion code 1
    /// * Chain id 1
    /// * Residue name 2 (CYS)
    /// * Residue serial number 2
    /// * Insertion code 2
    /// * Chain id 2
    /// * Symmetry operation residue 1
    /// * Symmetry operation residue 2
    /// * Bond length
    SSBond(
        (String, isize, Option<String>, String),
        (String, isize, Option<String>, String),
        Option<(String, String, f64)>,
    ),
    /// ENDMODEL, end of the current model
    EndModel(),
    /// TER =, termination of ATOM lines to allow for HETATMs to be defined
    TER(),
    /// END, end of the whole file
    End(),
    /// Empty line, just ignore
    Empty(),
}
