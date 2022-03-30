use super::*;
use std::ops;

/// Any parameter to use in a [Search] for atom(s) in a PDB.
/// For position related searches look into the [rstar] crate which can be combined
/// with this crate using the `rstar` feature, see [`PDB::create_atom_rtree`] and
/// [`PDB::create_hierarchy_rtree`]. The rstar crate makes spatial lookup and queries
/// way faster and feasible to use in high performance environments.
#[allow(unused)]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Term {
    /// The model serial number, only used in (NMR) PDB files with multiple states of a protein, see [Model::serial_number].
    ModelSerialNumber(usize),
    /// Search for a range of model serial numbers, starting at the first number and ending with the last number inclusive.
    ModelSerialNumberRange(usize, usize),
    /// The chain id eg `A`, see [Chain::id].
    ChainId(String),
    /// Search for a range of chain ids, using the Ord implementation of std::str <https://doc.rust-lang.org/std/primitive.str.html#impl-Ord>, starting at the first number and ending with the last number inclusive.
    ChainIdRange(String, String),
    /// The residue serial number, see [Residue::serial_number].
    ResidueSerialNumber(isize),
    /// Search for a range of residue serial numbers, starting at the first number and ending with the last number inclusive.
    ResidueSerialNumberRange(isize, isize),
    /// The residue insertion code eg `Some("A")`, see [Residue::insertion_code].
    ResidueInsertionCode(Option<String>),
    /// The residue serial number and insertion code combined, see [Residue::id].
    ResidueId(isize, Option<String>),
    /// The conformer name eg `ALA`, see [Conformer::name].
    ConformerName(String),
    /// The conformer alternative location eg `Some("A")`, see [Conformer::alternative_location].
    ConformerAlternativeLocation(Option<String>),
    /// The conformer name and alternative location combined, see [Conformer::id].
    ConformerId(String, Option<String>),
    /// The atom serial number, see [Atom::serial_number].
    AtomSerialNumber(usize),
    /// A range of atoms based on serial number starting at the first number and ending with the last number inclusive.
    AtomSerialNumberRange(usize, usize),
    /// The atom name eg `CA`, see [Atom::name].
    AtomName(String),
    /// The element eq `C`, see [Atom::element].
    Element(String),
    /// Atom b factor, see [Atom::b_factor].
    BFactor(f64),
    /// Atom B factor range starting at the first number and ending with the last number inclusive.
    BFactorRange(f64, f64),
    /// Atom occupancy, see [Atom::occupancy].
    Occupancy(f64),
    /// Atom occupancy range starting at the first number and ending with the last number inclusive.
    OccupancyRange(f64, f64),
    /// Search for backbone atoms, this means that [Conformer::is_amino_acid] is `true` and [Atom::is_backbone] is `true`.
    Backbone,
    /// Search for side chain atoms, this means that [Conformer::is_amino_acid] is `true` and [Atom::is_backbone] is `false`.
    SideChain,
    /// Search for hetero atoms, this means that [Atom::hetero] is `true`.
    Hetero,
}

impl Term {
    fn optional_matches_model(&self, model: &Model) -> Option<bool> {
        match self {
            Term::ModelSerialNumber(s) => Some(*s == model.serial_number()),
            Term::ModelSerialNumberRange(low, high) => {
                Some(*low <= model.serial_number() && *high >= model.serial_number())
            }
            _ => None,
        }
    }
    fn optional_matches_chain(&self, chain: &Chain) -> Option<bool> {
        match self {
            Term::ChainId(s) => Some(s == chain.id()),
            Term::ChainIdRange(low, high) => {
                Some(low.as_str() <= chain.id() && high.as_str() >= chain.id())
            }
            _ => None,
        }
    }
    fn optional_matches_residue(&self, residue: &Residue) -> Option<bool> {
        match self {
            Term::ResidueSerialNumber(s) => Some(*s == residue.serial_number()),
            Term::ResidueSerialNumberRange(low, high) => {
                Some(*low <= residue.serial_number() && *high >= residue.serial_number())
            }
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
            Term::Backbone if !conformer.is_amino_acid() => Some(false),
            Term::SideChain if !conformer.is_amino_acid() => Some(false),
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
            Term::Element(e) => Some(atom.element() == e),
            Term::BFactor(a) => Some((atom.b_factor() - *a).abs() < f64::EPSILON),
            Term::BFactorRange(low, high) => {
                Some(atom.b_factor() >= *low && atom.b_factor() <= *high)
            }
            Term::Occupancy(a) => Some((atom.occupancy() - *a).abs() < f64::EPSILON),
            Term::OccupancyRange(low, high) => {
                Some(atom.occupancy() >= *low && atom.occupancy() <= *high)
            }
            Term::Backbone => Some(atom.is_backbone()),
            Term::SideChain => Some(!atom.is_backbone()),
            Term::Hetero => Some(atom.hetero()),
            _ => None,
        }
    }
}

