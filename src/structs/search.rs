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
    /// The model serial number, only used in (NMR) PDB files with multiple states of a protein, see [`Model::serial_number`].
    ModelSerialNumber(usize),
    /// Search for a range of model serial numbers, starting at the first number and ending with the last number inclusive.
    ModelSerialNumberRange(usize, usize),
    /// The chain id eg `A`, see [`Chain::id`].
    ChainId(String),
    /// Search for a range of chain ids, using the `Ord` implementation of `std::str` <https://doc.rust-lang.org/std/primitive.str.html#impl-Ord>, starting at the first number and ending with the last number inclusive.
    ChainIdRange(String, String),
    /// The residue serial number, see [`Residue::serial_number`].
    ResidueSerialNumber(isize),
    /// Search for a range of residue serial numbers, starting at the first number and ending with the last number inclusive.
    ResidueSerialNumberRange(isize, isize),
    /// The residue insertion code eg `Some("A")`, see [`Residue::insertion_code`].
    ResidueInsertionCode(Option<String>),
    /// The residue serial number and insertion code combined, see [`Residue::id`].
    ResidueId(isize, Option<String>),
    /// The conformer name eg `ALA`, see [`Conformer::name`].
    ConformerName(String),
    /// The conformer alternative location eg `Some("A")`, see [`Conformer::alternative_location`].
    ConformerAlternativeLocation(Option<String>),
    /// The conformer name and alternative location combined, see [`Conformer::id`].
    ConformerId(String, Option<String>),
    /// The atom serial number, see [`Atom::serial_number`].
    AtomSerialNumber(usize),
    /// A range of atoms based on serial number starting at the first number and ending with the last number inclusive.
    AtomSerialNumberRange(usize, usize),
    /// The atom name eg `CA`, see [`Atom::name`].
    AtomName(String),
    /// The element eq `C`, see [`Atom::element`], see [Element].
    Element(Element),
    /// Atom b factor, see [`Atom::b_factor`].
    BFactor(f64),
    /// Atom B factor range starting at the first number and ending with the last number inclusive.
    BFactorRange(f64, f64),
    /// Atom occupancy, see [`Atom::occupancy`].
    Occupancy(f64),
    /// Atom occupancy range starting at the first number and ending with the last number inclusive.
    OccupancyRange(f64, f64),
    /// Search for backbone atoms, this means that [`Conformer::is_amino_acid`] is `true` and [`Atom::is_backbone`] is `true`.
    Backbone,
    /// Search for side chain atoms, this means that [`Conformer::is_amino_acid`] is `true` and [`Atom::is_backbone`] is `false`.
    SideChain,
    /// Search for hetero atoms, this means that [`Atom::hetero`] is `true`.
    Hetero,
}

