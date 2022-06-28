use std::convert::TryInto;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// All elements from the periodic system.
#[allow(missing_docs)]
pub enum Element {
    /// Element Hydrogen (H) atomic number: 1
    H = 1,
    /// Element Helium (He) atomic number: 2
    He,
    /// Element Lithium (Li) atomic number: 3
    Li,
    /// Element Beryllium (Be) atomic number: 4
    Be,
    /// Element Boron (B) atomic number: 5
    B,
    /// Element Carbon (C) atomic number: 6
    C,
    /// Element Nitrogen (N) atomic number: 7
    N,
    /// Element Oxygen (O) atomic number: 8
    O,
    /// Element Fluorine (F) atomic number: 9
    F,
    /// Element Neon (Ne) atomic number: 10
    Ne,
    /// Element Sodium (Na) atomic number: 11
    Na,
    /// Element Magnesium (Mg) atomic number: 12
    Mg,
    /// Element Aluminium (Al) atomic number: 13
    Al,
    /// Element Silicon (Si) atomic number: 14
    Si,
    /// Element Phosphorus (P) atomic number: 15
    P,
    /// Element Sulfur (S) atomic number: 16
    S,
    /// Element Chlorine (Cl) atomic number: 17
    Cl,
    /// Element Argon (Ar) atomic number: 18
    Ar,
    /// Element Potassium (K) atomic number: 19
    K,
    /// Element Calcium (Ca) atomic number: 20
    Ca,
    /// Element Scandium (Sc) atomic number: 21
    Sc,
    /// Element Titanium (Ti) atomic number: 22
    Ti,
    /// Element Vanadium (V) atomic number: 23
    V,
    /// Element Chromium (Cr) atomic number: 24
    Cr,
    /// Element Manganese (Mn) atomic number: 25
    Mn,
    /// Element Iron (Fe) atomic number: 26
    Fe,
    /// Element Cobalt (Co) atomic number: 27
    Co,
    /// Element Nickel (Ni) atomic number: 28
    Ni,
    /// Element Copper (Cu) atomic number: 29
    Cu,
    /// Element Zinc (Zn) atomic number: 30
    Zn,
    /// Element Gallium (Ga) atomic number: 31
    Ga,
    /// Element Germanium (Ge) atomic number: 32
    Ge,
    /// Element Arsenic (As) atomic number: 33
    As,
    /// Element Selenium (Se) atomic number: 34
    Se,
    /// Element Bromine (Br) atomic number: 35
    Br,
    /// Element Krypton (Kr) atomic number: 36
    Kr,
    /// Element Rubidium (Rb) atomic number: 37
    Rb,
    /// Element Strontium (Sr) atomic number: 38
    Sr,
    /// Element Yttrium (Y) atomic number: 39
    Y,
    /// Element Zirconium (Zr) atomic number: 40
    Zr,
    /// Element Niobium (Nb) atomic number: 41
    Nb,
    /// Element Molybdenum (Mo) atomic number: 42
    Mo,
    /// Element Technetium (Tc) atomic number: 43
    Tc,
    /// Element Ruthenium (Ru) atomic number: 44
    Ru,
    /// Element Rhodium (Rh) atomic number: 45
    Rh,
    /// Element Palladium (Pd) atomic number: 46
    Pd,
    /// Element Silver (Ag) atomic number: 47
    Ag,
    /// Element Cadmium (Cd) atomic number: 48
    Cd,
    /// Element Indium (In) atomic number: 49
    In,
    /// Element Tin (Sn) atomic number: 50
    Sn,
    /// Element Antimony (Sb) atomic number: 51
    Sb,
    /// Element Tellurium (Te) atomic number: 52
    Te,
    /// Element Iodine (I) atomic number: 53
    I,
    /// Element Xenon (Xe) atomic number: 54
    Xe,
    /// Element Caesium (Cs) atomic number: 55
    Cs,
    /// Element Barium (Ba) atomic number: 56
    Ba,
    /// Element Lanthanum (La) atomic number: 57
    La,
    /// Element Cerium (Ce) atomic number: 58
    Ce,
    /// Element Praseodymium (Pr) atomic number: 59
    Pr,
    /// Element Neodymium (Nd) atomic number: 60
    Nd,
    /// Element Promethium (Pm) atomic number: 61
    Pm,
    /// Element Samarium (Sm) atomic number: 62
    Sm,
    /// Element Europium (Eu) atomic number: 63
    Eu,
    /// Element Gadolinium (Gd) atomic number: 64
    Gd,
    /// Element Terbium (Tb) atomic number: 65
    Tb,
    /// Element Dysprosium (Dy) atomic number: 66
    Dy,
    /// Element Holmium (Ho) atomic number: 67
    Ho,
    /// Element Erbium (Er) atomic number: 68
    Er,
    /// Element Thulium (Tm) atomic number: 69
    Tm,
    /// Element Ytterbium (Yb) atomic number: 70
    Yb,
    /// Element Lutetium (Lu) atomic number: 71
    Lu,
    /// Element Hafnium (Hf) atomic number: 72
    Hf,
    /// Element Tantalum (Ta) atomic number: 73
    Ta,
    /// Element Tungsten (W) atomic number: 74
    W,
    /// Element Rhenium (Re) atomic number: 75
    Re,
    /// Element Osmium (Os) atomic number: 76
    Os,
    /// Element Iridium (Ir) atomic number: 77
    Ir,
    /// Element Platinum (Pt) atomic number: 78
    Pt,
    /// Element Gold (Au) atomic number: 79
    Au,
    /// Element Mercury (Hg) atomic number: 80
    Hg,
    /// Element Thallium (Tl) atomic number: 81
    Tl,
    /// Element Lead (Pb) atomic number: 82
    Pb,
    /// Element Bismuth (Bi) atomic number: 83
    Bi,
    /// Element Polonium (Po) atomic number: 84
    Po,
    /// Element Astatine (At) atomic number: 85
    At,
    /// Element Radon (Rn) atomic number: 86
    Rn,
    /// Element Francium (Fr) atomic number: 87
    Fr,
    /// Element Radium (Ra) atomic number: 88
    Ra,
    /// Element Actinium (Ac) atomic number: 89
    Ac,
    /// Element Thorium (Th) atomic number: 90
    Th,
    /// Element Protactinium (Pa) atomic number: 91
    Pa,
    /// Element Uranium (U) atomic number: 92
    U,
    /// Element Neptunium (Np) atomic number: 93
    Np,
    /// Element Plutonium (Pu) atomic number: 94
    Pu,
    /// Element Americium (Am) atomic number: 95
    Am,
    /// Element Curium (Cm) atomic number: 96
    Cm,
    /// Element Berkelium (Bk) atomic number: 97
    Bk,
    /// Element Californium (Cf) atomic number: 98
    Cf,
    /// Element Einsteinium (Es) atomic number: 99
    Es,
    /// Element Fermium (Fm) atomic number: 100
    Fm,
    /// Element Mendelevium (Md) atomic number: 101
    Md,
    /// Element Nobelium (No) atomic number: 102
    No,
    /// Element Lawrencium (Lr) atomic number: 103
    Lr,
    /// Element Rutherfordium (Rf) atomic number: 104
    Rf,
    /// Element Dubnium (Db) atomic number: 105
    Db,
    /// Element Seaborgium (Sg) atomic number: 106
    Sg,
    /// Element Bohrium (Bh) atomic number: 107
    Bh,
    /// Element Hassium (Hs) atomic number: 108
    Hs,
    /// Element Meitnerium (Mt) atomic number: 109
    Mt,
    /// Element Darmstadtium (Ds) atomic number: 110
    Ds,
    /// Element Roentgenium (Rg) atomic number: 111
    Rg,
    /// Element Copernicium (Cn) atomic number: 112
    Cn,
    /// Element Nihonium (Nh) atomic number: 113
    Nh,
    /// Element Flerovium (Fl) atomic number: 114
    Fl,
    /// Element Moscovium (Mc) atomic number: 115
    Mc,
    /// Element Livermorium (Lv) atomic number: 116
    Lv,
    /// Element Tennessine (Ts) atomic number: 117
    Ts,
    /// Element Oganesson (Og) atomic number: 118
    Og,
}

