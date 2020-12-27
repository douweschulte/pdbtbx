Primarily a hobby project to work more with Rust, but with a bit of luck it could end up being useful for someone. For now (27 Dec 2020) it can parse simple (crystallographic) PDBs, edit the PDB in place, gives some handy functionality and is able to svae the PDB.

Functionality (not guarenteed complete) and stuff to do
- Parse PDB
  - [x] ANISOU
  - [x] ATOM
  - [x] CRYST
  - [x] HETATM
  - [x] MODEL
  - [x] REMARK
  - [x] SCALEn
  - [x] Use streams (minimise memory usage)
  - [ ] Measure function calls to find suboptimal coding
- Save PDB
  - [x] ANISOU
  - [x] ATOM
  - [x] CRYST
  - [x] HETATM
  - [x] MODEL
  - [x] REMARK
  - [x] SCALEn
  - [ ] Test with other software
     - [x] 1ubq.pdb works with Chimera
     - [ ] pTLS-6484.pdb does not work with Chimera
- Edit PDB
  - [x] Create getters/setters for internal data (plus checks)
  - [x] Create iterators to children
  - [x] Create reference to parent
  - [ ] Create adders for each struct
    - [x] Add_atom
    - [ ] Add_(1 level down)
  - [x] Remove
    - [x] Remove() - itself from parent
    - [x] Remove child from itself (some different selecting options)
- Helping
  - [x] Tell if a residue is an amino acid
  - [x] Tell if an atom is in the backbone
  - [x] Renumber PDB
  - [ ] Find position in Å
  - [ ] Apply affine transformations to atoms
  - [ ] Find symmetry partners (affine) transformations for space_group


