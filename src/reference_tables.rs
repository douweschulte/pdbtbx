#![allow(dead_code)]
#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Gets the index (into Int. Crys. Handbook Vol A 2016) for the given symbol. First it is
/// interpreted as a Herman Mauguin symbol, if that is unsuccessful it is interpreted as a
/// Hall symbol.
pub fn get_index_for_symbol(symbol: &str) -> Option<usize> {
    if let Some(index) = HERMANN_MAUGUIN_SYMBOL.iter().position(|i| *i == symbol) {
        Some(index + 1)
    } else {
        HALL_SYMBOL.iter().position(|i| *i == symbol).map(|n| n + 1)
    }
}

/// Gets the index (into Int. Crys. Handbook Vol A 2016) for the given symbol in parallel. First it is
/// interpreted as a Herman Mauguin symbol, if that is unsuccessful it is interpreted as a
/// Hall symbol.
#[cfg(feature = "rayon")]
pub fn par_get_index_for_symbol(symbol: &str) -> Option<usize> {
    if let Some(index) = HERMANN_MAUGUIN_SYMBOL
        .par_iter()
        .position_any(|i| *i == symbol)
    {
        Some(index + 1)
    } else {
        HALL_SYMBOL
            .par_iter()
            .position_any(|i| *i == symbol)
            .map(|n| n + 1)
    }
}

/// Gets the Herman Mauguin symbol for the given index (into Int. Crys. Handbook Vol A 2016)
pub fn get_herman_mauguin_symbol_for_index(index: usize) -> Option<&'static str> {
    HERMANN_MAUGUIN_SYMBOL.get(index - 1).copied()
}

/// Gets the Hall symbol for the given index (into Int. Crys. Handbook Vol A 2016)
pub fn get_hall_symbol_for_index(index: usize) -> Option<&'static str> {
    HALL_SYMBOL.get(index - 1).copied()
}

/// Gets the transformations given an index (into Int. Crys. Handbook Vol A 2016) for the given space group
pub fn get_transformation(index: usize) -> Option<&'static [[[f64; 4]; 3]]> {
    SYMBOL_TRANSFORMATION.get(index - 1).copied()
}

/// Gets the atomic number for the given element. It is case insensitive for the element name.
pub fn get_atomic_number(element: &str) -> Option<usize> {
    let mut counter = 1;
    let element = element.to_ascii_uppercase();
    for item in ELEMENT_SYMBOLS {
        if item == &element {
            return Some(counter);
        }
        counter += 1;
    }
    None
}

/// Gets the atomic radius for the given atomic number (defined up until 'Cm' 96) in Å.
/// Source: Martin Rahm, Roald Hoffmann, and N. W. Ashcroft. Atomic and Ionic Radii of Elements 1-96. Chemistry - A European Journal, 22(41):14625–14632, oct 2016. URL: http://doi.org/10.1002/chem.201602949, doi:10.1002/chem.201602949.
/// Updated to the corrigendum: <https://doi.org/10.1002/chem.201700610>
pub fn get_atomic_radius(atomic_number: usize) -> Option<f64> {
    ELEMENT_ATOMIC_RADII.get(atomic_number - 1).copied()
}

/// Gets the van der Waals radius for the given atomic number (defined up until 'Es' 99) in Å.
/// Source: Alvarez, S. (2013). A cartography of the van der Waals territories. Dalton Transactions, 42(24), 8617. <https://doi.org/10.1039/c3dt50599e>
pub fn get_vanderwaals_radius(atomic_number: usize) -> Option<f64> {
    ELEMENT_VANDERWAALS_RADII.get(atomic_number - 1).copied()
}

