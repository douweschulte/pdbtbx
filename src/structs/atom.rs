#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use std::cmp::Ordering;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

static ATOM_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A struct to represent a single Atom in a protein
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Atom {
    /// The unique serial number given to this atom
    counter: usize,
    /// Determines if this atom is a hetero atom (true), a non standard atom, or a normal atom (false)
    hetero: bool,
    /// The serial number of the Atom, should be unique within its model
    serial_number: usize,
    /// The name of the Atom, can only use the standard allowed characters
    name: String,
    /// The X position of the Atom (Å)
    x: f64,
    /// The Y position of the Atom (Å)
    y: f64,
    /// The Z position of the Atom (Å)
    z: f64,
    /// The occupancy of the Atom
    occupancy: f64,
    /// The B-factor (or temperature factor) of the Atom
    b_factor: f64,
    /// The element of the Atom, can only use the standard allowed characters
    element: String,
    /// The charge of the Atom
    charge: isize,
    /// The anisotropic temperature factors, if applicable
    atf: Option<[[f64; 3]; 3]>,
}

impl Atom {
    /// Create a new Atom
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hetero: bool,
        serial_number: usize,
        atom_name: impl Into<String>,
        x: f64,
        y: f64,
        z: f64,
        occupancy: f64,
        b_factor: f64,
        element: impl Into<String>,
        charge: isize,
    ) -> Option<Atom> {
        let atom_name = atom_name.into();
        let element = element.into();
        if valid_identifier(&atom_name)
            && valid_identifier(&element)
            && x.is_finite()
            && y.is_finite()
            && z.is_finite()
            && occupancy.is_finite()
            && b_factor.is_finite()
        {
            Some(Atom {
                counter: ATOM_COUNTER.fetch_add(1, AtomicOrdering::SeqCst),
                hetero,
                serial_number,
                name: atom_name.trim().to_ascii_uppercase(),
                x,
                y,
                z,
                occupancy,
                b_factor,
                element: element.trim().to_ascii_uppercase(),
                charge,
                atf: None,
            })
        } else {
            None
        }
    }

    /// Get a unique immutable counter for this atom.
    pub(crate) fn counter(&self) -> usize {
        self.counter
    }

    /// Determine if this atom is an hetero atom (`true`), a non standard atom, or a normal atom (`false`).
    pub fn hetero(&self) -> bool {
        self.hetero
    }

    /// Set whether this atom is an hetero atom (`true`), a non standard atom, or a normal atom (`false`).
    pub fn set_hetero(&mut self, new_hetero: bool) {
        self.hetero = new_hetero
    }

    /// Get the position of the atom as a tuple of `f64`, in the following order: (x, y, z).
    /// Given in Å as defined by PDB in the orthogonal coordinate system.
    pub fn pos(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Set the position of the atom as a tuple of `f64`, in the following order: (x, y, z).
    /// It fails if one or more of the numbers is not finite (`f64.is_finite()`).
    pub fn set_pos(&mut self, new_pos: (f64, f64, f64)) -> Result<(), String> {
        if new_pos.0.is_finite() && new_pos.1.is_finite() && new_pos.2.is_finite() {
            self.x = new_pos.0;
            self.y = new_pos.1;
            self.z = new_pos.2;
            Ok(())
        } else {
            Err(format!(
                "One (or more) of values of the new position is not finite for atom {} values {:?}",
                self.serial_number, new_pos
            ))
        }
    }

    /// Get the X position of the atom.
    /// Given in Å as defined by PDB in the orthogonal coordinate system.
    /// This number has a precision of 8.3 in PDB files and 5 decimal places of precision in mmCIF files.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Set the X position of the atom in Å.
    /// It fails if `new_pos` is not finite (`f64.is_finite()`).
    pub fn set_x(&mut self, new_pos: f64) -> Result<(), String> {
        if new_pos.is_finite() {
            self.x = new_pos;
            Ok(())
        } else {
            Err(format!(
                "The value of the new x position is not finite for atom {} value {}",
                self.serial_number, new_pos
            ))
        }
    }

    /// Get the Y position of the atom.
    /// Given in Å as defined by PDB in the orthogonal coordinate system.
    /// This number has a precision of 8.3 in PDB files and 5 decimal places of precision in mmCIF files.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Set the Y position of the atom in Å.
    /// It fails if `new_pos` is not finite (`f64.is_finite()`).
    pub fn set_y(&mut self, new_pos: f64) -> Result<(), String> {
        if new_pos.is_finite() {
            self.y = new_pos;
            Ok(())
        } else {
            Err(format!(
                "The value of the new y position is not finite for atom {} value {}",
                self.serial_number, new_pos
            ))
        }
    }

    /// Get the Z position of the atom.
    /// Given in Å as defined by PDB in the orthogonal coordinate system.
    /// This number has a precision of 8.3 in PDB files and 5 decimal places of precision in mmCIF files.
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Set the Z position of the atom in Å.
    /// It fails if `new_pos` is not finite (`f64.is_finite()`).
    pub fn set_z(&mut self, new_pos: f64) -> Result<(), String> {
        if new_pos.is_finite() {
            self.z = new_pos;
            Ok(())
        } else {
            Err(format!(
                "The value of the new z position is not finite for atom {} value {}",
                self.serial_number, new_pos
            ))
        }
    }

    /// Get the serial number of the atom.
    /// This number, combined with the alt_loc from the Conformer of this Atom, is defined to be unique in the containing model, which is not enforced.
    /// The precision of this number is 5 digits in PDB files.
    /// If more than 99,999 atoms are present in the same model, the internal numbering will
    /// continue counting up even if the file from which the atoms were read does not.
    /// Importantly, this will not affect the saving of the file, only the internal handling of
    /// atom serial numbers.
    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    /// Set the serial number of the atom.
    /// This number, combined with the alt_loc from the Conformer, of this Atom is defined to be unique in the containing model, which is not enforced.
    pub fn set_serial_number(&mut self, new_serial_number: usize) {
        self.serial_number = new_serial_number;
    }

    /// Get the name of the atom. The name will be trimmed (whitespace removed) and changed to ASCII uppercase.
    /// For PDB files the name can at most contain 4 characters.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name of the atom. The name will be trimmed (whitespace removed) and changed to ASCII uppercase as requested by PDB/PDBx standard.
    /// For PDB files the name can at most contain 4 characters.
    /// The name can only contain valid characters, the ASCII graphic characters (`char.is_ascii_graphic() || char == ' '`).
    /// If the name is invalid an error message is provided.
    pub fn set_name(&mut self, new_name: impl Into<String>) -> Result<(), String> {
        let new_name = new_name.into();
        if !valid_identifier(&new_name) {
            Err(format!(
                "New name has invalid characters for atom {} name {}",
                self.serial_number, new_name
            ))
        } else {
            self.name = new_name.trim().to_ascii_uppercase();
            Ok(())
        }
    }

    /// Get the occupancy or Q factor of the atom. This indicates the fraction of unit cells in which this atom is present, in the normal case this will be one (1) and it can range between 1 and 0 (inclusive).
    /// This number has a precision of 6.2 in PDB files and 5 decimal places of precision in mmCIF files.
    pub fn occupancy(&self) -> f64 {
        self.occupancy
    }

    /// Set the occupancy or Q factor of the atom.
    /// It fails if `new_occupancy` is not finite (`f64.is_finite()`) or if it is negative.
    pub fn set_occupancy(&mut self, new_occupancy: f64) -> Result<(), String> {
        if new_occupancy.is_finite() {
            if new_occupancy >= 0.0 {
                self.occupancy = new_occupancy;
                Ok(())
            } else {
                Err(format!(
                    "The value of the new occupancy is negative for atom {} value {}",
                    self.serial_number, new_occupancy
                ))
            }
        } else {
            Err(format!(
                "The value of the new occupancy is not finite for atom {} value {}",
                self.serial_number, new_occupancy
            ))
        }
    }

    /// Get the B factor or temperature factor of the atom.
    /// This indicates the uncertainty in the position of the atom as seen over all unit cells in the whole crystal.
    /// A low uncertainty is modelled with a low B factor, with zero uncertainty being equal to a B factor of 0. A higher uncertainty is modelled by a high B factor.
    /// This number has a precision of 6.2 in PDB files and 5 decimal places of precision in mmCIF files.
    pub fn b_factor(&self) -> f64 {
        self.b_factor
    }

    /// Set the B factor or temperature factor of the atom.
    /// It fails if `new_b_factor` is not finite (`f64.is_finite()`) or if it is negative.
    pub fn set_b_factor(&mut self, new_b_factor: f64) -> Result<(), String> {
        if new_b_factor.is_finite() {
            if new_b_factor >= 0.0 {
                self.b_factor = new_b_factor;
                Ok(())
            } else {
                Err(format!(
                    "The value of the new b_factor is negative for atom {} value {}",
                    self.serial_number, new_b_factor
                ))
            }
        } else {
            Err(format!(
                "The value of the new b_factor is not finite for atom {} value {}",
                self.serial_number, new_b_factor
            ))
        }
    }

    /// Get the element of this atom.
    /// In PDB files the element can at most contain 2 characters.
    pub fn element(&self) -> &str {
        &self.element
    }

    /// Set the element of this atom. The element will be trimmed (whitespace removed) and changed to ASCII uppercase as requested by PDB/PDBx standard.
    /// For PDB files the element can at most contain 2 characters.
    /// The element can only contain valid characters, the ASCII graphic characters (`char.is_ascii_graphic() || char == ' '`).
    /// If the element is invalid an error message is provided.
    pub fn set_element(&mut self, new_element: impl Into<String>) -> Result<(), String> {
        let new_element = new_element.into();
        if !valid_identifier(&new_element) {
            Err(format!(
                "New element has invalid characters for atom {} name {}",
                self.serial_number, new_element
            ))
        } else {
            self.element = new_element.trim().to_ascii_uppercase();
            Ok(())
        }
    }

    /// Get the atomic number of this Atom. If defined, it uses `self.element()`, otherwise it uses `self.name()`, if that still does not
    /// find anything it uses the first character of `self.name()` if that is one of "CHONS". This could potentially lead to misclassifications
    /// but in most cases should result in the correct element. It should be noted that the element is a required part in the PDB file so not specifying it
    /// is likely to lead to unspecified behaviour.
    /// It returns `None` if `self.element()` is not an element in the periodic table, or if `self.element()` is undefined and `self.name()` is not an element in the periodic table.
    pub fn atomic_number(&self) -> Option<usize> {
        if !self.element.is_empty() {
            reference_tables::get_atomic_number(self.element())
        } else if !self.name().is_empty() {
            let name = reference_tables::get_atomic_number(self.name());
            #[allow(clippy::unwrap_used)] // The name is not empty so it cannot fail
            if name.is_none() && "CHONS".contains(self.name().chars().next().unwrap()) {
                reference_tables::get_atomic_number(
                    &self.name().chars().next().unwrap().to_string(),
                )
            } else {
                name
            }
        } else {
            None
        }
    }

    /// Get the atomic radius of this Atom in Å. The radius is defined up to element 'Cm' or atomic number 96.
    /// Source: Martin Rahm, Roald Hoffmann, and N. W. Ashcroft. Atomic and Ionic Radii of Elements 1-96.
    /// Chemistry - A European Journal, 22(41):14625–14632, oct 2016. URL: <http://doi.org/10.1002/chem.201602949>.
    /// Updated to the corrigendum: <https://doi.org/10.1002/chem.201700610>.
    ///
    /// It returns `None` if the atomic number of this Atom is not defined (see `self.atomic_number()`).
    /// The same is true if the atomic radius is not defined for the given atomic number, i.e. if the atomic
    /// number is higher than 96.
    pub fn atomic_radius(&self) -> Option<f64> {
        self.atomic_number()
            .and_then(reference_tables::get_atomic_radius)
    }

    /// Get the van der Waals radius for this Atom in Å. The radius is defined up to element 'Es' or atomic number 99.
    /// Source: Alvarez, S. (2013). A cartography of the van der Waals territories. Dalton Transactions, 42(24), 8617. <https://doi.org/10.1039/c3dt50599e>.
    ///
    /// It returns `None` if the atomic number of this Atom is not defined (see `self.atomic_number()`).
    /// The same is true if the atomic radius is not defined for the given atomic number, i.e. if the atomic
    /// number is higher than 99.
    pub fn vanderwaals_radius(&self) -> Option<f64> {
        self.atomic_number()
            .and_then(reference_tables::get_vanderwaals_radius)
    }

    /// Gets the covalent bond radii for this Atom.
    /// The result is the radius for a single, double and triple bond, where the last two are optional. If the radius for a double bond is unknown, the radius for a triple bond is also unknown.
    /// All values are given in Å.
    /// Sources:
    ///  * P. Pyykkö; M. Atsumi (2009). "Molecular Single-Bond Covalent Radii for Elements 1-118". Chemistry: A European Journal. 15 (1): 186–197. <https://doi.org/10.1002/chem.200800987>
    ///  * P. Pyykkö; M. Atsumi (2009). "Molecular Double-Bond Covalent Radii for Elements Li–E112". Chemistry: A European Journal. 15 (46): 12770–12779. <https://doi.org/10.1002/chem.200901472>
    ///  * P. Pyykkö; S. Riedel; M. Patzschke (2005). "Triple-Bond Covalent Radii". Chemistry: A European Journal. 11 (12): 3511–3520. <https://doi.org/10.1002/chem.200401299>
    ///
    /// It returns `None` if the atomic number of this Atom is not defined (see `self.atomic_number()`).
    pub fn covalent_bond_radii(&self) -> Option<(f64, Option<f64>, Option<f64>)> {
        self.atomic_number()
            .map(reference_tables::get_covalent_bond_radii)
    }

    /// Get the charge of this atom.
    /// In PDB files the charge is one digit with a sign.
    pub fn charge(&self) -> isize {
        self.charge
    }

    /// Set the charge of this atom.
    /// In PDB files the charge is one digit with a sign.
    pub fn set_charge(&mut self, new_charge: isize) {
        self.charge = new_charge;
    }

    /// Get the charge in the PDB format `[0-9][-+]`. If the charge is 0 or outside bounds (below -9 or above 9) it returns an empty string.
    #[allow(clippy::cast_possible_truncation)]
    pub fn pdb_charge(&self) -> String {
        if self.charge == 0 || self.charge < -9 || self.charge > 9 {
            String::new()
        } else {
            let mut sign = '+';
            let charge = (48 + self.charge.abs() as u8) as char;
            if self.charge < 0 {
                sign = '-';
            }
            let mut output = String::new();
            output.push(charge);
            output.push(sign);
            output
        }
    }

    /// Get the anisotropic temperature factors, if available.
    /// This number has a precision of 8.3 in PDB files and 5 decimal places of precision in mmCIF files.
    pub fn anisotropic_temperature_factors(&self) -> Option<[[f64; 3]; 3]> {
        self.atf
    }

    /// Set the anisotropic temperature factors.
    pub fn set_anisotropic_temperature_factors(&mut self, factors: [[f64; 3]; 3]) {
        self.atf = Some(factors);
    }

    /// Determine whether this atom is likely to be a part of the backbone of a protein.
    /// This is based on this Atom only, for a more precise definition use [hierarchy::ContainsAtomConformer]`.is_backbone()`.
    pub fn is_backbone(&self) -> bool {
        reference_tables::is_backbone(self.name())
    }

    /// Apply a transformation using a given `TransformationMatrix` to the position of this atom, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.set_pos(transformation.apply(self.pos()))
            .expect("Some numbers were invalid in applying a transformation");
    }

    /// See if the `other` Atom corresponds with this Atom.
    /// This means that the Atoms are equal except for the position, occupancy, and b_factor.
    /// Used to validate that multiple models contain the same atoms, but with different positional data.
    pub fn corresponds(&self, other: &Atom) -> bool {
        self.serial_number == other.serial_number
            && self.name() == other.name()
            && self.element() == other.element()
            && self.charge() == other.charge()
            && ((self.atf.is_none() && other.atf.is_none())
                || (self.atf.is_some() && other.atf.is_some()))
    }

    /// Gives the distance between the centers of two atoms in Aͦ.
    pub fn distance(&self, other: &Atom) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }

    /// Gives the distance between the centers of two atoms in Aͦ, wrapping around the unit cell if needed.
    /// This will give the shortest distance between the two atoms or any of their copies given a crystal of the size of the given unit cell stretching out to all sides.
    pub fn distance_wrapping(&self, other: &Atom, cell: &UnitCell) -> f64 {
        let mut x = other.x;
        if (self.x - other.x).abs() > cell.a() / 2.0 {
            if self.x > other.x {
                x += cell.a();
            } else {
                x -= cell.a();
            }
        }

        let mut y = other.y;
        if (self.y - other.y).abs() > cell.b() / 2.0 {
            if self.y > other.y {
                y += cell.b();
            } else {
                y -= cell.b();
            }
        }

        let mut z = other.z;
        if (self.z - other.z).abs() > cell.c() / 2.0 {
            if self.z > other.z {
                z += cell.c();
            } else {
                z -= cell.c();
            }
        }

        ((x - self.x).powi(2) + (y - self.y).powi(2) + (z - self.z).powi(2)).sqrt()
    }

    #[allow(clippy::similar_names)]
    /// Gives the angle between the centers of three atoms in degrees.
    /// The angle is calculated as the angle between the two lines that include
    /// atoms [1, 2] and [2, 3]
    pub fn angle(&self, atom2: &Atom, atom3: &Atom) -> f64 {
        let (a, b, c) = (self.pos(), atom2.pos(), atom3.pos());

        // Form the two vectors
        let ba = [a.0 - b.0, a.1 - b.1, a.2 - b.2];
        let bc = [c.0 - b.0, c.1 - b.1, c.2 - b.2];

        // Calculate absolute values of vectors
        let abs_ba = ba.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
        let abs_bc = bc.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

        // Form dot product between vecs
        let dot = ba
            .iter()
            .zip(bc.iter())
            .fold(0.0, |acc, (a, b)| acc + (a * b));

        // Calculate angle from all ingredients
        (dot / (abs_ba * abs_bc)).acos().to_degrees()
    }

    #[allow(clippy::similar_names)]
    /// Gives the dihedral between the centers of four atoms in degrees.
    /// The angle is calculated as the angle between the two planes spanned by
    /// atoms [1, 2, 3] and [2, 3, 4].
    pub fn dihedral(&self, atom2: &Atom, atom3: &Atom, atom4: &Atom) -> f64 {
        let (a, b, c, d) = (self.pos(), atom2.pos(), atom3.pos(), atom4.pos());

        // Form vectors
        let ba = [a.0 - b.0, a.1 - b.1, a.2 - b.2];
        let bc = [c.0 - b.0, c.1 - b.1, c.2 - b.2];
        let cb = [b.0 - c.0, b.1 - c.1, b.2 - c.2];
        let cd = [d.0 - c.0, d.1 - c.1, d.2 - c.2];

        // Form two normal vectors via cross products
        let n1 = [
            ba[1] * bc[2] - ba[2] * bc[1],
            ba[2] * bc[0] - ba[0] * bc[2],
            ba[0] * bc[1] - ba[1] * bc[0],
        ];
        let n2 = [
            cb[1] * cd[2] - cb[2] * cd[1],
            cb[2] * cd[0] - cb[0] * cd[2],
            cb[0] * cd[1] - cb[1] * cd[0],
        ];

        // calculate abs of vecs
        let abs_n1 = n1.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();
        let abs_n2 = n2.iter().fold(0.0, |acc, x| acc + (x * x)).sqrt();

        let dot = n1
            .iter()
            .zip(n2.iter())
            .fold(0.0, |acc, (a, b)| acc + (a * b));
        (dot / (abs_n1 * abs_n2)).acos().to_degrees()
    }

    /// Checks whether this Atom overlaps with the given atom. It overlaps if the distance between the atoms is
    /// less then the sum of the radii of this atom and the other atom. The used radius is (`atom.atomic_radius()`).
    ///
    /// Note: the atomic radius used is the unbound radius, this is in most cases bigger than the bound radius
    /// and as such can result in false positives.
    ///
    /// It returns `None` if, for any one of the two atoms, the radius (`atom.atomic_radius()`) is not defined.
    pub fn overlaps(&self, other: &Atom) -> Option<bool> {
        self.atomic_radius().and_then(|self_rad| {
            other
                .atomic_radius()
                .map(|other_rad| self.distance(other) <= self_rad + other_rad)
        })
    }

    /// Checks if this Atom overlaps with the given atom. It overlaps if the distance between the atoms is
    /// less then the sum of the radii of this atom and the other atom, wrapping around the unit cell if needed.
    /// The used radius is (`atom.atomic_radius()`). This will give the shortest distance between the two
    /// atoms or any of their copies given a crystal of the size of the given unit cell stretching out to
    /// all sides.
    ///
    /// Note: the atomic radius used is the unbound radius, this is in most cases bigger than the bound radius
    /// and as such can result in false positives.
    ///
    /// It fails if for any one of the two atoms the radius (`atom.atomic_radius()`) is not defined.
    pub fn overlaps_wrapping(&self, other: &Atom, cell: &UnitCell) -> Option<bool> {
        self.atomic_radius().and_then(|self_rad| {
            other
                .atomic_radius()
                .map(|other_rad| self.distance_wrapping(other, cell) <= self_rad + other_rad)
        })
    }

    /// Checks if this Atom overlaps with the given atom. It overlaps if the distance between the atoms is
    /// less then the sum of the radius from this atom and the other atom. The used radius is `atom.covalent_bond_radii().0`.
    ///
    /// Note: the atomic radius used in the bound radius to a single atom, this is similar to the bound radius for double or
    /// triple bonds but could result in incorrect results.
    ///
    /// It fails if for any one of the two atoms the radius (`atom.covalent_bond_radii()`) is not defined.
    pub fn overlaps_bound(&self, other: &Atom) -> Option<bool> {
        self.covalent_bond_radii().and_then(|self_rad| {
            other
                .covalent_bond_radii()
                .map(|other_rad| self.distance(other) <= self_rad.0 + other_rad.0)
        })
    }

    /// Checks if this Atom overlaps with the given atom. It overlaps if the distance between the atoms is
    /// less then the sum of the radii of this atom and the other atom, wrapping around the unit cell if needed.
    /// The used radius is `atom.covalent_bond_radii().0`. This will give the shortest distance between the two
    /// atoms or any of their copies given a crystal of the size of the given unit cell stretching out to
    /// all sides.
    ///
    /// Note: the atomic radius used is the bound radius to a single atom, this is similar to the bound radius for double or
    /// triple bonds but could result in incorrect results.
    ///
    /// It returns `None` if for any one of the two atoms the radius (`atom.covalent_bond_radii()`) is not defined.
    pub fn overlaps_bound_wrapping(&self, other: &Atom, cell: &UnitCell) -> Option<bool> {
        self.covalent_bond_radii().and_then(|self_rad| {
            other
                .covalent_bond_radii()
                .map(|other_rad| self.distance_wrapping(other, cell) <= self_rad.0 + other_rad.0)
        })
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ATOM ID: {}, Number: {}, Element: {}, X: {}, Y: {}, Z: {}, OCC: {}, B: {}, ANISOU: {}",
            self.name(),
            self.serial_number(),
            self.element(),
            self.x(),
            self.y(),
            self.z(),
            self.occupancy(),
            self.b_factor(),
            self.atf.is_some()
        )
    }
}

