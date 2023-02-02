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
    /// Create a new `UnitCell` construct.
    /// ## Arguments
    /// * `a` - a-axis dimension
    /// * `b` - b-axis dimension
    /// * `c` - c-axis dimension
    /// * `alpha` - alpha angle in degrees
    /// * `beta` - beta angle in degrees
    /// * `gamma` - gamma angle in degrees
    #[must_use]
    pub const fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }
    /// Get the a-axis dimension
    #[must_use]
    pub const fn a(&self) -> f64 {
        self.a
    }
    /// Get the b-axis dimension
    #[must_use]
    pub const fn b(&self) -> f64 {
        self.b
    }
    /// Get the c-axis dimension
    #[must_use]
    pub const fn c(&self) -> f64 {
        self.c
    }
    /// Set the a-axis dimension
    /// ## Panics
    /// It panics if the new value is not finite
    pub fn set_a(&mut self, new_a: f64) {
        assert!(
            new_a.is_finite(),
            "The new a value of this UnitCell is not finite. Value: {new_a}"
        );
        self.a = new_a;
    }
    /// Set the b-axis dimension
    /// ## Panics
    /// It panics if the new value is not finite
    pub fn set_b(&mut self, new_b: f64) {
        assert!(
            new_b.is_finite(),
            "The new b value of this UnitCell is not finite. Value: {new_b}"
        );
        self.b = new_b;
    }
    /// Set the c-axis dimension
    /// ## Panics
    /// It panics if the new value is not finite
    pub fn set_c(&mut self, new_c: f64) {
        assert!(
            new_c.is_finite(),
            "The new c value of this UnitCell is not finite. Value: {new_c}"
        );
        self.c = new_c;
    }
    /// Get the alpha angle in degrees
    #[must_use]
    pub const fn alpha(&self) -> f64 {
        self.alpha
    }
    /// Set the alpha angle in degrees
    /// ## Panics
    /// It panics if the new value is not finite.
    /// It also panics if the alpha value is outside of bounds [0, 360)
    pub fn set_alpha(&mut self, new_alpha: f64) {
        assert!(
            new_alpha.is_finite(),
            "The new alpha value of this UnitCell is not finite. Value: {new_alpha}"
        );
        assert!(
            (0.0..360.0).contains(&new_alpha),
            "The new alpha value of this UnitCell is out of bounds [0, 360). Value: {new_alpha}"
        );
        self.alpha = new_alpha;
    }
    /// Get the beta angle in degrees
    #[must_use]
    pub const fn beta(&self) -> f64 {
        self.beta
    }
    /// Set the beta angle in degrees
    /// ## Panics
    /// It panics if the new value is not finite.
    /// It also panics if the beta value is outside of bounds [0, 360)
    pub fn set_beta(&mut self, new_beta: f64) {
        assert!(
            new_beta.is_finite(),
            "The new beta value of this UnitCell is not finite. Value: {new_beta}"
        );
        assert!(
            (0.0..360.0).contains(&new_beta),
            "The new beta value of this UnitCell is out of bounds [0, 360).. Value: {new_beta}"
        );
        self.beta = new_beta;
    }
    /// Get the gamma angle in degrees
    #[must_use]
    pub const fn gamma(&self) -> f64 {
        self.gamma
    }
    /// Set the gamma angle in degrees
    /// ## Panics
    /// It panics if the new value is not finite.
    /// It also panics if the gamma value is outside of bounds [0, 360)
    pub fn set_gamma(&mut self, new_gamma: f64) {
        assert!(
            new_gamma.is_finite(),
            "The new gamma value of this UnitCell is not finite. Value: {new_gamma}"
        );
        assert!(
            (0.0..360.0).contains(&new_gamma),
            "The new gamma value of this UnitCell is out of bounds [0, 360).. Value: {new_gamma}"
        );
        self.gamma = new_gamma;
    }
    /// Get the dimensions in a tuple (a, b, c)
    #[must_use]
    pub const fn size(&self) -> (f64, f64, f64) {
        (self.a, self.b, self.c)
    }
}

impl Default for UnitCell {
    /// Default `UnitCell` with all sizes set to 0.0 and angles to 90.0
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