impl Term {
    const fn optional_matches_model(&self, model: &Model) -> Option<bool> {
        match self {
            Self::ModelSerialNumber(s) => Some(*s == model.serial_number()),
            Self::ModelSerialNumberRange(low, high) => {
                Some(*low <= model.serial_number() && *high >= model.serial_number())
            }
            _ => None,
        }
    }
    fn optional_matches_chain(&self, chain: &Chain) -> Option<bool> {
        match self {
            Self::ChainId(s) => Some(s == chain.id()),
            Self::ChainIdRange(low, high) => {
                Some(low.as_str() <= chain.id() && high.as_str() >= chain.id())
            }
            _ => None,
        }
    }
    fn optional_matches_residue(&self, residue: &Residue) -> Option<bool> {
        match self {
            Self::ResidueSerialNumber(s) => Some(*s == residue.serial_number()),
            Self::ResidueSerialNumberRange(low, high) => {
                Some(*low <= residue.serial_number() && *high >= residue.serial_number())
            }
            Self::ResidueInsertionCode(ic) => Some(ic.as_deref() == residue.insertion_code()),
            Self::ResidueId(s, ic) => Some((*s, ic.as_deref()) == residue.id()),
            _ => None,
        }
    }
    fn optional_matches_conformer(&self, conformer: &Conformer) -> Option<bool> {
        match self {
            Self::ConformerName(n) => Some(n == conformer.name()),
            Self::ConformerAlternativeLocation(al) => {
                Some(al.as_deref() == conformer.alternative_location())
            }
            Self::ConformerId(n, al) => Some((n.as_str(), al.as_deref()) == conformer.id()),
            Self::Backbone | Self::SideChain if !conformer.is_amino_acid() => Some(false),
            _ => None,
        }
    }
    fn optional_matches_atom(&self, atom: &Atom) -> Option<bool> {
        match self {
            Self::AtomSerialNumber(n) => Some(atom.serial_number() == *n),
            Self::AtomSerialNumberRange(low, high) => {
                Some(atom.serial_number() >= *low && atom.serial_number() <= *high)
            }
            Self::AtomName(n) => Some(atom.name() == n),
            Self::Element(e) => atom.element().map(|a| a == e),
            Self::BFactor(a) => Some((atom.b_factor() - *a).abs() < f64::EPSILON),
            Self::BFactorRange(low, high) => {
                Some(atom.b_factor() >= *low && atom.b_factor() <= *high)
            }
            Self::Occupancy(a) => Some((atom.occupancy() - *a).abs() < f64::EPSILON),
            Self::OccupancyRange(low, high) => {
                Some(atom.occupancy() >= *low && atom.occupancy() <= *high)
            }
            Self::Backbone => Some(atom.is_backbone()),
            Self::SideChain => Some(!atom.is_backbone()),
            Self::Hetero => Some(atom.hetero()),
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
/// let (pdb, _errors) = open("example-pdbs/1ubq.pdb").unwrap();
/// let selection = pdb.find(
///     Term::ConformerName("ALA".to_owned()) & !Term::Element(Element::N));
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
    fn simplify(self) -> Self {
        match self {
            Self::Ops(ops, a, b) => match (ops, a.simplify(), b.simplify()) {
                (Ops::And, Self::Known(false), _) | (Ops::And, _, Self::Known(false)) => {
                    Self::Known(false)
                }
                (Ops::And, Self::Known(a), Self::Known(b)) => Self::Known(a & b),
                (Ops::Or, Self::Known(true), _) | (Ops::Or, _, Self::Known(true)) => {
                    Self::Known(true)
                }
                (Ops::Or, Self::Known(a), Self::Known(b)) => Self::Known(a | b),
                (Ops::Xor, Self::Known(a), Self::Known(b)) => Self::Known(a ^ b),
                (ops, a, b) => Self::Ops(ops, Box::new(a), Box::new(b)),
            },
            Self::Not(a) => match a.simplify() {
                Self::Known(a) => Self::Known(!a),
                a => Self::Not(Box::new(a)),
            },
            _ => self,
        }
    }

    /// Check if the search is done.
    #[must_use]
    pub const fn complete(&self) -> Option<bool> {
        match self {
            Self::Known(a) => Some(*a),
            _ => None,
        }
    }

    /// Add information about the model into the search, returns a new search with the information integrated
    #[must_use]
    pub fn add_model_info(&self, model: &Model) -> Self {
        match self {
            Self::Ops(ops, a, b) => Self::Ops(
                *ops,
                Box::new(a.add_model_info(model)),
                Box::new(b.add_model_info(model)),
            ),
            Self::Not(a) => Self::Not(Box::new(a.add_model_info(model))),
            Self::Single(a) => match a.optional_matches_model(model) {
                Some(true) => Self::Known(true),
                Some(false) => Self::Known(false),
                None => self.clone(),
            },
            Self::Known(_) => self.clone(),
        }
        .simplify()
    }

    /// Add information about the chain into the search, returns a new search with the information integrated
    #[must_use]
    pub fn add_chain_info(&self, chain: &Chain) -> Self {
        match self {
            Self::Ops(ops, a, b) => Self::Ops(
                *ops,
                Box::new(a.add_chain_info(chain)),
                Box::new(b.add_chain_info(chain)),
            ),
            Self::Not(a) => Self::Not(Box::new(a.add_chain_info(chain))),
            Self::Single(a) => match a.optional_matches_chain(chain) {
                Some(true) => Self::Known(true),
                Some(false) => Self::Known(false),
                None => self.clone(),
            },
            Self::Known(_) => self.clone(),
        }
        .simplify()
    }

    /// Add information about the residue into the search, returns a new search with the information integrated
    #[must_use]
    pub fn add_residue_info(&self, residue: &Residue) -> Self {
        match self {
            Self::Ops(ops, a, b) => Self::Ops(
                *ops,
                Box::new(a.add_residue_info(residue)),
                Box::new(b.add_residue_info(residue)),
            ),
            Self::Not(a) => Self::Not(Box::new(a.add_residue_info(residue))),
            Self::Single(a) => match a.optional_matches_residue(residue) {
                Some(true) => Self::Known(true),
                Some(false) => Self::Known(false),
                None => self.clone(),
            },
            Self::Known(_) => self.clone(),
        }
        .simplify()
    }

    /// Add information about the conformer into the search, returns a new search with the information integrated
    #[must_use]
    pub fn add_conformer_info(&self, conformer: &Conformer) -> Self {
        match self {
            Self::Ops(ops, a, b) => Self::Ops(
                *ops,
                Box::new(a.add_conformer_info(conformer)),
                Box::new(b.add_conformer_info(conformer)),
            ),
            Self::Not(a) => Self::Not(Box::new(a.add_conformer_info(conformer))),
            Self::Single(a) => match a.optional_matches_conformer(conformer) {
                Some(true) => Self::Known(true),
                Some(false) => Self::Known(false),
                None => self.clone(),
            },
            Self::Known(_) => self.clone(),
        }
        .simplify()
    }

    /// Add information about the atom into the search, returns a new search with the information integrated
    #[must_use]
    pub fn add_atom_info(&self, atom: &Atom) -> Self {
        match self {
            Self::Ops(ops, a, b) => Self::Ops(
                *ops,
                Box::new(a.add_atom_info(atom)),
                Box::new(b.add_atom_info(atom)),
            ),
            Self::Not(a) => Self::Not(Box::new(a.add_atom_info(atom))),
            Self::Single(a) => match a.optional_matches_atom(atom) {
                Some(true) => Self::Known(true),
                Some(false) => Self::Known(false),
                None => self.clone(),
            },
            Self::Known(_) => self.clone(),
        }
        .simplify()
    }
}

impl ops::BitAnd<Self> for Term {
    type Output = Search;
    fn bitand(self, rhs: Self) -> Self::Output {
        Search::Ops(
            Ops::And,
            Box::new(Search::Single(self)),
            Box::new(Search::Single(rhs)),
        )
    }
}

impl ops::BitOr<Self> for Term {
    type Output = Search;
    fn bitor(self, rhs: Self) -> Self::Output {
        Search::Ops(
            Ops::Or,
            Box::new(Search::Single(self)),
            Box::new(Search::Single(rhs)),
        )
    }
}

impl ops::BitXor<Self> for Term {
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

impl ops::BitAnd<Self> for Search {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::Ops(Ops::And, Box::new(self), Box::new(rhs))
    }
}

impl ops::BitOr<Self> for Search {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Ops(Ops::Or, Box::new(self), Box::new(rhs))
    }
}

impl ops::BitXor<Self> for Search {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Ops(Ops::Xor, Box::new(self), Box::new(rhs))
    }
}

