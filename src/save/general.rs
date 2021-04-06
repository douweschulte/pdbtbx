use super::*;
use crate::error::*;
use crate::structs::PDB;
use crate::StrictnessLevel;

/// Save the given PDB struct to the given file.
/// It validates the PDB. It fails if the validation fails with the given `level`.
/// If validation gives rise to problems use the `save_raw` function. The correct file
/// type (pdb or mmCIF/PDBx) will be determined based on the extension of the file.
pub fn save(pdb: PDB, filename: &str, level: StrictnessLevel) -> Result<(), Vec<PDBError>> {
    if filename.ends_with(".pdb") {
        save_pdb(pdb, filename, level)
    } else if filename.ends_with(".cif") {
        save_mmcif(pdb, filename, level)
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
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn invalid_name() {
        let res = save(PDB::new(), "dump/save_test.name", StrictnessLevel::Loose);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].short_description(), "Incorrect extension")
    }

    #[test]
    fn pdb_strict() {
        let res = save(PDB::new(), "dump/save_test.pdb", StrictnessLevel::Strict);
        assert!(res.is_ok());
        let (_pdb, errors) = crate::open("dump/save_test.pdb", StrictnessLevel::Strict).unwrap();
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn mmcif_strict() {
        let res = save(PDB::new(), "dump/save_test.cif", StrictnessLevel::Strict);
        println!("{:?}", res);
        assert!(res.is_ok());
        let (_pdb, errors) = crate::open("dump/save_test.cif", StrictnessLevel::Strict).unwrap();
        assert_eq!(errors.len(), 0);
    }
}
