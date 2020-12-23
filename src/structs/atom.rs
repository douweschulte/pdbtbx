#![allow(dead_code)]
use crate::structs::*;
use std::fmt;

#[derive(Debug)]
pub struct Atom {
    serial_number: usize,
    name: [char; 4],
    x: f64,
    y: f64,
    z: f64,
    occupancy: f64,
    b_factor: f64,
    element: [char; 2],
    charge: [char; 2],
    residue: Option<*mut Residue>,
}

impl Atom {
    pub fn new(
        residue: Option<&mut Residue>,
        serial_number: usize,
        atom_name: [char; 4],
        x: f64,
        y: f64,
        z: f64,
        occupancy: f64,
        b_factor: f64,
        element: [char; 2],
        charge: [char; 2],
    ) -> Option<Atom> {
        let mut atom = Atom {
            serial_number: serial_number,
            name: atom_name,
            x: x,
            y: y,
            z: z,
            occupancy: occupancy,
            b_factor: b_factor,
            element: element,
            charge: charge,
            residue: None,
        };

        if let Some(reference) = residue {
            atom.residue = Some(reference);
        }

        if !check_char4(atom_name) || !check_char2(element) || !check_char2(charge) {
            None
        } else {
            Some(atom)
        }
    }

    pub fn pos(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

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

    pub fn x(&self) -> f64 {
        self.x
    }

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

    pub fn y(&self) -> f64 {
        self.y
    }

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

    pub fn z(&self) -> f64 {
        self.z
    }

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

    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    pub fn set_serial_number(&mut self, new_serial_number: usize) {
        self.serial_number = new_serial_number;
    }

    pub fn name(&self) -> String {
        self.name
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

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

    pub fn occupancy(&self) -> f64 {
        self.occupancy
    }

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

    pub fn b_factor(&self) -> f64 {
        self.b_factor
    }

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

    pub fn element(&self) -> String {
        self.element
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

    pub fn set_element(&mut self, new_element: &str) -> Result<(), String> {
        let chars = new_element
            .to_ascii_uppercase()
            .chars()
            .collect::<Vec<char>>();
        if chars.len() <= 2 {
            if !check_chars(new_element.to_string()) {
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

    pub fn charge(&self) -> String {
        self.charge
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

    pub fn set_charge(&mut self, new_charge: &str) -> Result<(), String> {
        let chars = new_charge
            .to_ascii_uppercase()
            .chars()
            .collect::<Vec<char>>();
        if chars.len() <= 2 {
            if !check_chars(new_charge.to_string()) {
                self.charge = [chars[0], chars[1]];
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

    pub fn set_residue(&mut self, new_residue: &mut Residue) {
        self.residue = Some(new_residue);
    }

    pub fn set_residue_pointer(&mut self, new_residue: *mut Residue) {
        self.residue = Some(new_residue);
    }

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

    pub fn residue_safe(&self) -> Option<&Residue> {
        if let Some(reference) = self.residue {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    fn residue_mut(&self) -> Option<&mut Residue> {
        if let Some(reference) = self.residue {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    pub fn backbone(&self) -> Option<bool> {
        let residue = self.residue_safe();
        if residue.is_some() {
            let backbone_names = vec!["N", "CA", "C", "O"];
            if residue.unwrap().amino_acid() && backbone_names.contains(&self.name().as_str()) {
                Some(true)
            } else {
                Some(false)
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
            "ATOM ID:{}, Number:{}, Element: {}, X:{}, Y:{}, Z:{}, OCC:{}, B:{}",
            self.name(),
            self.serial_number(),
            self.element(),
            self.x(),
            self.y(),
            self.z(),
            self.occupancy(),
            self.b_factor()
        )
    }
}