/// Gets the covalent bond radii for the given atomic number (defined for all elements (<=118)).
/// The result is the radius for a single, double and triple bond, where the last two are optional.
/// All values are given in Å.
/// Sources:
///  * P. Pyykkö; M. Atsumi (2009). "Molecular Single-Bond Covalent Radii for Elements 1-118". Chemistry: A European Journal. 15 (1): 186–197. doi:10.1002/chem.200800987
///  * P. Pyykkö; M. Atsumi (2009). "Molecular Double-Bond Covalent Radii for Elements Li–E112". Chemistry: A European Journal. 15 (46): 12770–12779. doi:10.1002/chem.200901472
///  * P. Pyykkö; S. Riedel; M. Patzschke (2005). "Triple-Bond Covalent Radii". Chemistry: A European Journal. 11 (12): 3511–3520. doi:10.1002/chem.200401299
pub fn get_covalent_bond_radii(atomic_number: usize) -> (f64, Option<f64>, Option<f64>) {
    *ELEMENT_BOND_RADII
        .get(atomic_number - 1)
        .expect("Invalid atomic number provided for element bond radius lookup. The number should be less than or equal to 118.")
}

/// Gets the amino acid number into the table, effectively providing the recognition of it being an amino acid or not
pub fn get_amino_acid_number(aa: &str) -> Option<usize> {
    let mut counter = 1;
    for item in AMINO_ACIDS {
        if *item == aa {
            return Some(counter);
        }
        counter += 1;
    }
    None
}

/// Returns if the given atom name is a name for an atom in the backbone of a protein
pub fn is_backbone(name: &str) -> bool {
    BACKBONE_NAMES.contains(&name)
}

/// Returns if the given number is a valid remark-type-number (according to wwPDB v 3.30)
pub fn valid_remark_type_number(number: usize) -> bool {
    REMARK_TYPES.contains(&number)
}

/// The valid remark type numbers as of PDB v3.30
const REMARK_TYPES: [usize; 41] = [
    0, 1, 2, 3, 4, 5, 100, 200, 205, 210, 215, 217, 230, 240, 245, 247, 250, 265, 280, 285, 290,
    300, 350, 375, 450, 465, 470, 475, 480, 500, 525, 600, 610, 615, 620, 630, 650, 700, 800, 900,
    999,
];

/// The symbols/names of the elements of the periodic table
const ELEMENT_SYMBOLS: &[&str] = &[
    "H", "HE", "LI", "BE", "B", "C", "N", "O", "F", "NE", "NA", "MG", "AL", "SI", "P", "S", "CL",
    "AR", "K", "CA", "SC", "TI", "V", "CR", "MN", "FE", "CO", "NI", "CU", "ZN", "GA", "GE", "AS",
    "SE", "BR", "KR", "RB", "SR", "Y", "ZR", "NB", "MO", "TC", "RU", "RH", "PD", "AG", "CD", "IN",
    "SN", "SB", "TE", "I", "XE", "CS", "BA", "LA", "CE", "PR", "ND", "PM", "SM", "EU", "GD", "TB",
    "DY", "HO", "ER", "TM", "YB", "LU", "HF", "TA", "W", "RE", "OS", "IR", "PT", "AU", "HG", "TL",
    "PB", "BI", "PO", "AT", "RN", "FR", "RA", "AC", "TH", "PA", "U", "NP", "PU", "AM", "CM", "BK",
    "CF", "ES", "FM", "MD", "NO", "LR", "RF", "DB", "SG", "BH", "HS", "MT", "DS", "RG", "CN", "NH",
    "FL", "MC", "LV", "TS", "OG",
];

/// The radii of the elements up to Cs 96
const ELEMENT_ATOMIC_RADII: &[f64] = &[
    1.54, 1.34, 2.20, 2.19, 2.05, 1.90, 1.79, 1.71, 1.63, 1.56, 2.25, 2.40, 2.39, 2.32, 2.23, 2.14,
    2.06, 1.97, 2.34, 2.70, 2.63, 2.57, 2.52, 2.33, 2.42, 2.37, 2.33, 2.29, 2.17, 2.22, 2.33, 2.34,
    2.31, 2.24, 2.19, 2.12, 2.40, 2.79, 2.74, 2.69, 2.51, 2.44, 2.52, 2.37, 2.33, 2.15, 2.25, 2.38,
    2.46, 2.48, 2.46, 2.42, 2.38, 2.32, 2.49, 2.93, 2.84, 2.82, 2.86, 2.84, 2.83, 2.80, 2.80, 2.77,
    2.76, 2.75, 2.73, 2.72, 2.71, 2.77, 2.70, 2.64, 2.58, 2.53, 2.49, 2.44, 2.40, 2.30, 2.26, 2.29,
    2.42, 2.49, 2.50, 2.50, 2.47, 2.43, 2.58, 2.92, 2.93, 2.88, 2.85, 2.83, 2.81, 2.78, 2.76, 2.64,
];

