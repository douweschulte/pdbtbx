use super::*;
use std::ops;

// WIP: the API is very neat, but this way of searching does not work, it loses it progress while traversing the tree
// eg ConformerName "VAL" | AtomElement "C" cannot right now track if the first part is true or not when validating the second part
// it should fill in all values with constants for all hierarchies until it can prove/disprove the hierarchy

/// The trait representing a (set of) parameter(s) to be used in the search for atom(s) in a PDB
pub trait SearchPDB: Clone {
    /// Determine if this model matches the search
    fn optional_matches_model(&mut self, model: &Model) -> Option<bool>;
    /// Determine if this chain matches the search
    fn optional_matches_chain(&mut self, chain: &Chain) -> Option<bool>;
    /// Determine if this residue matches the search
    fn optional_matches_residue(&mut self, residue: &Residue) -> Option<bool>;
    /// Determine if this conformer matches the search
    fn optional_matches_conformer(&mut self, conformer: &Conformer) -> Option<bool>;
    /// Determine if this atom matches the search
    fn optional_matches_atom(&mut self, atom: &Atom) -> bool;
    /// Try if the search is done, return Some(result) if this is the case, otherwise returns None
    fn finished(&self) -> Option<bool>;
}

/// Any parameter to use in a search for atom(s) in a PDB
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Term {
    /// The model serial number, only used in (NMR) PDB files with multiple states of a protein
    ModelSerialNumber(usize),
    /// The chain id eg 'A'
    ChainId(String),
    /// The serial number is known, but the insertion code is not.
    ResidueSerialNumber(isize),
    /// The insertion code is known, but the serial number is not.
    ResidueInsertionCode(Option<String>),
    /// The serial number and insertion code are known.
    ResidueId(isize, Option<String>),
    /// The name is known, but not the alternative location.
    ConformerName(String),
    /// The alternative location is known, but not the name.
    ConformerAlternativeLocation(Option<String>),
    /// The name and alternative location are known.
    ConformerId(String, Option<String>),
    /// The serial number is known.
    AtomSerialNumber(usize),
    /// Select a range of atoms starting at the first number and ending with the last number inclusive.
    AtomSerialNumberRange(usize, usize),
    /// The name is known.
    AtomName(String),
    /// The element is known.
    AtomElement(String),
}

impl Term {
    fn optional_matches_model(&self, model: &Model) -> Option<bool> {
        match self {
            Term::ModelSerialNumber(s) => Some(*s == model.serial_number()),
            _ => None,
        }
    }
    fn optional_matches_chain(&self, chain: &Chain) -> Option<bool> {
        match self {
            Term::ChainId(s) => Some(s == chain.id()),
            _ => None,
        }
    }
    fn optional_matches_residue(&self, residue: &Residue) -> Option<bool> {
        match self {
            Term::ResidueSerialNumber(s) => Some(*s == residue.serial_number()),
            Term::ResidueInsertionCode(ic) => Some(ic.as_deref() == residue.insertion_code()),
            Term::ResidueId(s, ic) => Some((*s, ic.as_deref()) == residue.id()),
            _ => None,
        }
    }
    fn optional_matches_conformer(&self, conformer: &Conformer) -> Option<bool> {
        match self {
            Term::ConformerName(n) => Some(n == conformer.name()),
            Term::ConformerAlternativeLocation(al) => {
                Some(al.as_deref() == conformer.alternative_location())
            }
            Term::ConformerId(n, al) => Some((n.as_str(), al.as_deref()) == conformer.id()),
            _ => None,
        }
    }
    fn optional_matches_atom(&self, atom: &Atom) -> Option<bool> {
        match self {
            Term::AtomSerialNumber(n) => Some(atom.serial_number() == *n),
            Term::AtomSerialNumberRange(low, high) => {
                Some(atom.serial_number() >= *low && atom.serial_number() <= *high)
            }
            Term::AtomName(n) => Some(atom.name() == n),
            Term::AtomElement(e) => Some(atom.element() == e),
            _ => None,
        }
    }
}

/// A collection of multiple search terms in the search for (an) atom(s) in a PDB
#[derive(Debug, Clone)]
pub enum Search {
    /// A search with operators, &, |, or ^
    Ops(Ops, Box<Search>, Box<Search>),
    /// !A (not)
    Not(Box<Search>),
    /// A (single search term)
    Single(Term),
    /// Known value
    Known(bool),
}

