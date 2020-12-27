#![allow(dead_code)]

#[derive(Debug)]
pub struct UnitCell {
    a: f64,
    b: f64,
    c: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl UnitCell {
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> UnitCell {
        UnitCell {
            a: a,
            b: b,
            c: c,
            alpha: alpha,
            beta: beta,
            gamma: gamma,
        }
    }

    pub fn a(&self) -> f64 {
        self.a
    }
    pub fn b(&self) -> f64 {
        self.b
    }
    pub fn c(&self) -> f64 {
        self.c
    }
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    pub fn beta(&self) -> f64 {
        self.beta
    }
    pub fn gamma(&self) -> f64 {
        self.gamma
    }
}

impl Clone for UnitCell {
    fn clone(&self) -> Self {
        UnitCell::new(self.a, self.b, self.c, self.alpha, self.beta, self.gamma)
    }
}
