use super::*;

/// All possible nuggets of information regarding models for selecting a(n) atom(s) in a PDB file.
#[derive(Debug, Clone)]
pub enum FindModel {
    /// No information available.
    NoInfo,
    /// The serial number is known.
    SerialNumber(usize),
}

impl FindModel {
    /// Find if the information about the model of the atom(s) matches this model.
    pub fn matches(&self, model: &Model) -> bool {
        match self {
            FindModel::NoInfo => true,
            FindModel::SerialNumber(n) => model.serial_number() == *n,
        }
    }
}

/// All possible nuggets of information regarding chains for selecting a(n) atom(s) in a PDB file.
#[derive(Debug, Clone)]
pub enum FindChain {
    /// No information available.
    NoInfo,
    /// The ID is known.
    ID(String),
}

impl FindChain {
    /// Find if the information about the chain of the atom(s) matches this chain.
    pub fn matches(&self, chain: &Chain) -> bool {
        match self {
            FindChain::NoInfo => true,
            FindChain::ID(n) => chain.id() == *n,
        }
    }
}

/// All possible nuggets of information regarding residues for selecting a(n) atom(s) in a PDB file.
#[derive(Debug, Clone)]
pub enum FindResidue {
    /// No information available.
    NoInfo,
    /// The serial number is known, but the insertion code is not.
    SerialNumber(isize),
    /// The insertion code is known, but the serial number is not.
    InsertionCode(Option<String>),
    /// The serial number and insertion code are known.
    ID(isize, Option<String>),
}

impl FindResidue {
    /// Find if the information about the residue of the atom(s) matches this residue.
    pub fn matches(&self, residue: &Residue) -> bool {
        match self {
            FindResidue::NoInfo => true,
            FindResidue::SerialNumber(n) => residue.serial_number() == *n,
            FindResidue::InsertionCode(a) => {
                residue.insertion_code() == a.as_ref().map(|s| s.as_str())
            }
            FindResidue::ID(n, a) => residue.id() == (*n, a.as_ref().map(|s| s.as_str())),
        }
    }
}

/// All possible nuggets of information regarding conformers for selecting a(n) atom(s) in a PDB file.
#[derive(Debug, Clone)]
pub enum FindConformer {
    /// No information available.
    NoInfo,
    /// The name is known, but not the alternative location.
    Name(String),
    /// The alternative location is known, but not the name.
    AlternativeLocation(Option<String>),
    /// The name and alternative location are known.
    ID(String, Option<String>),
}

impl FindConformer {
    /// Find if the information about the conformer of the atom(s) matches this conformer.
    pub fn matches(&self, conformer: &Conformer) -> bool {
        match self {
            FindConformer::NoInfo => true,
            FindConformer::Name(n) => conformer.name() == n,
            FindConformer::AlternativeLocation(a) => {
                conformer.alternative_location() == a.as_ref().map(|s| s.as_str())
            }
            FindConformer::ID(n, a) => conformer.id() == (n, a.as_ref().map(|s| s.as_str())),
        }
    }
}

/// All possible nuggets of information regarding atoms for selecting a(n) atom(s) in a PDB file.
#[derive(Debug, Clone)]
pub enum FindAtom {
    /// No information available.
    NoInfo,
    /// The serial number is known.
    SerialNumber(usize),
    /// The name is known.
    Name(String),
    /// The element is known.
    Element(String),
}

impl FindAtom {
    /// Find if the information about the atom of the atom(s) matches this atom.
    pub fn matches(&self, atom: &Atom) -> bool {
        match self {
            FindAtom::NoInfo => true,
            FindAtom::SerialNumber(n) => atom.serial_number() == *n,
            FindAtom::Name(n) => atom.name() == n,
            FindAtom::Element(e) => atom.element() == e,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn atom_find() {
        let a = Atom::new(true, 123, "CA", 0.0, 0.0, 0.0, 0.0, 0.0, "C", 1).unwrap();
        assert!(FindAtom::NoInfo.matches(&a));
        assert!(FindAtom::Name("CA".to_owned()).matches(&a));
        assert!(FindAtom::SerialNumber(123).matches(&a));
        assert!(FindAtom::Element("C".to_owned()).matches(&a));
        assert!(!FindAtom::Name("CB".to_owned()).matches(&a));
        assert!(!FindAtom::SerialNumber(23).matches(&a));
        assert!(!FindAtom::Element("F".to_owned()).matches(&a));
    }
}