/// All operators that can be used in a search
#[derive(Debug, Clone, Copy)]
pub enum Ops {
    /// Binary and `&`
    And,
    /// Binary or `|`
    Or,
    /// Binary xor `^`
    Xor,
}

impl Search {
    fn simplify(self) -> Search {
        match self {
            Search::Ops(ops, a, b) => match (ops, a.simplify(), b.simplify()) {
                (Ops::And, Search::Known(false), _) => Search::Known(false),
                (Ops::And, Search::Known(a), Search::Known(b)) => Search::Known(a & b),
                (Ops::Or, Search::Known(true), _) => Search::Known(true),
                (Ops::Or, Search::Known(a), Search::Known(b)) => Search::Known(a | b),
                (Ops::Xor, Search::Known(a), Search::Known(b)) => Search::Known(a ^ b),
                (ops, a, b) => Search::Ops(ops, Box::new(a), Box::new(b)),
            },
            Search::Not(a) => match a.simplify() {
                Search::Known(a) => Search::Known(!a),
                a => Search::Not(Box::new(a)),
            },
            _ => self,
        }
    }

    /// Check if the search is done.
    pub fn complete(&self) -> Option<bool> {
        match self {
            Search::Known(a) => Some(*a),
            _ => None,
        }
    }

    /// Add information about the model into the search, returns a new search with the information integrated
    pub fn add_model_info(&self, model: &Model) -> Search {
        match self {
            Search::Ops(ops, a, b) => Search::Ops(
                *ops,
                Box::new(a.add_model_info(model)),
                Box::new(b.add_model_info(model)),
            ),
            Search::Not(a) => Search::Not(Box::new(a.add_model_info(model))),
            Search::Single(a) => match a.optional_matches_model(model) {
                Some(true) => Search::Known(true),
                Some(false) => Search::Known(false),
                None => self.clone(),
            },
            _ => self.clone(),
        }
        .simplify()
    }

    /// Add information about the chain into the search, returns a new search with the information integrated
    pub fn add_chain_info(&self, chain: &Chain) -> Search {
        match self {
            Search::Ops(ops, a, b) => Search::Ops(
                *ops,
                Box::new(a.add_chain_info(chain)),
                Box::new(b.add_chain_info(chain)),
            ),
            Search::Not(a) => Search::Not(Box::new(a.add_chain_info(chain))),
            Search::Single(a) => match a.optional_matches_chain(chain) {
                Some(true) => Search::Known(true),
                Some(false) => Search::Known(false),
                None => self.clone(),
            },
            _ => self.clone(),
        }
        .simplify()
    }

    /// Add information about the residue into the search, returns a new search with the information integrated
    pub fn add_residue_info(&self, residue: &Residue) -> Search {
        match self {
            Search::Ops(ops, a, b) => Search::Ops(
                *ops,
                Box::new(a.add_residue_info(residue)),
                Box::new(b.add_residue_info(residue)),
            ),
            Search::Not(a) => Search::Not(Box::new(a.add_residue_info(residue))),
            Search::Single(a) => match a.optional_matches_residue(residue) {
                Some(true) => Search::Known(true),
                Some(false) => Search::Known(false),
                None => self.clone(),
            },
            _ => self.clone(),
        }
        .simplify()
    }

    /// Add information about the conformer into the search, returns a new search with the information integrated
    pub fn add_conformer_info(&self, conformer: &Conformer) -> Search {
        match self {
            Search::Ops(ops, a, b) => Search::Ops(
                *ops,
                Box::new(a.add_conformer_info(conformer)),
                Box::new(b.add_conformer_info(conformer)),
            ),
            Search::Not(a) => Search::Not(Box::new(a.add_conformer_info(conformer))),
            Search::Single(a) => match a.optional_matches_conformer(conformer) {
                Some(true) => Search::Known(true),
                Some(false) => Search::Known(false),
                None => self.clone(),
            },
            _ => self.clone(),
        }
        .simplify()
    }

    /// Add information about the atom into the search, returns a new search with the information integrated
    pub fn add_atom_info(&self, atom: &Atom) -> Search {
        match self {
            Search::Ops(ops, a, b) => Search::Ops(
                *ops,
                Box::new(a.add_atom_info(atom)),
                Box::new(b.add_atom_info(atom)),
            ),
            Search::Not(a) => Search::Not(Box::new(a.add_atom_info(atom))),
            Search::Single(a) => match a.optional_matches_atom(atom) {
                Some(true) => Search::Known(true),
                Some(false) => Search::Known(false),
                None => self.clone(),
            },
            _ => self.clone(),
        }
        .simplify()
    }
}

