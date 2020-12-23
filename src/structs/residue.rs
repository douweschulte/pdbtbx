#![allow(dead_code)]
use crate::structs::*;
use std::fmt;

#[derive(Debug)]
pub struct Residue {
    id: [char; 3],
    serial_number: usize,
    atoms: Vec<Atom>,
    chain: Option<*mut Chain>,
}

impl Residue {
    pub fn new(
        number: usize,
        name: Option<[char; 3]>,
        atom: Option<Atom>,
        chain: Option<&mut Chain>,
    ) -> Option<Residue> {
        let mut res = Residue {
            id: [' ', ' ', ' '],
            serial_number: number,
            atoms: Vec::new(),
            chain: None,
        };

        if let Some(mut a) = atom {
            a.set_residue(&mut res);
            res.atoms.push(a);
        }

        if let Some(c) = chain {
            res.chain = Some(c);
        }

        if let Some(n) = name {
            if !check_char3(n) {
                return None;
            }
            res.id = n;
        }

        Some(res)
    }

    pub fn id(&self) -> String {
        let str_id = self.id.iter().collect::<String>();
        if str_id != "   " {
            str_id.split_whitespace().collect::<String>()
        } else {
            " ".to_string()
        }
    }

    pub fn set_id(&mut self, new_id: &str) -> Result<(), String> {
        let chars = new_id.to_ascii_uppercase().chars().collect::<Vec<char>>();
        if chars.len() <= 3 {
            if !check_chars(new_id.to_string()) {
                self.id = [chars[0], chars[1], chars[2]];
                Ok(())
            } else {
                Err(format!(
                    "New id has invalid characters for residue {} name {}",
                    self.serial_number, new_id
                ))
            }
        } else {
            Err(format!(
                "New id is too long (max 3 chars) for residue {} name {}",
                self.serial_number, new_id
            ))
        }
    }

    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    pub fn atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();
        for atom in &self.atoms {
            output.push(atom);
        }
        output
    }

    pub fn atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();
        for atom in &mut self.atoms {
            output.push(atom);
        }
        output
    }

    pub fn add_atom(&mut self, mut new_atom: Atom) {
        new_atom.set_residue(self);
        self.atoms.push(new_atom);
    }

    pub fn amino_acid(&self) -> bool {
        let amino_acid_names = vec![
            "ALA", "ARG", "ASN", "ASP", "CYS", "GLN", "GLU", "GLY", "HIS", "ILE", "LEU", "LYS",
            "MET", "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL",
        ];
        if amino_acid_names.contains(&self.id().as_str()) {
            true
        } else {
            false
        }
    }

    pub fn set_chain(&mut self, new_chain: &mut Chain) {
        self.chain = Some(new_chain);
    }

    pub fn set_chain_pointer(&mut self, new_chain: *mut Chain) {
        self.chain = Some(new_chain);
    }

    pub fn chain(&self) -> &Chain {
        if let Some(reference) = self.chain {
            unsafe { &*reference }
        } else {
            panic!(format!(
                "No value for chain parent for the current residue {}",
                self.serial_number
            ))
        }
    }

    pub fn chain_safe(&self) -> Option<&Chain> {
        if let Some(reference) = self.chain {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    fn chain_mut(&self) -> Option<&mut Chain> {
        if let Some(reference) = self.chain {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Residue = self;
        for atom in &mut self.atoms {
            atom.set_residue_pointer(reference);
        }
    }
}

impl fmt::Display for Residue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RESIDUE ID:{}, Number:{}, Atoms:{}",
            self.id(),
            self.serial_number(),
            self.atoms.len(),
        )
    }
}