impl ops::Not for Search {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

impl ops::BitAnd<Term> for Search {
    type Output = Self;
    fn bitand(self, rhs: Term) -> Self::Output {
        Self::Ops(Ops::And, Box::new(self), Box::new(Self::Single(rhs)))
    }
}

impl ops::BitOr<Term> for Search {
    type Output = Self;
    fn bitor(self, rhs: Term) -> Self::Output {
        Self::Ops(Ops::Or, Box::new(self), Box::new(Self::Single(rhs)))
    }
}

impl ops::BitXor<Term> for Search {
    type Output = Self;
    fn bitxor(self, rhs: Term) -> Self::Output {
        Self::Ops(Ops::Xor, Box::new(self), Box::new(Self::Single(rhs)))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn atom_find() {
        let a = Atom::new(true, 123, "123", "CA", 0.0, 0.0, 0.0, 0.0, 0.0, "C", 1).unwrap();
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
            Search::Single(Term::Element(Element::C))
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
        let a = Atom::new(true, 123, "123", "CA", 0.0, 0.0, 0.0, 0.0, 0.0, "C", 1).unwrap();
        assert_eq!(
            (Term::AtomName("CA".to_string()) & Term::Element(Element::C))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            (Term::AtomName("CA".to_string()) | Term::Element(Element::Ca))
                .add_atom_info(&a)
                .complete(),
            Some(true)
        );
        assert_eq!(
            (!Term::AtomName("CA".to_string()) ^ Term::Element(Element::C))
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