impl Element {
    /// Get the number in the periodic system of the elements for this element.
    pub const fn atomic_number(&self) -> usize {
        *self as usize
    }

    /// Get an element based on the atomic number
    pub const fn new(atomic_number: usize) -> Option<Element> {
        match atomic_number {
            1 => Some(Element::H),
            2 => Some(Element::He),
            3 => Some(Element::Li),
            4 => Some(Element::Be),
            5 => Some(Element::B),
            6 => Some(Element::C),
            7 => Some(Element::N),
            8 => Some(Element::O),
            9 => Some(Element::F),
            10 => Some(Element::Ne),
            11 => Some(Element::Na),
            12 => Some(Element::Mg),
            13 => Some(Element::Al),
            14 => Some(Element::Si),
            15 => Some(Element::P),
            16 => Some(Element::S),
            17 => Some(Element::Cl),
            18 => Some(Element::Ar),
            19 => Some(Element::K),
            20 => Some(Element::Ca),
            21 => Some(Element::Sc),
            22 => Some(Element::Ti),
            23 => Some(Element::V),
            24 => Some(Element::Cr),
            25 => Some(Element::Mn),
            26 => Some(Element::Fe),
            27 => Some(Element::Co),
            28 => Some(Element::Ni),
            29 => Some(Element::Cu),
            30 => Some(Element::Zn),
            31 => Some(Element::Ga),
            32 => Some(Element::Ge),
            33 => Some(Element::As),
            34 => Some(Element::Se),
            35 => Some(Element::Br),
            36 => Some(Element::Kr),
            37 => Some(Element::Rb),
            38 => Some(Element::Sr),
            39 => Some(Element::Y),
            40 => Some(Element::Zr),
            41 => Some(Element::Nb),
            42 => Some(Element::Mo),
            43 => Some(Element::Tc),
            44 => Some(Element::Ru),
            45 => Some(Element::Rh),
            46 => Some(Element::Pd),
            47 => Some(Element::Ag),
            48 => Some(Element::Cd),
            49 => Some(Element::In),
            50 => Some(Element::Sn),
            51 => Some(Element::Sb),
            52 => Some(Element::Te),
            53 => Some(Element::I),
            54 => Some(Element::Xe),
            55 => Some(Element::Cs),
            56 => Some(Element::Ba),
            57 => Some(Element::La),
            58 => Some(Element::Ce),
            59 => Some(Element::Pr),
            60 => Some(Element::Nd),
            61 => Some(Element::Pm),
            62 => Some(Element::Sm),
            63 => Some(Element::Eu),
            64 => Some(Element::Gd),
            65 => Some(Element::Tb),
            66 => Some(Element::Dy),
            67 => Some(Element::Ho),
            68 => Some(Element::Er),
            69 => Some(Element::Tm),
            70 => Some(Element::Yb),
            71 => Some(Element::Lu),
            72 => Some(Element::Hf),
            73 => Some(Element::Ta),
            74 => Some(Element::W),
            75 => Some(Element::Re),
            76 => Some(Element::Os),
            77 => Some(Element::Ir),
            78 => Some(Element::Pt),
            79 => Some(Element::Au),
            80 => Some(Element::Hg),
            81 => Some(Element::Tl),
            82 => Some(Element::Pb),
            83 => Some(Element::Bi),
            84 => Some(Element::Po),
            85 => Some(Element::At),
            86 => Some(Element::Rn),
            87 => Some(Element::Fr),
            88 => Some(Element::Ra),
            89 => Some(Element::Ac),
            90 => Some(Element::Th),
            91 => Some(Element::Pa),
            92 => Some(Element::U),
            93 => Some(Element::Np),
            94 => Some(Element::Pu),
            95 => Some(Element::Am),
            96 => Some(Element::Cm),
            97 => Some(Element::Bk),
            98 => Some(Element::Cf),
            99 => Some(Element::Es),
            100 => Some(Element::Fm),
            101 => Some(Element::Md),
            102 => Some(Element::No),
            103 => Some(Element::Lr),
            104 => Some(Element::Rf),
            105 => Some(Element::Db),
            106 => Some(Element::Sg),
            107 => Some(Element::Bh),
            108 => Some(Element::Hs),
            109 => Some(Element::Mt),
            110 => Some(Element::Ds),
            111 => Some(Element::Rg),
            112 => Some(Element::Cn),
            113 => Some(Element::Nh),
            114 => Some(Element::Fl),
            115 => Some(Element::Mc),
            116 => Some(Element::Lv),
            117 => Some(Element::Ts),
            118 => Some(Element::Og),
            _ => None,
        }
    }