impl Clone for Atom {
    /// The clone implementation needs to use the constructor to guarantee the uniqueness of the counter
    fn clone(&self) -> Self {
        let mut atom = Atom::new(
            self.hetero,
            self.serial_number,
            &self.name,
            self.x,
            self.y,
            self.z,
            self.occupancy,
            self.b_factor,
            &self.element,
            self.charge,
        )
        .expect("Invalid Atom properties in a clone");
        atom.atf = self.atf;
        atom
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.serial_number == other.serial_number
            && self.name() == other.name()
            && self.element() == other.element()
            && self.charge() == other.charge()
            && self.atf == other.atf
            && self.pos() == other.pos()
            && self.occupancy == other.occupancy
            && self.b_factor == other.b_factor
    }
}

/// As there are a lot of checks to make sure only 'normal' f64 values are used
/// Atom satisfies the properties needed for Eq while having f64 values.
impl Eq for Atom {}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.serial_number.cmp(&other.serial_number))
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.serial_number.cmp(&other.serial_number)
    }
}

#[cfg(feature = "rstar")]
use rstar::{PointDistance, RTreeObject, AABB};

#[cfg(feature = "rstar")]
impl RTreeObject for &Atom {
    type Envelope = AABB<(f64, f64, f64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point(self.pos())
    }
}

