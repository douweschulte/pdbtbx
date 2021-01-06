use rust_pdb::*;
use std::time::Instant;

#[test]
fn simple_waterbox() {
    let water = create_waterbox((25.0, 25.0, 25.0));
    save(&water, "dump/waterbox.pdb").expect("Save not successful");
}

fn create_waterbox(size: (f64, f64, f64)) -> PDB {
    let now = Instant::now();

    let (mut liquid, _errors) = parse("example-pdbs/liquid.pdb").unwrap();

    let time = now.elapsed();

    println!("Time to parse liquid.pdb {}ms", time.as_millis());

    let cell = liquid.unit_cell().size().clone();
    let fa = (size.0 / cell.0).ceil() as usize;
    let fb = (size.1 / cell.1).ceil() as usize;
    let fc = (size.2 / cell.2).ceil() as usize;

    for a in 0..fa {
        for b in 0..fb {
            for c in 0..fc {
                let mut extra = liquid.model(0).unwrap().clone();
                extra.apply_transformation(&TransformationMatrix::translation(
                    a as f64 * cell.0,
                    b as f64 * cell.1,
                    c as f64 * cell.2,
                ));
                liquid.model_mut(0).unwrap().join(extra);
            }
        }
    }

    liquid.remove_atoms_by(|atom| {
        atom.x() < 0.0
            || atom.x() > size.0
            || atom.y() < 0.0
            || atom.y() > size.1
            || atom.z() < 0.0
            || atom.z() > size.2
    });

    liquid.renumber();

    liquid
}
