#![allow(dead_code)]

pub struct PDB {
    pub remarks: Vec<String>,
    pub scale: Option<Scale>,
    pub unit_cell: Option<UnitCell>,
    pub symmetry: Option<Symmetry>,
    pub models: Vec<Model>,
}

pub struct Scale {
    factors: [[f64; 4]; 3]
}

pub struct UnitCell {
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
}

pub struct Symmetry {
    symbols: Vec<usize>,
}

pub struct Model {
    pub id: String,
    pub chains: Vec<Chain>,
    pub hetero_atoms: Vec<Atom>,
}

pub struct Chain {
    pub id: char,
    pub residues: Vec<Residue>
}

pub struct Residue {
    pub id: [char; 3],
    pub serial_number: usize,
    pub atoms: Vec<Atom>
}

pub struct Atom {
    pub serial_number: usize,
    pub atom_name: [char; 4],
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub occupancy: f64,
    pub b_factor: f64,
    pub element: [char; 2],
    pub charge: [char; 2],
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
}

impl Scale {
    pub fn new() -> Scale {
        Scale {
            factors: [[0.0; 4]; 3]
        }
    }
}

impl UnitCell {
    pub fn new() -> UnitCell {
        UnitCell {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            alpha: 0.0,
            beta: 0.0,
            gamma: 0.0,
        }
    }
}

impl Symmetry {
    pub fn new() -> Symmetry {
        Symmetry {
            symbols: vec!(1)
        }
    }
}

impl Model {
    pub fn new() -> Model { Model {id: "".to_string(), chains: Vec::new(), hetero_atoms: Vec::new()}}

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

impl Residue {
    pub fn new(number: usize, name: Option<[char; 3]>, atom: Option<Atom>) -> Residue {
        let mut res = Residue {
            id: [' ', ' ', ' '],
            serial_number: number,
            atoms: Vec::new(),
        };

        if let Some(a) = atom {
            res.atoms.push(a);
        }

        if let Some(n) = name {
            res.id = n;
        }

        res
    }
}