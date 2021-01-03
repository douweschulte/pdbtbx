#![allow(dead_code)]

#[derive(Debug, Clone)]
/// A unit cell of a crystal, containing its dimensions and angles
pub struct UnitCell {
    /// a-axis dimension
    a: f64,
    /// b-axis dimension
    b: f64,
    /// c-axis dimension
    c: f64,
    /// alpha angle in degrees
    alpha: f64,
    /// beta angle in degrees
    beta: f64,
    /// gamma angle in degrees
    gamma: f64,
}

impl UnitCell {
    /// Create a new UnitCell construct.
    /// ## Arguments
    /// * `a` - a-axis dimension
    /// * `b` - b-axis dimension
    /// * `c` - c-axis dimension
    /// * `alpha` - alpha angle in degrees
    /// * `beta` - beta angle in degrees
    /// * `gamma` - gamma angle in degrees
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

    /// Get the a-axis dimension
    pub fn a(&self) -> f64 {
        self.a
    }
    /// Get the b-axis dimension
    pub fn b(&self) -> f64 {
        self.b
    }
    /// Get the c-axis dimension
    pub fn c(&self) -> f64 {
        self.c
    }
    /// Get the alpha angle in degrees
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    /// Get the beta angle in degrees
    pub fn beta(&self) -> f64 {
        self.beta
    }
    /// Get the gamma angle in degrees
    pub fn gamma(&self) -> f64 {
        self.gamma
    }
    /// Get the dimensions in a tuple (a, b, c)
    pub fn size(&self) -> (f64, f64, f64) {
        (self.a, self.b, self.c)
    }
}

impl PartialEq for UnitCell {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
            && self.alpha == other.alpha
            && self.beta == other.beta
            && self.gamma == other.gamma
    }
}