#[cfg(feature = "rstar")]
impl PointDistance for &Atom {
    fn distance_2(&self, other: &(f64, f64, f64)) -> f64 {
        // No square root as that is required by the package
        (other.0 - self.x).powi(2) + (other.1 - self.y).powi(2) + (other.2 - self.z).powi(2)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::Atom;
    use super::UnitCell;

    #[test]
    fn set_name() {
        let mut a = Atom::new(false, 0, "", 0.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap();
        assert!(a.set_name("Å").is_err());
        assert!(a.set_name("ATOMS").is_ok());
        a.set_name("ATOM").unwrap();
        a.set_name("HOH").unwrap();
        a.set_name("RK").unwrap();
        a.set_name("R").unwrap();
        a.set_name("").unwrap();
    }

    #[test]
    fn set_element() {
        let mut a = Atom::new(false, 0, "", 0.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap();
        assert!(a.set_element("R̈").is_err());
        assert!(a.set_element("HOH").is_ok());
        a.set_element("RK").unwrap();
        a.set_element("R").unwrap();
        a.set_element("").unwrap();
    }

    #[test]
    fn distance() {
        let a = Atom::new(false, 0, "", 1.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap();
        let b = Atom::new(false, 0, "", 9.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap();
        let cell = UnitCell::new(10.0, 10.0, 10.0, 90.0, 90.0, 90.0);
        assert!(!a.overlaps(&b).unwrap());
        assert!(a.overlaps_wrapping(&b, &cell).unwrap());
        assert_eq!(a.distance(&b), 8.0);
        assert_eq!(a.distance_wrapping(&b, &cell), 2.0);
    }

    #[test]
    fn angles() {
        let a = Atom::new(false, 0, "", 1.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap();
        let b = Atom::new(false, 0, "", 0.0, 1.0, 0.0, 0.0, 0.0, "C", 0).unwrap();
        let c = Atom::new(false, 0, "", 0.0, 0.0, 1.0, 0.0, 0.0, "C", 0).unwrap();
        let d = Atom::new(false, 0, "", 1.0, 1.0, 1.0, 0.0, 0.0, "C", 0).unwrap();
        let e = Atom::new(false, 0, "", 0.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap();

        assert!((a.angle(&b, &c) - 60.0).abs() < 0.0001);
        assert!((a.dihedral(&e, &c, &d) - 45.0).abs() < 0.0001);
    }

    #[test]
    fn distance_all_axes() {
        let a = Atom::new(false, 0, "", 1.0, 1.0, 1.0, 0.0, 0.0, "C", 0).unwrap();
        let b = Atom::new(false, 0, "", 9.0, 9.0, 9.0, 0.0, 0.0, "C", 0).unwrap();
        let cell = UnitCell::new(10.0, 10.0, 10.0, 90.0, 90.0, 90.0);
        assert!(!a.overlaps(&b).unwrap());
        assert!(a.overlaps_wrapping(&b, &cell).unwrap());
    }

    #[test]
    fn check_equality() {
        let a = Atom::new(false, 0, "", 1.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap();
        let b = Atom::new(false, 0, "", 9.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap();
        let c = Atom::new(false, 0, "", 9.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap();
        assert_ne!(a, b);
        assert_eq!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn invalid_new_values() {
        let mut a = Atom::new(false, 0, "", 1.0, 1.0, 1.0, 0.0, 0.0, "C", 0).unwrap();
        assert!(Atom::new(false, 0, "Rͦ", 1.0, 1.0, 1.0, 0.0, 0.0, "C", 0).is_none());
        assert!(Atom::new(false, 0, "R", 1.0, 1.0, 1.0, 0.0, 0.0, "Cͦ", 0).is_none());
        assert!(a.set_x(f64::INFINITY).is_err());
        assert!(a.set_x(f64::NAN).is_err());
        assert!(a.set_x(f64::NEG_INFINITY).is_err());
        assert!(a.set_y(f64::INFINITY).is_err());
        assert!(a.set_z(f64::INFINITY).is_err());
        assert!(a.set_pos((f64::INFINITY, 0., 0.)).is_err());
        assert!(a.set_pos((0., f64::INFINITY, 0.)).is_err());
        assert!(a.set_pos((0., 0., f64::INFINITY)).is_err());
        assert!(a.set_b_factor(f64::INFINITY).is_err());
        assert!(a.set_b_factor(-1.0).is_err());
        assert!(a.set_occupancy(f64::INFINITY).is_err());
        assert!(a.set_occupancy(-1.).is_err());
    }

    #[test]
    fn check_setters() {
        let mut a = Atom::new(false, 0, "C", 1.0, 1.0, 1.0, 0.0, 0.0, "", 0).unwrap();
        assert!(a.set_x(2.0).is_ok());
        assert_eq!(a.x(), 2.0);
        assert!(a.set_y(2.0).is_ok());
        assert_eq!(a.y(), 2.0);
        assert!(a.set_z(2.0).is_ok());
        assert_eq!(a.z(), 2.0);
        assert!(a.set_pos((3.0, 3.0, 3.0)).is_ok());
        assert_eq!(a.x(), 3.0);
        assert_eq!(a.y(), 3.0);
        assert_eq!(a.z(), 3.0);
        assert_eq!(a.pos(), (3.0, 3.0, 3.0));
        assert!(a.set_b_factor(2.0).is_ok());
        assert_eq!(a.b_factor(), 2.0);
        assert!(a.set_occupancy(2.0).is_ok());
        assert_eq!(a.occupancy(), 2.0);
        assert!(a.set_occupancy(0.0).is_ok());
        assert!(a.set_b_factor(0.0).is_ok());
        a.set_hetero(true);
        assert!(a.hetero());
        a.set_serial_number(42);
        assert_eq!(a.serial_number(), 42);
        assert_eq!(a.atomic_number(), Some(6));
        a.set_charge(-1);
        assert_eq!(a.charge(), -1);
        assert_eq!(a.pdb_charge(), "1-".to_string());
    }

    #[test]
    fn check_radii() {
        let a = Atom::new(false, 0, "H", 1.0, 1.0, 1.0, 0.0, 0.0, "", 0).unwrap();
        assert_eq!(a.atomic_radius(), Some(1.54));
        assert_eq!(a.vanderwaals_radius(), Some(1.20));
        assert_eq!(a.covalent_bond_radii(), Some((0.32, None, None)));
        let a = Atom::new(false, 0, "Cl", 1.0, 1.0, 1.0, 0.0, 0.0, "", 0).unwrap();
        assert_eq!(a.atomic_radius(), Some(2.06));
        assert_eq!(a.vanderwaals_radius(), Some(1.82));
        assert_eq!(
            a.covalent_bond_radii(),
            Some((0.99, Some(0.95), Some(0.93)))
        );
    }

    #[test]
    fn check_display() {
        let a = Atom::new(false, 0, "C", 1.0, 1.0, 1.0, 0.0, 0.0, "", 0).unwrap();
        format!("{:?}", a);
        format!("{}", a);
    }
}
