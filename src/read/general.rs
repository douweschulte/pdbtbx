use super::*;
use crate::error::*;
use crate::structs::PDB;
use crate::StrictnessLevel;

/// Open an atomic data file, either PDB or mmCIF/PDBx. The correct type will be
/// determined based on the extension of the file. Returns an [`PDBError`] when it found
/// a [`ErrorLevel::BreakingError`]. Otherwise it returns the PDB with all errors/warnings found while parsing it.
pub fn open(filename: &str, level: StrictnessLevel) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    if filename.ends_with(".pdb") {
        open_pdb(filename, level)
    } else if filename.ends_with(".cif") {
        open_mmcif(filename, level)
    } else {
        Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Incorrect extension",
            "Could not determine the type of the given file, make it .pdb or .cif",
            Context::show(filename),
        )])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_invalid() {
        assert!(open("file.png", StrictnessLevel::Medium).is_err());
        assert!(open("file.mmcif", StrictnessLevel::Medium).is_err());
        assert!(open("file.pdbml", StrictnessLevel::Medium).is_err());
        assert!(open("file.pd", StrictnessLevel::Medium).is_err());
    }

    #[test]
    fn open_not_existing() {
        let pdb = open("file.pdb", StrictnessLevel::Medium).unwrap_err();
        assert_eq!(pdb[0].short_description(), "Could not open file");
        let cif = open("file.cif", StrictnessLevel::Medium).unwrap_err();
        assert_eq!(cif[0].short_description(), "Could not open file");
    }
}