/// A collection of multiple search [Term]s in the search for (an) atom(s) in a PDB.
/// You can use bitwise and (`&`), or (`|`), and xor (`^`) to chain a search.
/// In the same way you can use not `!` to negate a search term.
///
/// ```
/// use pdbtbx::*;
/// let (pdb, _errors) = open("example-pdbs/1ubq.pdb", StrictnessLevel::Medium).unwrap();
/// let selection = pdb.find(
///     Term::ConformerName("ALA".to_owned()) & !Term::Element("N".to_owned()));
/// for hierarchy in selection {
///     println!("Atom '{}' is selected", hierarchy.atom().serial_number());
/// }
/// ```
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
                (Ops::And, Search::Known(false), _) | (Ops::And, _, Search::Known(false)) => {
                    Search::Known(false)
                }
                (Ops::And, Search::Known(a), Search::Known(b)) => Search::Known(a & b),
                (Ops::Or, Search::Known(true), _) | (Ops::Or, _, Search::Known(true)) => {
                    Search::Known(true)
                }
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
    #[must_use]
    pub fn complete(&self) -> Option<bool> {
        match self {
            Search::Known(a) => Some(*a),
            _ => None,
        }
    }

    /// Add information about the model into the search, returns a new search with the information integrated
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
            Search::Single(Term::Element("C".to_string()))
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
            (Term::AtomName("CA".to_string()) & Term::Element("C".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            (Term::AtomName("CA".to_string()) | Term::Element("E".to_string()))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            (!Term::AtomName("CA".to_string()) ^ Term::Element("C".to_string()))
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

    #[test]
    fn simplify() {
        assert_eq!(
            (Term::AtomSerialNumber(123) & Search::Known(false))
                .simplify()
                .complete(),
            Some(false)
        );
        assert_eq!(
            (Search::Known(false) & Term::AtomSerialNumber(123))
                .simplify()
                .complete(),
            Some(false)
        );
        assert_eq!(
            (Search::Known(true) | Term::AtomSerialNumber(123))
                .simplify()
                .complete(),
            Some(true)
        );
        assert_eq!(
            (Term::AtomSerialNumber(123) | Search::Known(true))
                .simplify()
                .complete(),
            Some(true)
        );
    }

    #[test]
    fn complex_simplify() {
        assert_eq!(
            (Term::AtomSerialNumber(123)
                & (Term::AtomSerialNumber(123)
                    & (Term::AtomSerialNumber(123)
                        & (Term::AtomSerialNumber(123)
                            & (Term::AtomSerialNumber(123)
                                & (Term::AtomSerialNumber(123)
                                    & (Term::AtomSerialNumber(123) & Search::Known(false))))))))
            .simplify()
            .complete(),
            Some(false)
        );
        assert_eq!(
            (Term::AtomSerialNumber(123)
                | (Term::AtomSerialNumber(123)
                    | (Term::AtomSerialNumber(123)
                        | (Term::AtomSerialNumber(123)
                            | (Term::AtomSerialNumber(123)
                                | (Term::AtomSerialNumber(123)
                                    | (Search::Known(false) ^ Search::Known(true))))))))
            .simplify()
            .complete(),
            Some(true)
        );
        assert_eq!(
            (!Search::Known(false) | Term::AtomSerialNumber(123))
                .simplify()
                .complete(),
            Some(true)
        );
    }
}