    /// Get an element based on the symbol, eg "He" for Helium
    pub fn from_symbol(symbol: impl AsRef<str>) -> Option<Element> {
        symbol.as_ref().try_into().ok()
    }

    /// Get the symbol for this element eg "He" for Helium
    pub fn symbol(&self) -> &'static str {
        ELEMENT_SYMBOLS[self.atomic_number() - 1]
    }

    /// Get the full name for an element, eg "Helium" for Element::He
    pub const fn full_name(&self) -> &'static str {
        ELEMENT_NAMES[self.atomic_number() - 1]
    }

    /// Get the [AtomicRadius] for this element, see the documentation for the struct to get more information.
    pub const fn atomic_radius(&self) -> &'static AtomicRadius {
        &ELEMENT_ATOMIC_RADII[self.atomic_number() - 1]
    }

    /// Get the atomic weight for the given element. Applicable for all normal materials.
    /// The mean value is given for the uncertainty surrounding the values for all elements.
    /// Source: CIAAW. Atomic weights of the elements 2020. Available online at <https://www.ciaaw.org/atomic-weights.htm>.
    pub const fn weight(&self) -> Option<f64> {
        ELEMENT_WEIGHT[self.atomic_number() - 1]
    }

    /// Get the Pauling electronegativity for the element.
    /// Source: WM Haynes (ed), CRC Handbook of Chemistry and Physics, 95th Edition. CRC Press. Boca Raton, Florida, 2014-2015; Section 9, Molecular Structure and Spectroscopy; Electronegativity
    pub const fn electro_negativity(&self) -> Option<f64> {
        ELEMENT_ELECTRON_NEGATIVITY[self.atomic_number() - 1]
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::convert::TryFrom<&str> for Element {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_ascii_uppercase();
        if let Some(element) = ELEMENT_SYMBOLS
            .iter()
            .position(|name| *name == value.as_str())
            .and_then(|n| Element::new(n + 1))
        {
            Ok(element)
        } else {
            Err("Invalid element code")
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::convert::TryInto;

    use crate::Element;

    #[test]
    fn atomic_number() {
        assert_eq!(Element::Og.atomic_number(), 118);
        assert_eq!(Element::Og, Element::new(118).unwrap());
    }

    #[test]
    fn display() {
        assert_eq!(Element::Lv.to_string(), "Lv");
        assert_eq!(Element::Cl.to_string(), "Cl");
        let element: Element = "Cl".try_into().unwrap();
        assert_eq!(Element::Cl.atomic_number(), element.atomic_number());
    }
}

/// The symbols of the elements of the periodic table
const ELEMENT_SYMBOLS: [&str; 118] = [
    "H", "HE", "LI", "BE", "B", "C", "N", "O", "F", "NE", "NA", "MG", "AL", "SI", "P", "S", "CL",
    "AR", "K", "CA", "SC", "TI", "V", "CR", "MN", "FE", "CO", "NI", "CU", "ZN", "GA", "GE", "AS",
    "SE", "BR", "KR", "RB", "SR", "Y", "ZR", "NB", "MO", "TC", "RU", "RH", "PD", "AG", "CD", "IN",
    "SN", "SB", "TE", "I", "XE", "CS", "BA", "LA", "CE", "PR", "ND", "PM", "SM", "EU", "GD", "TB",
    "DY", "HO", "ER", "TM", "YB", "LU", "HF", "TA", "W", "RE", "OS", "IR", "PT", "AU", "HG", "TL",
    "PB", "BI", "PO", "AT", "RN", "FR", "RA", "AC", "TH", "PA", "U", "NP", "PU", "AM", "CM", "BK",
    "CF", "ES", "FM", "MD", "NO", "LR", "RF", "DB", "SG", "BH", "HS", "MT", "DS", "RG", "CN", "NH",
    "FL", "MC", "LV", "TS", "OG",
];

/// The names of the elements of the periodic table
const ELEMENT_NAMES: [&str; 118] = [
    "Hydrogen",
    "Helium",
    "Lithium",
    "Beryllium",
    "Boron",
    "Carbon",
    "Nitrogen",
    "Oxygen",
    "Fluorine",
    "Neon",
    "Sodium",
    "Magnesium",
    "Aluminum",
    "Silicon",
    "Phosphorus",
    "Sulfur",
    "Chlorine",
    "Argon",
    "Potassium",
    "Calcium",
    "Scandium",
    "Titanium",
    "Vanadium",
    "Chromium",
    "Manganese",
    "Iron",
    "Cobalt",
    "Nickel",
    "Copper",
    "Zinc",
    "Gallium",
    "Germanium",
    "Arsenic",
    "Selenium",
    "Bromine",
    "Krypton",
    "Rubidium",
    "Strontium",
    "Yttrium",
    "Zirconium",
    "Niobium",
    "Molybdenum",
    "Technetium",
    "Ruthenium",
    "Rhodium",
    "Palladium",
    "Silver",
    "Cadmium",
    "Indium",
    "Tin",
    "Antimony",
    "Tellurium",
    "Iodine",
    "Xenon",
    "Cesium",
    "Barium",
    "Lanthanum",
    "Cerium",
    "Praseodymium",
    "Neodymium",
    "Promethium",
    "Samarium",
    "Europium",
    "Gadolinium",
    "Terbium",
    "Dysprosium",
    "Holmium",
    "Erbium",
    "Thulium",
    "Ytterbium",
    "Lutetium",
    "Hafnium",
    "Tantalum",
    "Wolfram",
    "Rhenium",
    "Osmium",
    "Iridium",
    "Platinum",
    "Gold",
    "Mercury",
    "Thallium",
    "Lead",
    "Bismuth",
    "Polonium",
    "Astatine",
    "Radon",
    "Francium",
    "Radium",
    "Actinium",
    "Thorium",
    "Protactinium",
    "Uranium",
    "Neptunium",
    "Plutonium",
    "Americium",
    "Curium",
    "Berkelium",
    "Californium",
    "Einsteinium",
    "Fermium",
    "Mendelevium",
    "Nobelium",
    "Lawrencium",
    "Rutherfordium",
    "Dubnium",
    "Seaborgium",
    "Bohrium",
    "Hassium",
    "Meitnerium",
    "Darmstadtium ",
    "Roentgenium ",
    "Copernicium ",
    "Nihonium",
    "Flerovium",
    "Moscovium",
    "Livermorium",
    "Tennessine",
    "Oganesson",
];

/// Hold all atomic radii for a single element. So that in the code it is obvious which radius you use. All values are in Å (10e-10 m or 0.1 nm).
#[derive(Debug)]
pub struct AtomicRadius {
    /// Gets the atomic radius (defined up until 'Cm' 96) in Å.
    /// Source: Martin Rahm, Roald Hoffmann, and N. W. Ashcroft. Atomic and Ionic Radii of Elements 1-96. Chemistry - A European Journal, 22(41):14625–14632, oct 2016. <http://doi.org/10.1002/chem.201602949>.
    /// Updated to the corrigendum: <https://doi.org/10.1002/chem.201700610>
    pub unbound: Option<f64>,
    /// Gets the van der Waals radius (defined up until 'Es' 99 excluding 62, 84-88) in Å.
    /// Source: Alvarez, S. (2013). A cartography of the van der Waals territories. Dalton Transactions, 42(24), 8617. <https://doi.org/10.1039/c3dt50599e>
    pub van_der_waals: Option<f64>,
    /// Gets the single covalently bonded atom radius (defined for all elements (<=118)) in Å.
    /// Source: P. Pyykkö; M. Atsumi (2009). "Molecular Single-Bond Covalent Radii for Elements 1-118". Chemistry: A European Journal. 15 (1): 186–197. <http://doi.org/10.1002/chem.200800987>
    pub covalent_single: f64,
    /// Gets the double covalently bonded atom radius (defined for the elements 3-112) in Å.
    /// Source: P. Pyykkö; M. Atsumi (2009). "Molecular Double-Bond Covalent Radii for Elements Li–E112". Chemistry: A European Journal. 15 (46): 12770–12779. <http://doi.org/10.1002/chem.200901472>
    pub covalent_double: Option<f64>,
    /// Gets the double covalently bonded atom radius (defined for all applicable elements (<=112)) in Å.
    /// Source: P. Pyykkö; S. Riedel; M. Patzschke (2005). "Triple-Bond Covalent Radii". Chemistry: A European Journal. 11 (12): 3511–3520. <http://doi.org/10.1002/chem.200401299>
    pub covalent_triple: Option<f64>,
}

impl AtomicRadius {
    const fn new(
        unbound: Option<f64>,
        van_der_waals: Option<f64>,
        covalent_single: f64,
        covalent_double: Option<f64>,
        covalent_triple: Option<f64>,
    ) -> Self {
        Self {
            unbound,
            van_der_waals,
            covalent_single,
            covalent_double,
            covalent_triple,
        }
    }
}

/// All atomic radii for all atoms
const ELEMENT_ATOMIC_RADII: [AtomicRadius; 118] = [
    AtomicRadius::new(Some(1.54), Some(1.2), 0.32, None, None),
    AtomicRadius::new(Some(1.34), Some(1.43), 0.46, None, None),
    AtomicRadius::new(Some(2.20), Some(2.12), 1.33, Some(1.24), None),
    AtomicRadius::new(Some(2.19), Some(1.98), 1.02, Some(0.90), Some(0.85)),
    AtomicRadius::new(Some(2.05), Some(1.91), 0.85, Some(0.78), Some(0.73)),
    AtomicRadius::new(Some(1.90), Some(1.77), 0.75, Some(0.67), Some(0.60)),
    AtomicRadius::new(Some(1.79), Some(1.66), 0.71, Some(0.60), Some(0.54)),
    AtomicRadius::new(Some(1.71), Some(1.5), 0.63, Some(0.57), Some(0.53)),
    AtomicRadius::new(Some(1.63), Some(1.46), 0.64, Some(0.59), Some(0.53)),
    AtomicRadius::new(Some(1.56), Some(1.58), 0.67, Some(0.96), None),
    AtomicRadius::new(Some(2.25), Some(2.5), 1.55, Some(1.60), None),
    AtomicRadius::new(Some(2.40), Some(2.51), 1.39, Some(1.32), Some(1.27)),
    AtomicRadius::new(Some(2.39), Some(2.25), 1.26, Some(1.13), Some(1.11)),
    AtomicRadius::new(Some(2.32), Some(2.19), 1.16, Some(1.07), Some(1.02)),
    AtomicRadius::new(Some(2.23), Some(1.9), 1.11, Some(1.02), Some(0.94)),
    AtomicRadius::new(Some(2.14), Some(1.89), 1.03, Some(0.94), Some(0.95)),
    AtomicRadius::new(Some(2.06), Some(1.82), 0.99, Some(0.95), Some(0.93)),
    AtomicRadius::new(Some(1.97), Some(1.83), 0.96, Some(1.07), Some(0.96)),
    AtomicRadius::new(Some(2.34), Some(2.73), 1.96, Some(1.93), None),
    AtomicRadius::new(Some(2.70), Some(2.62), 1.71, Some(1.47), Some(1.33)),
    AtomicRadius::new(Some(2.63), Some(2.58), 1.48, Some(1.16), Some(1.14)),
    AtomicRadius::new(Some(2.57), Some(2.46), 1.36, Some(1.17), Some(1.08)),
    AtomicRadius::new(Some(2.52), Some(2.42), 1.34, Some(1.12), Some(1.06)),
    AtomicRadius::new(Some(2.33), Some(2.45), 1.22, Some(1.11), Some(1.03)),
    AtomicRadius::new(Some(2.42), Some(2.45), 1.19, Some(1.05), Some(1.03)),
    AtomicRadius::new(Some(2.37), Some(2.44), 1.16, Some(1.09), Some(1.02)),
    AtomicRadius::new(Some(2.33), Some(2.4), 1.11, Some(1.03), Some(9.6)),
    AtomicRadius::new(Some(2.29), Some(2.4), 1.10, Some(1.01), Some(1.01)),
    AtomicRadius::new(Some(2.17), Some(2.38), 1.12, Some(1.15), Some(1.20)),
    AtomicRadius::new(Some(2.22), Some(2.39), 1.18, Some(1.20), None),
    AtomicRadius::new(Some(2.33), Some(2.32), 1.24, Some(1.17), Some(1.21)),
    AtomicRadius::new(Some(2.34), Some(2.29), 1.21, Some(1.11), Some(1.14)),
    AtomicRadius::new(Some(2.31), Some(1.88), 1.21, Some(1.14), Some(1.06)),
    AtomicRadius::new(Some(2.24), Some(1.82), 1.16, Some(1.07), Some(1.07)),
    AtomicRadius::new(Some(2.19), Some(1.86), 1.14, Some(1.09), Some(1.10)),
    AtomicRadius::new(Some(2.12), Some(2.25), 1.17, Some(1.21), Some(1.08)),
    AtomicRadius::new(Some(2.40), Some(3.21), 2.10, Some(2.02), None),
    AtomicRadius::new(Some(2.79), Some(2.84), 1.85, Some(1.57), Some(1.39)),
    AtomicRadius::new(Some(2.74), Some(2.75), 1.63, Some(1.30), Some(1.24)),
    AtomicRadius::new(Some(2.69), Some(2.52), 1.54, Some(1.27), Some(1.21)),
    AtomicRadius::new(Some(2.51), Some(2.56), 1.47, Some(1.25), Some(1.16)),
    AtomicRadius::new(Some(2.44), Some(2.45), 1.38, Some(1.21), Some(1.13)),
    AtomicRadius::new(Some(2.52), Some(2.44), 1.28, Some(1.20), Some(1.10)),
    AtomicRadius::new(Some(2.37), Some(2.46), 1.25, Some(1.14), Some(1.03)),
    AtomicRadius::new(Some(2.33), Some(2.44), 1.25, Some(1.10), Some(1.06)),
    AtomicRadius::new(Some(2.15), Some(2.15), 1.20, Some(1.17), Some(1.12)),
    AtomicRadius::new(Some(2.25), Some(2.53), 1.28, Some(1.39), Some(1.37)),
    AtomicRadius::new(Some(2.38), Some(2.49), 1.36, Some(1.44), None),
    AtomicRadius::new(Some(2.46), Some(2.43), 1.42, Some(1.36), Some(1.46)),
    AtomicRadius::new(Some(2.48), Some(2.42), 1.40, Some(1.30), Some(1.32)),
    AtomicRadius::new(Some(2.46), Some(2.47), 1.40, Some(1.33), Some(1.27)),
    AtomicRadius::new(Some(2.42), Some(1.99), 1.36, Some(1.28), Some(1.21)),
    AtomicRadius::new(Some(2.38), Some(2.04), 1.33, Some(1.29), Some(1.25)),
    AtomicRadius::new(Some(2.32), Some(2.06), 1.31, Some(1.35), Some(1.22)),
    AtomicRadius::new(Some(2.49), Some(3.48), 2.32, Some(2.09), None),
    AtomicRadius::new(Some(2.93), Some(3.03), 1.96, Some(1.61), Some(1.49)),
    AtomicRadius::new(Some(2.84), Some(2.98), 1.80, Some(1.39), Some(1.39)),
    AtomicRadius::new(Some(2.82), Some(2.88), 1.63, Some(1.37), Some(1.31)),
    AtomicRadius::new(Some(2.86), Some(2.92), 1.76, Some(1.38), Some(1.28)),
    AtomicRadius::new(Some(2.84), Some(2.95), 1.74, Some(1.37), None),
    AtomicRadius::new(Some(2.83), None, 1.73, Some(1.35), None),
    AtomicRadius::new(Some(2.80), Some(2.9), 1.72, Some(1.34), None),
    AtomicRadius::new(Some(2.80), Some(2.87), 1.68, Some(1.34), None),
    AtomicRadius::new(Some(2.77), Some(2.83), 1.69, Some(1.35), Some(1.32)),
    AtomicRadius::new(Some(2.76), Some(2.79), 1.68, Some(1.35), None),
    AtomicRadius::new(Some(2.75), Some(2.87), 1.67, Some(1.33), None),
    AtomicRadius::new(Some(2.73), Some(2.81), 1.66, Some(1.33), None),
    AtomicRadius::new(Some(2.72), Some(2.83), 1.65, Some(1.33), None),
    AtomicRadius::new(Some(2.71), Some(2.79), 1.64, Some(1.31), None),
    AtomicRadius::new(Some(2.77), Some(2.8), 1.70, Some(1.29), None),
    AtomicRadius::new(Some(2.70), Some(2.74), 1.62, Some(1.31), Some(1.31)),
    AtomicRadius::new(Some(2.64), Some(2.63), 1.52, Some(1.28), Some(1.22)),
    AtomicRadius::new(Some(2.58), Some(2.53), 1.46, Some(1.26), Some(1.19)),
    AtomicRadius::new(Some(2.53), Some(2.57), 1.37, Some(1.20), Some(1.15)),
    AtomicRadius::new(Some(2.49), Some(2.49), 1.31, Some(1.19), Some(1.10)),
    AtomicRadius::new(Some(2.44), Some(2.48), 1.29, Some(1.16), Some(1.09)),
    AtomicRadius::new(Some(2.40), Some(2.41), 1.22, Some(1.15), Some(1.07)),
    AtomicRadius::new(Some(2.30), Some(2.29), 1.23, Some(1.12), Some(1.10)),
    AtomicRadius::new(Some(2.26), Some(2.32), 1.24, Some(1.21), Some(1.23)),
    AtomicRadius::new(Some(2.29), Some(2.45), 1.33, Some(1.42), None),
    AtomicRadius::new(Some(2.42), Some(2.47), 1.44, Some(1.42), Some(1.50)),
    AtomicRadius::new(Some(2.49), Some(2.6), 1.44, Some(1.35), Some(1.37)),
    AtomicRadius::new(Some(2.50), Some(2.54), 1.51, Some(1.41), Some(1.35)),
    AtomicRadius::new(Some(2.50), None, 1.45, Some(1.35), Some(1.29)),
    AtomicRadius::new(Some(2.47), None, 1.47, Some(1.38), Some(1.38)),
    AtomicRadius::new(Some(2.43), None, 1.42, Some(1.45), Some(1.33)),
    AtomicRadius::new(Some(2.58), None, 2.23, Some(2.18), None),
    AtomicRadius::new(Some(2.92), None, 2.01, Some(1.73), Some(1.59)),
    AtomicRadius::new(Some(2.93), Some(2.8), 1.86, Some(1.53), Some(1.40)),
    AtomicRadius::new(Some(2.88), Some(2.93), 1.75, Some(1.43), Some(1.36)),
    AtomicRadius::new(Some(2.85), Some(2.88), 1.69, Some(1.38), Some(1.29)),
    AtomicRadius::new(Some(2.83), Some(2.71), 1.70, Some(1.34), Some(1.18)),
    AtomicRadius::new(Some(2.81), Some(2.82), 1.71, Some(1.36), Some(1.16)),
    AtomicRadius::new(Some(2.78), Some(2.81), 1.72, Some(1.35), None),
    AtomicRadius::new(Some(2.76), Some(2.83), 1.66, Some(1.35), None),
    AtomicRadius::new(Some(2.64), Some(3.05), 1.66, Some(1.36), None),
    AtomicRadius::new(None, Some(3.4), 1.68, Some(1.39), None),
    AtomicRadius::new(None, Some(3.05), 1.68, Some(1.40), None),
    AtomicRadius::new(None, Some(2.7), 1.65, Some(1.40), None),
    AtomicRadius::new(None, None, 1.67, Some(1.67), Some(1.67)),
    AtomicRadius::new(None, None, 1.73, Some(1.39), None),
    AtomicRadius::new(None, None, 1.76, Some(1.76), Some(1.76)),
    AtomicRadius::new(None, None, 1.61, Some(1.41), None),
    AtomicRadius::new(None, None, 1.57, Some(1.40), Some(1.31)),
    AtomicRadius::new(None, None, 1.49, Some(1.36), Some(1.26)),
    AtomicRadius::new(None, None, 1.43, Some(1.28), Some(1.21)),
    AtomicRadius::new(None, None, 1.41, Some(1.28), Some(1.19)),
    AtomicRadius::new(None, None, 1.34, Some(1.25), Some(1.18)),
    AtomicRadius::new(None, None, 1.29, Some(1.25), Some(1.13)),
    AtomicRadius::new(None, None, 1.28, Some(1.16), Some(1.12)),
    AtomicRadius::new(None, None, 1.21, Some(1.16), Some(1.18)),
    AtomicRadius::new(None, None, 1.22, Some(1.37), Some(1.30)),
    AtomicRadius::new(None, None, 1.36, None, None),
    AtomicRadius::new(None, None, 1.43, None, None),
    AtomicRadius::new(None, None, 1.62, None, None),
    AtomicRadius::new(None, None, 1.75, None, None),
    AtomicRadius::new(None, None, 1.65, None, None),
    AtomicRadius::new(None, None, 1.57, None, None),
];

const ELEMENT_WEIGHT: [Option<f64>; 118] = [
    Some(1.007975),
    Some(4.002602),
    Some(6.9675),
    Some(9.0121831),
    Some(10.8135),
    Some(12.0106),
    Some(14.006855),
    Some(15.9994),
    Some(18.998403163),
    Some(20.1797),
    Some(22.98976928),
    Some(24.3055),
    Some(26.9815384),
    Some(28.085),
    Some(30.973761998),
    Some(32.0675),
    Some(35.4515),
    Some(39.8775),
    Some(39.0983),
    Some(40.078),
    Some(44.955908),
    Some(47.867),
    Some(50.9415),
    Some(51.9961),
    Some(54.938043),
    Some(55.845),
    Some(58.933194),
    Some(58.6934),
    Some(63.546),
    Some(65.38),
    Some(69.723),
    Some(72.630),
    Some(74.921595),
    Some(78.971),
    Some(79.904),
    Some(83.798),
    Some(85.4678),
    Some(87.62),
    Some(88.90584),
    Some(91.224),
    Some(92.90637),
    Some(95.95),
    None,
    Some(101.07),
    Some(102.90549),
    Some(106.42),
    Some(107.8682),
    Some(112.414),
    Some(114.818),
    Some(118.710),
    Some(121.760),
    Some(127.60),
    Some(126.90447),
    Some(131.293),
    Some(132.90545196),
    Some(137.327),
    Some(138.90547),
    Some(140.116),
    Some(140.90766),
    Some(144.242),
    None,
    Some(150.36),
    Some(151.964),
    Some(157.25),
    Some(158.925354),
    Some(162.500),
    Some(164.930328),
    Some(167.259),
    Some(168.934218),
    Some(173.045),
    Some(174.9668),
    Some(178.486),
    Some(180.94788),
    Some(183.84),
    Some(186.207),
    Some(190.23),
    Some(192.217),
    Some(195.084),
    Some(196.966570),
    Some(200.592),
    Some(204.3835),
    Some(207.04),
    Some(208.98040),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(232.0377),
    Some(231.03588),
    Some(238.02891),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

const ELEMENT_ELECTRON_NEGATIVITY: [Option<f64>; 118] = [
    Some(2.20),
    None,
    Some(0.98),
    Some(1.57),
    Some(2.04),
    Some(2.55),
    Some(3.04),
    Some(3.44),
    Some(3.98),
    None,
    Some(0.93),
    Some(1.31),
    Some(1.61),
    Some(1.90),
    Some(2.19),
    Some(2.58),
    Some(3.16),
    None,
    Some(0.82),
    Some(1.00),
    Some(1.36),
    Some(1.54),
    Some(1.63),
    Some(1.66),
    Some(1.55),
    Some(1.83),
    Some(1.88),
    Some(1.91),
    Some(1.90),
    Some(1.65),
    Some(1.81),
    Some(2.01),
    Some(2.18),
    Some(2.55),
    Some(2.96),
    None,
    Some(0.82),
    Some(0.95),
    Some(1.22),
    Some(1.33),
    Some(1.6),
    Some(2.16),
    Some(2.10),
    Some(2.2),
    Some(2.28),
    Some(2.20),
    Some(1.93),
    Some(1.69),
    Some(1.78),
    Some(1.96),
    Some(2.05),
    Some(2.1),
    Some(2.66),
    Some(2.60),
    Some(0.79),
    Some(0.89),
    Some(1.10),
    Some(1.12),
    Some(1.13),
    Some(1.14),
    None,
    Some(1.17),
    None,
    Some(1.20),
    None,
    Some(1.22),
    Some(1.23),
    Some(1.24),
    Some(1.25),
    None,
    Some(1.0),
    Some(1.3),
    Some(1.5),
    Some(1.7),
    Some(1.9),
    Some(2.2),
    Some(2.2),
    Some(2.2),
    Some(2.4),
    Some(1.9),
    Some(1.8),
    Some(1.8),
    Some(1.9),
    Some(2.0),
    Some(2.2),
    None,
    Some(0.7),
    Some(0.9),
    Some(1.1),
    Some(1.3),
    Some(1.5),
    Some(1.7),
    Some(1.3),
    Some(1.3),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
