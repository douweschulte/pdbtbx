#![allow(dead_code)]

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
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
    /// Set the a-axis dimension
    /// ## Panics
    /// It panics if the new value is not finite
    pub fn set_a(&mut self, new_a: f64) {
        if !new_a.is_finite() {
            panic!("The new a value of this UnitCell is not finite");
        }
        self.a = new_a;
    }
    /// Set the b-axis dimension
    /// ## Panics
    /// It panics if the new value is not finite
    pub fn set_b(&mut self, new_b: f64) {
        if !new_b.is_finite() {
            panic!("The new b value of this UnitCell is not finite");
        }
        self.b = new_b;
    }
    /// Set the c-axis dimension
    /// ## Panics
    /// It panics if the new value is not finite
    pub fn set_c(&mut self, new_c: f64) {
        if !new_c.is_finite() {
            panic!("The new c value of this UnitCell is not finite");
        }
        self.c = new_c;
    }
    /// Get the alpha angle in degrees
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    /// Set the alpha angle in degrees
    /// ## Panics
    /// It panics if the new value is not finite.
    /// It also panics if the alpha value is outside of bounds [0, 360)
    pub fn set_alpha(&mut self, new_alpha: f64) {
        if !new_alpha.is_finite() {
            panic!("The new alpha value of this UnitCell is not finite");
        }
        if !(0.0..360.0).contains(&new_alpha) {
            panic!("The new alpha value of this UnitCell is out of bounds [0, 360).")
        }
        self.alpha = new_alpha;
    }
    /// Get the beta angle in degrees
    pub fn beta(&self) -> f64 {
        self.beta
    }
    /// Set the beta angle in degrees
    /// ## Panics
    /// It panics if the new value is not finite.
    /// It also panics if the beta value is outside of bounds [0, 360)
    pub fn set_beta(&mut self, new_beta: f64) {
        if !new_beta.is_finite() {
            panic!("The new beta value of this UnitCell is not finite");
        }
        if !(0.0..360.0).contains(&new_beta) {
            panic!("The new beta value of this UnitCell is out of bounds [0, 360).")
        }
        self.beta = new_beta;
    }
    /// Get the gamma angle in degrees
    pub fn gamma(&self) -> f64 {
        self.gamma
    }
    /// Set the gamma angle in degrees
    /// ## Panics
    /// It panics if the new value is not finite.
    /// It also panics if the gamma value is outside of bounds [0, 360)
    pub fn set_gamma(&mut self, new_gamma: f64) {
        if !new_gamma.is_finite() {
            panic!("The new gamma value of this UnitCell is not finite");
        }
        if !(0.0..360.0).contains(&new_gamma) {
            panic!("The new gamma value of this UnitCell is out of bounds [0, 360).")
        }
        self.gamma = new_gamma;
    }
    /// Get the dimensions in a tuple (a, b, c)
    pub fn size(&self) -> (f64, f64, f64) {
        (self.a, self.b, self.c)
    }
}

impl Default for UnitCell {
    /// Default UnitCell with all sizes set to 0.0 and angles to 90.0
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 90.0, 90.0, 90.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let a = UnitCell::new(10.0, 10.0, 15.0, 90.0, 90.0, 87.0);
        let b = UnitCell::new(10.0, 10.0, 15.0, 90.0, 90.0, 87.0);
        let c = UnitCell::new(12.0, 10.0, 15.0, 90.0, 90.0, 87.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
