#![allow(dead_code)]

#[derive(Debug)]
pub struct OrigX {
    pub factors: [[f64; 4]; 3],
}

impl OrigX {
    pub fn new() -> OrigX {
        OrigX {
            factors: [[0.0; 4]; 3],
        }
    }
}

impl Clone for OrigX {
    fn clone(&self) -> Self {
        let mut orig = OrigX::new();

        orig.factors = self.factors.clone();

        orig
    }
}
