#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use std::fmt;

/// A struct to represent a single Atom in a protein
#[derive(Debug)]
pub struct Atom {
    /// The serial number of the Atom, should be unique within its model
    serial_number: usize,
    /// The name of the Atom, can only be four chars, can only use the standard allowed characters
    name: [char; 4],
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
    /// The element of the Atom, can only be two chars, can only use the standard allowed characters
    element: [char; 2],
    /// The charge of the Atom
    charge: isize,
    /// The anisotropic temperature factors, if applicable
    atf: Option<[[f64; 3]; 2]>,
}

impl Atom {
    /// Create a new Atom
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serial_number: usize,
        atom_name: [char; 4],
        x: f64,
        y: f64,
        z: f64,
        occupancy: f64,
        b_factor: f64,
        element: [char; 2],
        charge: isize,
    ) -> Option<Atom> {
        let atom = Atom {
            serial_number,
            name: atom_name,
            x,
            y,
            z,
            occupancy,
            b_factor,
            element,
            charge,
            atf: None,
        };

        if !check_char4(atom_name) || !check_char2(element) {
            None
        } else {
            Some(atom)
        }
    }

    /// Get the position of the atom as a tuple of f64, in the following order: (x, y, z)
    /// Returned in the units of the PDB file, which is defined to be orthogonal coordinate system in Å
    pub fn pos(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Set the position of the atom as a tuple of f64, in the following order: (x, y, z)
    /// ## Panics
    /// It panics if one or more of the numbers are not finite (`f64.is_finite()`)
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

    /// Get the X position of the atom
    /// Returned in the units of the PDB file, which is defined to be orthogonal coordinate system in Å
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Set the X position of the atom
    /// ## Panics
    /// It panics if `new_pos` is not finite (`f64.is_finite()`)
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

    /// Get the Y position of the atom
    /// Returned in the units of the PDB file, which is defined to be orthogonal coordinate system in Å
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Set the Y position of the atom
    /// ## Panics
    /// It panics if `new_pos` is not finite (`f64.is_finite()`)
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

    /// Get the Z position of the atom
    /// Returned in the units of the PDB file, which is defined to be orthogonal coordinate system in Å
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Set the Z position of the atom
    /// ## Panics
    /// It panics if `new_pos` is not finite (`f64.is_finite()`)
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

    /// Get the serial number of the atom
    /// This number is defined to be unique in the containing model, which is not enforced
    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    /// Set the serial number of the atom
    /// This number is defined to be unique in the containing model, which is not enforced
    pub fn set_serial_number(&mut self, new_serial_number: usize) {
        self.serial_number = new_serial_number;
    }

    /// Get the name of the atom
    /// The name is max 4 characters and is trimmed
    pub fn name(&self) -> String {
        self.name
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

    /// Set the name of the atom
    /// If the name is invalid an error message is provided
    /// ## Errors
    /// The name should at max contain 4 characters (ASCII)
    /// The name can only contain valid characters, the ASCII graphic characters (char.is_ascii_graphic() || char == ' ')
    pub fn set_name(&mut self, new_name: &str) -> Result<(), String> {
        let chars = new_name.to_ascii_uppercase().chars().collect::<Vec<char>>();
        if chars.len() < 5 {
            if !check_chars(new_name.to_string()) {
                self.name = [chars[0], chars[1], chars[2], chars[3]];
                Ok(())
            } else {
                Err(format!(
                    "New name has invalid characters for atom {} name {}",
                    self.serial_number, new_name
                ))
            }
        } else {
            Err(format!(
                "New name is too long (max 4 chars) for atom {} name {}",
                self.serial_number, new_name
            ))
        }
    }

    /// Get the occupancy of the atom
    pub fn occupancy(&self) -> f64 {
        self.occupancy
    }

    /// Set the occupancy of the atom
    /// ## Panics
    /// It panics if `new_occupancy` is not finite (`f64.is_finite()`)
    pub fn set_occupancy(&mut self, new_occupancy: f64) -> Result<(), String> {
        if new_occupancy.is_finite() {
            self.occupancy = new_occupancy;
            Ok(())
        } else {
            Err(format!(
                "The value of the new occupancy is not finite for atom {} value {}",
                self.serial_number, new_occupancy
            ))
        }
    }

    /// Get the b-factor or temperature factor of the atom
    pub fn b_factor(&self) -> f64 {
        self.b_factor
    }

    /// Set the b-factor of the atom
    /// ## Panics
    /// It panics if `new_b_factor` is not finite (`f64.is_finite()`)
    pub fn set_b_factor(&mut self, new_b_factor: f64) -> Result<(), String> {
        if new_b_factor.is_finite() {
            self.b_factor = new_b_factor;
            Ok(())
        } else {
            Err(format!(
                "The value of the new b_factor is not finite for atom {} value {}",
                self.serial_number, new_b_factor
            ))
        }
    }

    /// Get the element of this atom
    pub fn element(&self) -> String {
        self.element
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

    /// Get the atomic number of this atom. If defined it uses `self.element()`, otherwise it uses `self.name()` of the atom.
    /// ## Fails
    /// It fails when the element() or name() is not a valid element name.
    pub fn atomic_number(&self) -> Option<usize> {
        if self.element != [' ', ' '] {
            reference_tables::get_atomic_number(&self.element())
        } else {
            reference_tables::get_atomic_number(&self.name())
        }
    }

    /// Get the atomic radius of this Atom in Å. The radius is defined up to Cm.
    /// Source: Martin Rahm, Roald Hoffmann, and N. W. Ashcroft. Atomic and Ionic Radii of Elements 1-96.
    /// Chemistry - A European Journal, 22(41):14625–14632, oct 2016. URL:
    /// http://doi.wiley.com/10.1002/chem.201602949, doi:10.1002/chem.201602949.
    /// ## Fails
    /// It fails if the element name if this Atom is not defined (see `self.atomic_number()`).
    /// It also fails when the atomic radius is not defined for the given atomic number, so if the atomic
    /// number is higher than 96.
    pub fn atomic_radius(&self) -> Option<f64> {
        if let Some(s) = self.atomic_number() {
            reference_tables::get_atomic_radius(s)
        } else {
            None
        }
    }

    /// Set the element of this atom
    /// ## Fails
    /// It fails if the element contains invalid characters (only ASCII graphic and space is allowed).
    /// It also fails if the string is too ling, the max length is 2 characters.
    pub fn set_element(&mut self, new_element: &str) -> Result<(), String> {
        let chars = new_element
            .to_ascii_uppercase()
            .chars()
            .collect::<Vec<char>>();
        if chars.len() <= 2 {
            if check_chars(new_element.to_string()) {
                self.element = [chars[0], chars[1]];
                Ok(())
            } else {
                Err(format!(
                    "New element has invalid characters for atom {} name {}",
                    self.serial_number, new_element
                ))
            }
        } else {
            Err(format!(
                "New element is too long (max 2 chars) for atom {} name {}",
                self.serial_number, new_element
            ))
        }
    }

    /// Get the charge of the atom
    pub fn charge(&self) -> isize {
        self.charge
    }

    /// Get the charge in the PDB format [0-9][-+]
    pub fn pdb_charge(&self) -> String {
        if self.charge == 0 {
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

    /// Set the charge of this atom
    /// ## Fails
    /// It fails if the charge contains invalid characters (only ASCII graphic and space is allowed).
    /// It also fails if the string is too ling, the max length is 2 characters.
    pub fn set_charge(&mut self, new_charge: isize) -> Result<(), String> {
        if new_charge < -9 || new_charge > 9 {
            Err(format!(
                "New charge is out of bounds, for Atom {}, with new charge {}",
                self.serial_number, new_charge
            ))
        } else {
            self.charge = new_charge;
            Ok(())
        }
    }

    /// Get the anisotropic temperature factors, if available
    pub fn anisotropic_temperature_factors(&self) -> Option<[[f64; 3]; 2]> {
        self.atf
    }

    /// Set the anisotropic temperature factors
    pub fn set_anisotropic_temperature_factors(&mut self, factors: [[f64; 3]; 2]) {
        self.atf = Some(factors);
    }

    /// Get if this atom is likely to be a part of the backbone of a protein
    pub fn backbone(&self) -> bool {
        let backbone_names = vec!["N", "CA", "C", "O"];
        backbone_names.contains(&self.name().as_str())
    }

    /// Apply a transformation to the position of this atom, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.set_pos(transformation.apply(self.pos())).unwrap();
    }

    /// See if the `other` Atom corresponds with this Atom.
    /// Which means that the Atoms are equal except for the position, occupancy, and b_factor.
    /// Used to validate that multiple models contain the same atoms, but with different positional data.
    pub fn corresponds(&self, other: &Atom) -> bool {
        self.serial_number == other.serial_number
            && self.name() == other.name()
            && self.element() == other.element()
            && self.charge() == other.charge()
            && ((self.atf.is_none() && other.atf.is_none())
                || (self.atf.is_some() && other.atf.is_some()))
    }

    /// Checks if this Atom overlaps with the given atom. It overlaps if the sphere defined as sitting at
    /// the atom position with a radius of the atomic radius (`atom.atomic_radius()`) intersect with this
    /// sphere from the other Atom.
    /// ## Fails
    /// It fails if any one of the two radii are not defined.
    pub fn overlaps(&self, other: &Atom) -> Option<bool> {
        if let Some(self_rad) = self.atomic_radius() {
            if let Some(other_rad) = other.atomic_radius() {
                Some(
                    self.x() + self_rad > other.x() - other_rad
                        && self.x() - self_rad < other.x() + other_rad
                        && self.y() + self_rad > other.y() - other_rad
                        && self.y() - self_rad < other.y() + other_rad
                        && self.z() + self_rad > other.z() - other_rad
                        && self.z() - self_rad < other.z() + other_rad,
                )
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn clone(&self) -> Self {
        let mut atom = Atom::new(
            self.serial_number,
            self.name,
            self.x,
            self.y,
            self.z,
            self.occupancy,
            self.b_factor,
            self.element,
            self.charge,
        )
        .unwrap();

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
