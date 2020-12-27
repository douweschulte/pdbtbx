#![allow(dead_code)]

#[derive(Debug)]
pub struct Scale {
    pub factors: [[f64; 4]; 3],
}

impl Scale {
    pub fn new() -> Scale {
        Scale {
            factors: [[0.0; 4]; 3],
        }
    }
}

impl Clone for Scale {
    fn clone(&self) -> Self {
        let mut scale = Scale::new();

        scale.factors = self.factors.clone();

        scale
    }
}
