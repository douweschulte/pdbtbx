#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Gets the index (into Int. Crys. Handbook Vol A 2016) for the given symbol. First it is
/// interpreted as a Herman Mauguin symbol, if that is unsuccessful it is interpreted as a
/// Hall symbol.
pub(crate) fn get_index_for_symbol(symbol: impl AsRef<str>) -> Option<usize> {
    let symbol = symbol.as_ref();
    HERMANN_MAUGUIN_SYMBOL
        .iter()
        .position(|i| *i == symbol)
        .map_or_else(
            || HALL_SYMBOL.iter().position(|i| *i == symbol).map(|n| n + 1),
            |index| Some(index + 1),
        )
}

/// Gets the index (into Int. Crys. Handbook Vol A 2016) for the given symbol in parallel. First it is
/// interpreted as a Herman Mauguin symbol, if that is unsuccessful it is interpreted as a
/// Hall symbol.
#[cfg(feature = "rayon")]
#[allow(dead_code)]
pub(crate) fn par_get_index_for_symbol(symbol: impl AsRef<str>) -> Option<usize> {
    let symbol = symbol.as_ref();
    HERMANN_MAUGUIN_SYMBOL
        .par_iter()
        .position_any(|i| *i == symbol)
        .map_or_else(
            || {
                HALL_SYMBOL
                    .par_iter()
                    .position_any(|i| *i == symbol)
                    .map(|n| n + 1)
            },
            |index| Some(index + 1),
        )
}

/// Gets the Herman Mauguin symbol for the given index (into Int. Crys. Handbook Vol A 2016)
pub(crate) fn get_herman_mauguin_symbol_for_index(index: usize) -> Option<&'static str> {
    HERMANN_MAUGUIN_SYMBOL.get(index - 1).copied()
}

/// Gets the Hall symbol for the given index (into Int. Crys. Handbook Vol A 2016)
pub(crate) fn get_hall_symbol_for_index(index: usize) -> Option<&'static str> {
    HALL_SYMBOL.get(index - 1).copied()
}

/// Gets the transformations given an index (into Int. Crys. Handbook Vol A 2016) for the given space group
pub(crate) fn get_transformation(index: usize) -> Option<&'static [[[f64; 4]; 3]]> {
    SYMBOL_TRANSFORMATION.get(index - 1).copied()
}

/// Returns if the given atom name is a common amino acid
pub(crate) fn is_amino_acid(aa: impl AsRef<str>) -> bool {
    AMINO_ACIDS.contains(&aa.as_ref())
}

/// Returns if the given atom name is a name for an atom in the backbone of a protein
pub(crate) fn is_backbone(name: impl AsRef<str>) -> bool {
    BACKBONE_NAMES.contains(&name.as_ref())
}

/// Returns if the given number is a valid remark-type-number (according to wwPDB v 3.30)
pub(crate) fn valid_remark_type_number(number: usize) -> bool {
    REMARK_TYPES.contains(&number)
}

/// The valid remark type numbers as of PDB v3.30
const REMARK_TYPES: [usize; 42] = [
    0, 1, 2, 3, 4, 5, 100, 200, 205, 210, 215, 217, 230, 240, 245, 247, 250, 265, 280, 285, 290,
    300, 350, 375, 400, 450, 465, 470, 475, 480, 500, 525, 600, 610, 615, 620, 630, 650, 700, 800,
    900, 999,
];

/// All amino acids. Includes Amber-specific naming conventions for (de-)protonated versions, CYS involved in
/// disulfide bonding and the like.
const AMINO_ACIDS: &[&str] = &[
    "ALA", "ARG", "ASH", "ASN", "ASP", "ASX", "CYS", "CYX", "GLH", "GLN", "GLU", "GLY", "HID",
    "HIE", "HIM", "HIP", "HIS", "ILE", "LEU", "LYN", "LYS", "MET", "PHE", "PRO", "SER", "THR",
    "TRP", "TYR", "VAL", "SEC", "PYL",
];

/// The names of atom in the backbone of proteins
const BACKBONE_NAMES: &[&str] = &[
    "N", "CA", "C", "O", "H", "H1", "H2", "H3", "HA", "HA2", "HA3",
];

/// The list of Hermann Mauguin symbols in the same order as in the handbook
const HERMANN_MAUGUIN_SYMBOL: &[&str] = include!("reference/hermann_mauguin_symbols.txt");

/// The list of Hall crystal symmetry symbols in the same order as in the handbook (and the Hermann Mauguin table above)
const HALL_SYMBOL: &[&str] = include!("reference/hall_symbols.txt");

/// Reworked from CCTBX output (Jan 2021)
const SYMBOL_TRANSFORMATION: &[&[[[f64; 4]; 3]]] =
    include!("reference/crystal_transformations.txt");
