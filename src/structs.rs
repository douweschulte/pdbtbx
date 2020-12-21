#![allow(dead_code)]

pub struct PDB {
    pub remarks: Vec<String>,
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
    atom_name: [char; 4],
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
            atom_name: atom_name,
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
            && backbone_names.contains(&atom.atom_name().as_str())
        {
            atom.backbone = true;
        }

        atom
    }

    pub fn pos(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    pub fn atom_name(&self) -> String {
        self.atom_name
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

    pub fn occupancy(&self) -> f64 {
        self.occupancy
    }

    pub fn b_factor(&self) -> f64 {
        self.b_factor
    }

    pub fn element(&self) -> String {
        self.element
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

    pub fn charge(&self) -> String {
        self.charge
            .iter()
            .collect::<String>()
            .split_whitespace()
            .collect::<String>()
    }

    pub fn backbone(&self) -> bool {
        self.backbone
    }
}
