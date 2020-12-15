pub enum LexItem {
    Remark(String),
    Atom(bool, usize, [char; 4], char, [char; 3], char, usize, char, f64, f64, f64, f64, f64, [char; 4], [char; 2], [char; 2]),
    Scale(usize, [f64; 4]),
    Crystal(f64, f64, f64, f64, f64, f64, Vec<usize>),
    Model(String),
    EndModel(),
    TER(),
    End(),
}