/// The van der waals radii of the elements up to Es 99
const ELEMENT_VANDERWAALS_RADII: &[f64] = &[
    1.20, 1.43, 2.12, 1.98, 1.91, 1.77, 1.66, 1.50, 1.46, 1.58, 2.50, 2.51, 2.25, 2.19, 1.90, 1.89,
    1.82, 1.83, 2.73, 2.62, 2.58, 2.46, 2.42, 2.45, 2.45, 2.44, 2.40, 2.40, 2.38, 2.39, 2.32, 2.29,
    1.88, 1.82, 1.86, 2.25, 3.21, 2.84, 2.75, 2.52, 2.56, 2.45, 2.44, 2.46, 2.44, 2.15, 2.53, 2.49,
    2.43, 2.42, 2.47, 1.99, 2.04, 2.06, 3.48, 3.03, 2.98, 2.88, 2.92, 2.95, 2.90, 2.87, 2.83, 2.79,
    2.87, 2.81, 2.83, 2.79, 2.80, 2.74, 2.63, 2.53, 2.57, 2.49, 2.48, 2.41, 2.29, 2.32, 2.45, 2.47,
    2.60, 2.54, 2.80, 2.93, 2.88, 2.71, 2.82, 2.81, 2.83, 3.05, 3.40, 3.05, 2.70,
];

