pub struct PDB {
    remarks: Vec<String>,
    scale: Scale,
    unit_cell: UnitCell,
    symmetry: Symmetry,
    models: Vec<Model>,
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
    id: String,
    chains: Vec<Chain>,
    hetero_atoms: Vec<Atom>,
}

pub struct Chain {
    id: char,
    residues: Vec<Residue>
}

pub struct Residue {
    id: String,
    serial_number: usize,
    atoms: Vec<Atom>
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
}