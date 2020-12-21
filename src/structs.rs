#![allow(dead_code)]

pub struct PDB {
    pub remarks: Vec<(usize, String)>,
    pub scale: Option<Scale>,
    pub unit_cell: Option<UnitCell>,
    pub symmetry: Option<Symmetry>,
    pub models: Vec<Model>,
}

impl PDB {
    pub fn new() -> PDB {
        PDB {
            remarks: Vec::new(),
            scale: None,
            unit_cell: None,
            symmetry: None,
            models: Vec::new(),
        }
    }

    pub fn chains(&mut self) -> Vec<&mut Chain> {
        let mut output = Vec::new();

        for model in &mut self.models {
            for chain in &mut model.chains {
                output.push(chain)
            }
        }

        output
    }

    pub fn residues(&mut self) -> Vec<&mut Residue> {
        let mut output = Vec::new();

        for model in &mut self.models {
            output.append(&mut model.residues())
        }

        output
    }

    pub fn atoms(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for model in &mut self.models {
            output.append(&mut model.atoms())
        }

        output
    }

    pub fn scale(&mut self) -> &mut Scale {
        match &mut self.scale {
            Some(s) => s,
            None => panic!("Expected a Scale but it was not in place (it was None)."),
        }
    }
}

pub struct Scale {
    pub factors: [[f64; 4]; 3],
}

impl Scale {
    pub fn new() -> Scale {
        Scale {
            factors: [[0.0; 4]; 3],
        }
    }
}

pub struct UnitCell {
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl UnitCell {
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> UnitCell {
        UnitCell {
            a: a,
            b: b,
            c: c,
            alpha: alpha,
            beta: beta,
            gamma: gamma,
        }
    }

    pub fn a(&self) -> f64 {
        self.a
    }
    pub fn b(&self) -> f64 {
        self.b
    }
    pub fn c(&self) -> f64 {
        self.c
    }
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    pub fn beta(&self) -> f64 {
        self.beta
    }
    pub fn gamma(&self) -> f64 {
        self.gamma
    }
}

pub struct Symmetry {
    space_group: char,
    symbols: Vec<usize>,
}

impl Symmetry {
    pub fn new(space_group: char, symbols: Vec<usize>) -> Symmetry {
        Symmetry {
            space_group: space_group,
            symbols: symbols,
        }
    }

    pub fn symbols(&self) -> &Vec<usize> {
        &self.symbols
    }

    pub fn space_group(&self) -> char {
        self.space_group
    }
}

pub struct Model {
    pub id: String,
    pub chains: Vec<Chain>,
    pub hetero_atoms: Vec<Atom>,
}

impl Model {
    pub fn new(name: Option<&str>) -> Model {
        let mut model = Model {
            id: "".to_string(),
            chains: Vec::new(),
            hetero_atoms: Vec::new(),
        };

        if let Some(n) = name {
            model.id = n.to_string();
        }

        model
    }

    pub fn residues(&mut self) -> Vec<&mut Residue> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            for residue in &mut chain.residues {
                output.push(residue)
            }
        }

        output
    }

    pub fn atoms(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            output.append(&mut chain.atoms())
        }

        for het in &mut self.hetero_atoms {
            output.push(het)
        }

        output
    }
}

pub struct Chain {
    pub id: char,
    pub residues: Vec<Residue>,
}

impl Chain {
    pub fn new(id: Option<char>) -> Chain {
        let mut c = 'a';
        if let Some(ch) = id {
            c = ch;
        }
        Chain {
            id: c,
            residues: Vec::new(),
        }
    }

    pub fn atoms(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for residue in &mut self.residues {
            for atom in &mut residue.atoms {
                output.push(atom)
            }
        }

        output
    }
}

pub struct Residue {
    pub id: [char; 3],
    pub serial_number: usize,
    pub atoms: Vec<Atom>,
    pub amino_acid: bool,
}

impl Residue {
    pub fn new(number: usize, name: Option<[char; 3]>, atom: Option<Atom>) -> Residue {
        let mut res = Residue {
            id: [' ', ' ', ' '],
            serial_number: number,
            atoms: Vec::new(),
            amino_acid: false,
        };

        if let Some(a) = atom {
            res.atoms.push(a);
        }

        if let Some(n) = name {
            res.id = n;

            let trimmed_name = n
                .iter()
                .collect::<String>()
                .split_whitespace()
                .collect::<String>();

            let amino_acid_names = vec![
                "ALA", "ARG", "ASN", "ASP", "CYS", "GLN", "GLU", "GLY", "HIS", "ILE", "LEU", "LYS",
                "MET", "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL",
            ];
            if amino_acid_names.contains(&trimmed_name.as_str()) {
                res.amino_acid = true;
            }
        }

        res
    }

    pub fn id(&self) -> String {
        self.id
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }
}

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
    backbone: bool,
}

impl Atom {
    pub fn new(
        residue: [char; 3],
        serial_number: usize,
        atom_name: [char; 4],
        x: f64,
        y: f64,
        z: f64,
        occupancy: f64,
        b_factor: f64,
        element: [char; 2],
        charge: [char; 2],
    ) -> Atom {
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
            backbone: false,
        };

        let amino_acid_names = vec![
            "ALA", "ARG", "ASN", "ASP", "CYS", "GLN", "GLU", "GLY", "HIS", "ILE", "LEU", "LYS",
            "MET", "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL",
        ];
        let backbone_names = vec!["N", "CA", "C", "O"];
        if amino_acid_names.contains(&residue.iter().collect::<String>().as_str())
            && backbone_names.contains(&atom.name().as_str())
        {
            atom.backbone = true;
        }

        atom
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
            self.name = [chars[0], chars[1], chars[2], chars[3]];
            Ok(())
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
            self.element = [chars[0], chars[1]];
            Ok(())
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
            self.charge = [chars[0], chars[1]];
            Ok(())
        } else {
            Err(format!(
                "New charge is too long (max 2 chars) for atom {} name {}",
                self.serial_number, new_charge
            ))
        }
    }

    pub fn backbone(&self) -> bool {
        self.backbone
    }
}
