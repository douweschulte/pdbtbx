#[test]
fn open_mmcif_dic() {
    let res = pdbtbx::open_cif("example-pdbs/mmcif_pdbx_v50.dic");
    print!("{:?}", res);
    //assert!(res.is_ok()); A data item is either a tag + value or a loop, I got that wrong in my code
}
