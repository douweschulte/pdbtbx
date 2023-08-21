use pdbtbx::*;
use std::path::Path;
use std::time::Instant;
use std::{env, fs};

#[test]
fn run_pdbs() {
    let current_dir = env::current_dir().unwrap();
    let pdb_dir = current_dir.as_path().join(Path::new("example-pdbs"));
    let dump_dir = current_dir
        .as_path()
        .join(Path::new("dump"))
        .into_os_string()
        .into_string()
        .unwrap()
        + &String::from(std::path::MAIN_SEPARATOR);
    let _ = fs::create_dir(dump_dir.clone());
    println!("{pdb_dir:?}");

    save_invalid_name();
    save_pdb_strict();
    save_mmcif_strict();

    for entry in fs::read_dir(pdb_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        if metadata.is_file() {
            do_something(
                &path.clone().into_os_string().into_string().unwrap(),
                &dump_dir,
                &path
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
            );
        }
    }
}

fn do_something(file: &str, folder: &str, name: &str) {
    println!("Working on file: {file}");
    let now = Instant::now();

    let (pdb, errors) = open(file, StrictnessLevel::Loose).unwrap();

    let time = now.elapsed();

    for error in errors {
        println!("{error}");
    }

    println!(
        "Found {} atoms, in {} residues, in {} chains, in {} models it all took {} ms",
        pdb.total_atom_count(),
        pdb.total_residue_count(),
        pdb.total_chain_count(),
        pdb.model_count(),
        time.as_millis()
    );

    assert!(pdb.total_atom_count() != 0, "No atoms found");

    println!("PDB parsed");

    let mut avg = 0.0;
    let mut total_back = 0;
    let mut avg_back = 0.0;
    let mut total_side = 0;
    let mut avg_side = 0.0;

    println!("Set values");

    for hierarchy in pdb.atoms_with_hierarchy() {
        avg += hierarchy.atom().b_factor();
        if hierarchy.is_backbone() {
            total_back += 1;
            avg_back += hierarchy.atom().b_factor();
        } else {
            total_side += 1;
            avg_side += hierarchy.atom().b_factor();
        }
    }

    println!("Counted for averages");

    avg /= (total_back + total_side) as f64;
    avg_back /= total_back as f64;
    avg_side /= total_side as f64;

    println!("Found averages");

    println!(
        "Average B factor: Total: {avg:.3}, Backbone: {avg_back:.3}, Sidechains: {avg_side:.3}"
    );

    if validate_pdb(&pdb)
        .iter()
        .all(|a| !a.fails(StrictnessLevel::Medium))
    {
        save(
            &pdb,
            folder.to_string() + name + ".pdb",
            StrictnessLevel::Loose,
        )
        .expect("PDB resave not successful");
        let (_saved_pdb, _) = open(folder.to_string() + name + ".pdb", StrictnessLevel::Loose)
            .expect("PDB reparse not successful");
        //assert_eq!(pdb, saved_pdb);
    }
    save(
        &pdb,
        folder.to_string() + name + ".cif",
        StrictnessLevel::Loose,
    )
    .expect("mmCIF resave not successful");
    let (_saved_mmcif, _) = open(folder.to_string() + name + ".cif", StrictnessLevel::Loose)
        .expect("mmCIF reparse not successful");

    // These should be equal in the future
    //assert_eq!(pdb, saved_mmcif);
}

fn save_invalid_name() {
    let name = env::current_dir()
        .unwrap()
        .as_path()
        .join(Path::new("dump"))
        .join(Path::new("save_test.name"))
        .into_os_string()
        .into_string()
        .unwrap();
    let res = save(&PDB::new(), name, StrictnessLevel::Loose);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.len(), 1);
    assert_eq!(err[0].short_description(), "Incorrect extension")
}

fn save_pdb_strict() {
    let name = env::current_dir()
        .unwrap()
        .as_path()
        .join(Path::new("dump"))
        .join(Path::new("save_test.pdb"))
        .into_os_string()
        .into_string()
        .unwrap();

    let atom = Atom::new(false, 0, "H", 0.0, 0.0, 0.0, 0.0, 0.0, "H", 0).unwrap();
    let mut model = Model::new(0);
    model.add_atom(atom, "A", (0, None), ("LYS", None));
    let mut pdb = PDB::new();
    pdb.add_model(model);

    let res = save(&pdb, &name, StrictnessLevel::Strict);
    assert!(res.is_ok());
    let (_pdb, errors) = crate::open(&name, StrictnessLevel::Strict).unwrap();
    assert_eq!(errors.len(), 0);

    // Do it also for gzip
    #[cfg(feature = "compression")]
    {
        let name = env::current_dir()
            .unwrap()
            .as_path()
            .join(Path::new("dump"))
            .join(Path::new("save_test.pdb.gz"))
            .into_os_string()
            .into_string()
            .unwrap();

        let res = save_gz(&pdb, &name, StrictnessLevel::Strict, None);
        assert!(res.is_ok());
        let (_pdb, errors) = crate::open_gz(&name, StrictnessLevel::Strict).unwrap();
        assert_eq!(errors.len(), 0);
    }
}

fn save_mmcif_strict() {
    let name = env::current_dir()
        .unwrap()
        .as_path()
        .join(Path::new("dump"))
        .join(Path::new("save_test.cif"))
        .into_os_string()
        .into_string()
        .unwrap();

    let atom = Atom::new(false, 0, "H", 0.0, 0.0, 0.0, 0.0, 0.0, "H", 0).unwrap();
    let mut model = Model::new(0);
    model.add_atom(atom, "A", (0, None), ("LYS", None));
    let mut pdb = PDB::new();
    pdb.add_model(model);

    let res = save(&pdb, &name, StrictnessLevel::Strict);
    println!("{res:?}");
    assert!(res.is_ok());
    let (_pdb, errors) = crate::open(&name, StrictnessLevel::Strict).unwrap();
    assert_eq!(errors.len(), 0);

    // Do it also for gzip
    #[cfg(feature = "compression")]
    {
        let name = env::current_dir()
            .unwrap()
            .as_path()
            .join(Path::new("dump"))
            .join(Path::new("save_test.cif.gz"))
            .into_os_string()
            .into_string()
            .unwrap();

        let res = save_gz(&pdb, &name, StrictnessLevel::Strict, None);
        assert!(res.is_ok());
        let (_pdb, errors) = crate::open_gz(&name, StrictnessLevel::Strict).unwrap();
        assert_eq!(errors.len(), 0);
    }
}
