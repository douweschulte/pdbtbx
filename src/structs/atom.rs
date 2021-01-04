#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;
use std::fmt;

/// A struct to represent a single Atom in a protein
#[derive(Debug)]
pub struct Atom {
    /// The serial number of the Atom, should be unique within its model
    serial_number: usize,
    /// The name of the Atom, can only be four chars, can only use the standard allowed characters
    name: [u8; 4],
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
    element: [u8; 2],
    /// The charge of the Atom, can only be two chars, can only use the standard allowed characters
    charge: [u8; 2],
    /// The parent residue, can only be safely used as a reference (not mutable)
    residue: Option<*mut Residue>,
    /// The anisotropic temperature factors, if applicable
    atf: Option<[[f64; 3]; 2]>,
}

impl Atom {
    /// Create a new Atom
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        residue: Option<*mut Residue>,
        serial_number: usize,
        atom_name: [u8; 4],
        x: f64,
        y: f64,
        z: f64,
        occupancy: f64,
        b_factor: f64,
        element: [u8; 2],
        charge: [u8; 2],
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
            residue,
            atf: None,
        };

        if !check_chars(&atom_name) || !check_chars(&element) || !check_chars(&charge) {
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
        String::from_utf8(
            self.name
                .iter()
                .filter_map(|c| {
                    if !c.is_ascii_whitespace() {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }

    /// Set the name of the atom
    /// If the name is invalid an error message is provided
    /// ## Errors
    /// The name should at max contain 4 characters (ASCII)
    /// The name can only contain valid characters, the ASCII graphic characters (char.is_ascii_graphic() || char == ' ')
    pub fn set_name(&mut self, new_name: &str) -> Result<(), String> {
        let new_name = new_name.to_uppercase();
        let bytes = new_name.as_bytes();
        if bytes.len() < 5 {
            if !check_chars(bytes) {
                self.name = [bytes[0], bytes[1], bytes[2], bytes[3]];
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
        String::from_utf8(
            self.element
                .iter()
                .filter_map(|c| {
                    if !c.is_ascii_whitespace() {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }

    /// Set the element of this atom
    /// ## Fails
    /// It fails if the element contains invalid characters (only ASCII graphic and space is allowed).
    /// It also fails if the string is too ling, the max length is 2 characters.
    pub fn set_element(&mut self, new_element: &str) -> Result<(), String> {
        let new_element = new_element.to_uppercase();
        let bytes = new_element.as_bytes();
        if bytes.len() <= 2 {
            if !check_chars(&bytes) {
                self.element = [bytes[0], bytes[1]];
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
    pub fn charge(&self) -> String {
        // TODO: charge should be i8?
        String::from_utf8(
            self.charge
                .iter()
                .filter_map(|c| {
                    if !c.is_ascii_whitespace() {
                        Some(*c)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }

    /// Set the charge of this atom
    /// ## Fails
    /// It fails if the charge contains invalid characters (only ASCII graphic and space is allowed).
    /// It also fails if the string is too ling, the max length is 2 characters.
    pub fn set_charge(&mut self, new_charge: &str) -> Result<(), String> {
        let new_charge = new_charge.to_uppercase();
        let bytes = new_charge.as_bytes();
        if bytes.len() <= 2 {
            if !check_chars(bytes) {
                self.charge = [bytes[0], bytes[1]];
                Ok(())
            } else {
                Err(format!(
                    "New charge has invalid characters for atom {} name {}",
                    self.serial_number, new_charge
                ))
            }
        } else {
            Err(format!(
                "New charge is too long (max 2 chars) for atom {} name {}",
                self.serial_number, new_charge
            ))
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

    /// Set the parent residue. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_residue(&mut self, new_residue: &mut Residue) {
        self.residue = Some(new_residue);
    }

    /// Set the parent residue. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_residue_pointer(&mut self, new_residue: *mut Residue) {
        self.residue = Some(new_residue);
    }

    /// Get the parent residue.
    /// ## Panics
    /// It panics if there is no parent residue set.
    pub fn residue(&self) -> &Residue {
        if let Some(reference) = self.residue {
            unsafe { &*reference }
        } else {
            panic!(format!(
                "No value for residue parent for the current atom {}",
                self.serial_number
            ))
        }
    }

    /// Get the parent residue, but it does not panic.
    pub fn residue_safe(&self) -> Option<&Residue> {
        if let Some(reference) = self.residue {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    /// Get a mutable reference to the parent, pretty unsafe so you need to make sure yourself the use case is correct.
    /// ## Panics
    /// It panics when no parent is set.
    #[allow(clippy::mut_from_ref)]
    fn residue_mut(&self) -> &mut Residue {
        if let Some(reference) = self.residue {
            unsafe { &mut *reference }
        } else {
            panic!(format!(
                "No value for residue parent for the current atom {}",
                self.serial_number
            ))
        }
    }

    /// Get a mutable reference to the parent, pretty unsafe so you need to make sure yourself the use case is correct.
    fn residue_mut_safe(&self) -> Option<&mut Residue> {
        if let Some(reference) = self.residue {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    /// Get if this atom is likely to be a part of the backbone of a protein
    pub fn backbone(&self) -> Option<bool> {
        let residue = self.residue_safe();
        if let Some(res) = residue {
            let backbone_names = vec!["N", "CA", "C", "O"];
            if res.amino_acid() && backbone_names.contains(&self.name().as_str()) {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    }

    /// Remove this Atom from its parent Residue
    pub fn remove(&mut self) {
        self.residue_mut()
            .remove_atom_serial_number(self.serial_number());
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
            None,
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
