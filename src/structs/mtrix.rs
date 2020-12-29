#![allow(dead_code)]

#[derive(Debug)]
pub struct MtriX {
    pub serial_number: usize,
    pub factors: [[f64; 4]; 3],
    pub contained: bool,
}

impl MtriX {
    pub fn new() -> Self {
        MtriX {
            serial_number: 0,
            factors: [[0.0; 4]; 3],
            contained: true,
        }
    }
}

impl Clone for MtriX {
    fn clone(&self) -> Self {
        let mut m = MtriX::new();

        m.factors = self.factors.clone();

        m
    }
}