impl ops::BitAnd<Term> for Term {
    type Output = Search;
    fn bitand(self, rhs: Self) -> Self::Output {
        Search::Ops(
            Ops::And,
            Box::new(Search::Single(self)),
            Box::new(Search::Single(rhs)),
        )
    }
}

impl ops::BitOr<Term> for Term {
    type Output = Search;
    fn bitor(self, rhs: Self) -> Self::Output {
        Search::Ops(
            Ops::Or,
            Box::new(Search::Single(self)),
            Box::new(Search::Single(rhs)),
        )
    }
}

impl ops::BitXor<Term> for Term {
    type Output = Search;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Search::Ops(
            Ops::Xor,
            Box::new(Search::Single(self)),
            Box::new(Search::Single(rhs)),
        )
    }
}

impl ops::Not for Term {
    type Output = Search;
    fn not(self) -> Self::Output {
        Search::Not(Box::new(Search::Single(self)))
    }
}

impl ops::BitAnd<Search> for Term {
    type Output = Search;
    fn bitand(self, rhs: Search) -> Self::Output {
        Search::Ops(Ops::And, Box::new(Search::Single(self)), Box::new(rhs))
    }
}

impl ops::BitOr<Search> for Term {
    type Output = Search;
    fn bitor(self, rhs: Search) -> Self::Output {
        Search::Ops(Ops::Or, Box::new(Search::Single(self)), Box::new(rhs))
    }
}

impl ops::BitXor<Search> for Term {
    type Output = Search;
    fn bitxor(self, rhs: Search) -> Self::Output {
        Search::Ops(Ops::Xor, Box::new(Search::Single(self)), Box::new(rhs))
    }
}

impl ops::BitAnd<Search> for Search {
    type Output = Search;
    fn bitand(self, rhs: Self) -> Self::Output {
        Search::Ops(Ops::And, Box::new(self), Box::new(rhs))
    }
}

impl ops::BitOr<Search> for Search {
    type Output = Search;
    fn bitor(self, rhs: Self) -> Self::Output {
        Search::Ops(Ops::Or, Box::new(self), Box::new(rhs))
    }
}

impl ops::BitXor<Search> for Search {
    type Output = Search;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Search::Ops(Ops::Xor, Box::new(self), Box::new(rhs))
    }
}

impl ops::Not for Search {
    type Output = Search;
    fn not(self) -> Self::Output {
        Search::Not(Box::new(self))
    }
}

impl ops::BitAnd<Term> for Search {
    type Output = Search;
    fn bitand(self, rhs: Term) -> Self::Output {
        Search::Ops(Ops::And, Box::new(self), Box::new(Search::Single(rhs)))
    }
}

impl ops::BitOr<Term> for Search {
    type Output = Search;
    fn bitor(self, rhs: Term) -> Self::Output {
        Search::Ops(Ops::Or, Box::new(self), Box::new(Search::Single(rhs)))
    }
}

impl ops::BitXor<Term> for Search {
    type Output = Search;
    fn bitxor(self, rhs: Term) -> Self::Output {
        Search::Ops(Ops::Xor, Box::new(self), Box::new(Search::Single(rhs)))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn atom_find() {
        let a = Atom::new(true, 123, "CA", 0.0, 0.0, 0.0, 0.0, 0.0, "C", 1).unwrap();
        assert_eq!(
            Search::Single(Term::AtomSerialNumber(123))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            Search::Single(Term::AtomName("CA".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            Search::Single(Term::AtomElement("C".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            Search::Single(Term::AtomSerialNumberRange(120, 130))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
    }

    #[test]
    fn search_combinations() {
        let a = Atom::new(true, 123, "CA", 0.0, 0.0, 0.0, 0.0, 0.0, "C", 1).unwrap();
        assert_eq!(
            (Term::AtomName("CA".to_string()) & Term::AtomElement("C".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            (Term::AtomName("CA".to_string()) | Term::AtomElement("E".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            (!Term::AtomName("CA".to_string()) ^ Term::AtomElement("C".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            (!Term::AtomName("BA".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
    }
}