/// The bond radii of all elements
const ELEMENT_BOND_RADII: &[(f64, Option<f64>, Option<f64>)] = &[
    (0.32, None, None),
    (0.46, None, None),
    (1.33, Some(1.24), None),
    (1.02, Some(0.90), Some(0.85)),
    (0.85, Some(0.78), Some(0.73)),
    (0.75, Some(0.67), Some(0.60)),
    (0.71, Some(0.60), Some(0.54)),
    (0.63, Some(0.57), Some(0.53)),
    (0.64, Some(0.59), Some(0.53)),
    (0.67, Some(0.96), None),
    (1.55, Some(1.60), None),
    (1.39, Some(1.32), Some(1.27)),
    (1.26, Some(1.13), Some(1.11)),
    (1.16, Some(1.07), Some(1.02)),
    (1.11, Some(1.02), Some(0.94)),
    (1.03, Some(0.94), Some(0.95)),
    (0.99, Some(0.95), Some(0.93)),
    (0.96, Some(1.07), Some(0.96)),
    (1.96, Some(1.93), None),
    (1.71, Some(1.47), Some(1.33)),
    (1.48, Some(1.16), Some(1.14)),
    (1.36, Some(1.17), Some(1.08)),
    (1.34, Some(1.12), Some(1.06)),
    (1.22, Some(1.11), Some(1.03)),
    (1.19, Some(1.05), Some(1.03)),
    (1.16, Some(1.09), Some(1.02)),
    (1.11, Some(1.03), Some(9.6)),
    (1.10, Some(1.01), Some(1.01)),
    (1.12, Some(1.15), Some(1.20)),
    (1.18, Some(1.20), None),
    (1.24, Some(1.17), Some(1.21)),
    (1.21, Some(1.11), Some(1.14)),
    (1.21, Some(1.14), Some(1.06)),
    (1.16, Some(1.07), Some(1.07)),
    (1.14, Some(1.09), Some(1.10)),
    (1.17, Some(1.21), Some(1.08)),
    (2.10, Some(2.02), None),
    (1.85, Some(1.57), Some(1.39)),
    (1.63, Some(1.30), Some(1.24)),
    (1.54, Some(1.27), Some(1.21)),
    (1.47, Some(1.25), Some(1.16)),
    (1.38, Some(1.21), Some(1.13)),
    (1.28, Some(1.20), Some(1.10)),
    (1.25, Some(1.14), Some(1.03)),
    (1.25, Some(1.10), Some(1.06)),
    (1.20, Some(1.17), Some(1.12)),
    (1.28, Some(1.39), Some(1.37)),
    (1.36, Some(1.44), None),
    (1.42, Some(1.36), Some(1.46)),
    (1.40, Some(1.30), Some(1.32)),
    (1.40, Some(1.33), Some(1.27)),
    (1.36, Some(1.28), Some(1.21)),
    (1.33, Some(1.29), Some(1.25)),
    (1.31, Some(1.35), Some(1.22)),
    (2.32, Some(2.09), None),
    (1.96, Some(1.61), Some(1.49)),
    (1.80, Some(1.39), Some(1.39)),
    (1.63, Some(1.37), Some(1.31)),
    (1.76, Some(1.38), Some(1.28)),
    (1.74, Some(1.37), None),
    (1.73, Some(1.35), None),
    (1.72, Some(1.34), None),
    (1.68, Some(1.34), None),
    (1.69, Some(1.35), Some(1.32)),
    (1.68, Some(1.35), None),
    (1.67, Some(1.33), None),
    (1.66, Some(1.33), None),
    (1.65, Some(1.33), None),
    (1.64, Some(1.31), None),
    (1.70, Some(1.29), None),
    (1.62, Some(1.31), Some(1.31)),
    (1.52, Some(1.28), Some(1.22)),
    (1.46, Some(1.26), Some(1.19)),
    (1.37, Some(1.20), Some(1.15)),
    (1.31, Some(1.19), Some(1.10)),
    (1.29, Some(1.16), Some(1.09)),
    (1.22, Some(1.15), Some(1.07)),
    (1.23, Some(1.12), Some(1.10)),
    (1.24, Some(1.21), Some(1.23)),
    (1.33, Some(1.42), None),
    (1.44, Some(1.42), Some(1.50)),
    (1.44, Some(1.35), Some(1.37)),
    (1.51, Some(1.41), Some(1.35)),
    (1.45, Some(1.35), Some(1.29)),
    (1.47, Some(1.38), Some(1.38)),
    (1.42, Some(1.45), Some(1.33)),
    (2.23, Some(2.18), None),
    (2.01, Some(1.73), Some(1.59)),
    (1.86, Some(1.53), Some(1.40)),
    (1.75, Some(1.43), Some(1.36)),
    (1.69, Some(1.38), Some(1.29)),
    (1.70, Some(1.34), Some(1.18)),
    (1.71, Some(1.36), Some(1.16)),
    (1.72, Some(1.35), None),
    (1.66, Some(1.35), None),
    (1.66, Some(1.36), None),
    (1.68, Some(1.39), None),
    (1.68, Some(1.40), None),
    (1.65, Some(1.40), None),
    (1.67, Some(1.67), Some(1.67)),
    (1.73, Some(1.39), None),
    (1.76, Some(1.76), Some(1.76)),
    (1.61, Some(1.41), None),
    (1.57, Some(1.40), Some(1.31)),
    (1.49, Some(1.36), Some(1.26)),
    (1.43, Some(1.28), Some(1.21)),
    (1.41, Some(1.28), Some(1.19)),
    (1.34, Some(1.25), Some(1.18)),
    (1.29, Some(1.25), Some(1.13)),
    (1.28, Some(1.16), Some(1.12)),
    (1.21, Some(1.16), Some(1.18)),
    (1.22, Some(1.37), Some(1.30)),
    (1.36, None, None),
    (1.43, None, None),
    (1.62, None, None),
    (1.75, None, None),
    (1.65, None, None),
    (1.57, None, None),
];

/// All amino acids. Includes Amber-specific naming conventions for (de-)protonated versions, CYS involved in
/// disulfide bonding and the like.
const AMINO_ACIDS: &[&str] = &[
    "ALA", "ARG", "ASH", "ASN", "ASP", "CYS", "CYX", "GLH", "GLN", "GLU", "GLY", "HID", "HIE",
    "HIM", "HIP", "HIS", "ILE", "LEU", "LYN", "LYS", "MET", "PHE", "PRO", "SER", "THR", "TRP",
    "TYR", "VAL",
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
