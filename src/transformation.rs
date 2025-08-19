#![allow(dead_code)]

/// A 3D affine transformation matrix
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TransformationMatrix {
    /// The matrix itself
    matrix: [[f64; 4]; 3],
}

impl TransformationMatrix {
    /// Get the raw matrix (row major order)
    #[must_use]
    pub const fn matrix(&self) -> [[f64; 4]; 3] {
        self.matrix
    }

    /// Get the raw matrix (row major order)
    #[must_use]
    pub fn matrix_mut(&mut self) -> &mut [[f64; 4]; 3] {
        &mut self.matrix
    }

    /// Set the raw matrix (row major order), the user needs to make sure the matrix is valid
    pub fn set_matrix(&mut self, new_matrix: [[f64; 4]; 3]) {
        self.matrix = new_matrix;
    }

    /// Create a matrix defining identity, so no transformation
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }
    }

    /// Create a matrix with the given matrix
    #[must_use]
    pub const fn from_matrix(matrix: [[f64; 4]; 3]) -> Self {
        Self { matrix }
    }

    /// Create a matrix defining a rotation around the X axis
    /// ## Arguments
    /// * `deg` the rotation in degrees
    /// ## Panics
    /// It panics if `deg` is not finite (`f64.is_finite()`)
    #[must_use]
    pub fn rotation_x(deg: f64) -> Self {
        assert!(deg.is_finite(), "The amount of degrees is not finite");
        let (s, c) = deg.to_radians().sin_cos();
        Self {
            matrix: [[1.0, 0.0, 0.0, 0.0], [0.0, c, -s, 0.0], [0.0, s, c, 0.0]],
        }
    }

    /// Create a matrix defining a rotation around the Y axis
    /// ## Arguments
    /// * `deg` the rotation in degrees
    /// ## Panics
    /// It panics if `deg` is not finite (`f64.is_finite()`)
    #[must_use]
    pub fn rotation_y(deg: f64) -> Self {
        assert!(deg.is_finite(), "The amount of degrees is not finite");
        let (s, c) = deg.to_radians().sin_cos();
        Self {
            matrix: [[c, 0.0, s, 0.0], [0.0, 1.0, 0.0, 0.0], [-s, 0.0, c, 0.0]],
        }
    }

    /// Create a matrix defining a rotation around the Z axis
    /// ## Arguments
    /// * `deg` the rotation in degrees
    /// ## Panics
    /// It panics if `deg` is not finite (`f64.is_finite()`)
    #[must_use]
    pub fn rotation_z(deg: f64) -> Self {
        assert!(deg.is_finite(), "The amount of degrees is not finite");
        let c = deg.to_radians().cos();
        let s = deg.to_radians().sin();
        Self {
            matrix: [[c, -s, 0.0, 0.0], [s, c, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
        }
    }

    /// Create a matrix defining a translation
    /// ## Panics
    /// It panics if any of the arguments is not finite (`f64.is_finite()`)
    #[must_use]
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        assert!(
            x.is_finite() && y.is_finite() && z.is_finite(),
            "One or more of the arguments is not finite"
        );
        Self {
            matrix: [[1.0, 0.0, 0.0, x], [0.0, 1.0, 0.0, y], [0.0, 0.0, 1.0, z]],
        }
    }

    /// Create a matrix defining a magnification
    /// ## Arguments
    /// * `f` the factor where 1.0 is the original size
    /// ## Panics
    /// It panics if `f` is not finite (`f64.is_finite()`)
    #[must_use]
    pub fn magnify(f: f64) -> Self {
        assert!(f.is_finite(), "The factor is not finite");
        Self {
            matrix: [[f, 0.0, 0.0, 0.0], [0.0, f, 0.0, 0.0], [0.0, 0.0, f, 0.0]],
        }
    }

    /// Create a matrix defining a magnification with three different factors
    /// ## Arguments
    /// * `x` the factor for the x dimension where 1.0 is the original size
    /// * `y` the factor for the y dimension where 1.0 is the original size
    /// * `z` the factor for the z dimension where 1.0 is the original size
    /// ## Panics
    /// It panics if any of the arguments is not finite (`f64.is_finite()`)
    #[must_use]
    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        assert!(
            x.is_finite() && y.is_finite() && z.is_finite(),
            "One or more of the arguments is not finite"
        );
        Self {
            matrix: [[x, 0.0, 0.0, 0.0], [0.0, y, 0.0, 0.0], [0.0, 0.0, z, 0.0]],
        }
    }

    /// This multiplies the translation with the given factors, this can be used to
    /// convert fractional units into absolute units.
    pub fn multiply_translation(&mut self, factors: (f64, f64, f64)) {
        self.matrix[0][3] *= factors.0;
        self.matrix[1][3] *= factors.1;
        self.matrix[2][3] *= factors.2;
    }

    /// Apply this transformation to the given position.
    /// It returns the new position.
    /// ## Arguments
    /// * `pos` the position (x, y, z)
    #[must_use]
    pub fn apply(&self, pos: (f64, f64, f64)) -> (f64, f64, f64) {
        (
            pos.2.mul_add(
                self.matrix[0][2],
                pos.0.mul_add(self.matrix[0][0], pos.1 * self.matrix[0][1]),
            ) + self.matrix[0][3],
            pos.2.mul_add(
                self.matrix[1][2],
                pos.0.mul_add(self.matrix[1][0], pos.1 * self.matrix[1][1]),
            ) + self.matrix[1][3],
            pos.2.mul_add(
                self.matrix[2][2],
                pos.0.mul_add(self.matrix[2][0], pos.1 * self.matrix[2][1]),
            ) + self.matrix[2][3],
        )
    }

    /// Combine this transformation with another transformation to deliver a new transformation.
    /// This transformation is applied before the other transformation.
    #[must_use]
    pub fn combine(&self, other: &Self) -> Self {
        Self {
            matrix: [
                [
                    other.matrix[0][2].mul_add(
                        self.matrix[2][0],
                        other.matrix[0][0]
                            .mul_add(self.matrix[0][0], other.matrix[0][1] * self.matrix[1][0]),
                    ),
                    other.matrix[0][2].mul_add(
                        self.matrix[2][1],
                        other.matrix[0][0]
                            .mul_add(self.matrix[0][1], other.matrix[0][1] * self.matrix[1][1]),
                    ),
                    other.matrix[0][2].mul_add(
                        self.matrix[2][2],
                        other.matrix[0][0]
                            .mul_add(self.matrix[0][2], other.matrix[0][1] * self.matrix[1][2]),
                    ),
                    other.matrix[0][2].mul_add(
                        self.matrix[2][3],
                        other.matrix[0][0]
                            .mul_add(self.matrix[0][3], other.matrix[0][1] * self.matrix[1][3]),
                    ) + other.matrix[0][3],
                ],
                [
                    other.matrix[1][2].mul_add(
                        self.matrix[2][0],
                        other.matrix[1][0]
                            .mul_add(self.matrix[0][0], other.matrix[1][1] * self.matrix[1][0]),
                    ),
                    other.matrix[1][2].mul_add(
                        self.matrix[2][1],
                        other.matrix[1][0]
                            .mul_add(self.matrix[0][1], other.matrix[1][1] * self.matrix[1][1]),
                    ),
                    other.matrix[1][2].mul_add(
                        self.matrix[2][2],
                        other.matrix[1][0]
                            .mul_add(self.matrix[0][2], other.matrix[1][1] * self.matrix[1][2]),
                    ),
                    other.matrix[1][2].mul_add(
                        self.matrix[2][3],
                        other.matrix[1][0]
                            .mul_add(self.matrix[0][3], other.matrix[1][1] * self.matrix[1][3]),
                    ) + other.matrix[1][3],
                ],
                [
                    other.matrix[2][2].mul_add(
                        self.matrix[2][0],
                        other.matrix[2][0]
                            .mul_add(self.matrix[0][0], other.matrix[2][1] * self.matrix[1][0]),
                    ),
                    other.matrix[2][2].mul_add(
                        self.matrix[2][1],
                        other.matrix[2][0]
                            .mul_add(self.matrix[0][1], other.matrix[2][1] * self.matrix[1][1]),
                    ),
                    other.matrix[2][2].mul_add(
                        self.matrix[2][2],
                        other.matrix[2][0]
                            .mul_add(self.matrix[0][2], other.matrix[2][1] * self.matrix[1][2]),
                    ),
                    other.matrix[2][2].mul_add(
                        self.matrix[2][3],
                        other.matrix[2][0]
                            .mul_add(self.matrix[0][3], other.matrix[2][1] * self.matrix[1][3]),
                    ) + other.matrix[2][3],
                ],
            ],
        }
    }
}

#[cfg(test)]
#[allow(clippy::print_stdout, clippy::use_debug)]
mod tests {
    use super::TransformationMatrix;

    #[test]
    fn identity() {
        let pos = (1.0, 1.0, 1.0);
        let new_pos = TransformationMatrix::identity().apply(pos);
        assert_eq!(pos, new_pos);
    }

    #[test]
    fn combination() {
        let pos = (10.0, 0.0, 0.0);
        let new_pos = TransformationMatrix::rotation_y(90.0)
            .combine(&TransformationMatrix::translation(0.0, 0.0, 10.0))
            .apply(pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 0.0)));
    }
    #[test]
    fn rot_x() {
        // 90 deg y
        let mut pos = (0.0, 10.0, 0.0);
        let mut new_pos = TransformationMatrix::rotation_x(90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, 10.0)));
        // 90 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_x(90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, -10.0, 0.0)));
        // -90 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_x(-90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 10.0, 0.0)));
        // 180 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_x(180.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // -180 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_x(-180.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // 360 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_x(360.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, 10.0)));
        // 44.5 deg z
        pos = (0.0, 0.0, -1.0);
        new_pos = TransformationMatrix::rotation_x(44.5).apply(pos);
        let end = (
            0.0,
            44.5_f64.to_radians().sin(),
            (-44.5_f64).to_radians().cos(),
        );
        println!("{new_pos:?} vs {end:?}");
        assert!(close_tuple(new_pos, end));
        // 44.5 + 45.5 deg z
        pos = (0.0, 0.0, -1.0);
        new_pos = TransformationMatrix::rotation_x(44.5)
            .combine(&TransformationMatrix::rotation_x(45.5))
            .apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 1.0, 0.0)));
    }
    #[test]
    fn rot_y() {
        // 90 deg x
        let mut pos = (10.0, 0.0, 0.0);
        let mut new_pos = TransformationMatrix::rotation_y(90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // 90 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_y(90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (10.0, 0.0, 0.0)));
        // -90 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_y(-90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (-10.0, 0.0, 0.0)));
        // 180 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_y(180.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // -180 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_y(-180.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // 360 deg z
        pos = (0.0, 0.0, 10.0);
        new_pos = TransformationMatrix::rotation_y(360.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 0.0, 10.0)));
        // 44.5 deg z
        pos = (0.0, 0.0, -1.0);
        new_pos = TransformationMatrix::rotation_y(44.5).apply(pos);
        let end = (
            (-44.5_f64).to_radians().sin(),
            0.0,
            (-44.5_f64).to_radians().cos(),
        );
        println!("{new_pos:?} vs {end:?}");
        assert!(close_tuple(new_pos, end));
        // 44.5 + 45.5 deg z
        pos = (0.0, 0.0, -1.0);
        new_pos = TransformationMatrix::rotation_y(44.5)
            .combine(&TransformationMatrix::rotation_y(45.5))
            .apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (-1.0, 0.0, 0.0)));
    }

    #[test]
    fn rot_z() {
        // 90 deg x
        let mut pos = (10.0, 0.0, 0.0);
        let mut new_pos = TransformationMatrix::rotation_z(90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, 10.0, 0.0)));
        // 90 deg y
        pos = (0.0, 10.0, 0.0);
        new_pos = TransformationMatrix::rotation_z(90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (-10.0, 0.0, 0.0)));
        // -90 deg y
        pos = (0.0, 10.0, 0.0);
        new_pos = TransformationMatrix::rotation_z(-90.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (10.0, 0.0, 0.0)));
        // 180 deg y
        pos = (0.0, 10.0, 0.0);
        new_pos = TransformationMatrix::rotation_z(180.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, -10.0, 0.0)));
        // -180 deg y
        pos = (0.0, 10.0, 0.0);
        new_pos = TransformationMatrix::rotation_z(-180.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (0.0, -10.0, 0.0)));
        // 360 deg x
        pos = (10.0, 0.0, 0.0);
        new_pos = TransformationMatrix::rotation_z(360.0).apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (10.0, 0.0, 0.0)));
        // 44.5 deg y
        pos = (0.0, -1.0, 0.0);
        new_pos = TransformationMatrix::rotation_z(44.5).apply(pos);
        let end = (
            44.5_f64.to_radians().sin(),
            (-44.5_f64).to_radians().cos(),
            0.0,
        );
        println!("{new_pos:?} vs {end:?}");
        assert!(close_tuple(new_pos, end));
        // 44.5 + 45.5 deg y
        pos = (0.0, -1.0, 0.0);
        new_pos = TransformationMatrix::rotation_z(44.5)
            .combine(&TransformationMatrix::rotation_z(45.5))
            .apply(pos);
        println!("{new_pos:?}");
        assert!(close_tuple(new_pos, (1.0, 0.0, 0.0)));
    }

    #[test]
    fn translation() {
        let mut pos = (10.0, 0.0, 0.0);
        let mut new_pos = TransformationMatrix::translation(-10.0, 0.0, 0.0).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 0.0)));
        pos = (-897.0, 0.0023, 1.0);
        new_pos = TransformationMatrix::translation(897.0, -0.0023, -1.0).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 0.0)));
        pos = (0.0, 0.0, 0.0);
        new_pos = TransformationMatrix::translation(0.0, 0.0, 0.0).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 0.0)));
    }

    #[test]
    fn magnification() {
        let mut pos = (10.0, 0.0, 0.0);
        let mut new_pos = TransformationMatrix::magnify(10.0).apply(pos);
        assert!(close_tuple(new_pos, (100.0, 0.0, 0.0)));
        pos = (-897.0, 0.0023, 1.0);
        new_pos = TransformationMatrix::magnify(0.1).apply(pos);
        assert!(close_tuple(new_pos, (-89.7, 0.00023, 0.1)));
        pos = (0.0, 1.0, 0.0);
        new_pos = TransformationMatrix::magnify(2.5).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 2.5, 0.0)));
    }

    #[test]
    fn multiply_translation() {
        let pos = (0.0, 0.0, 0.0);
        let mut matrix = TransformationMatrix::translation(1.0, 2.0, -5.0);
        matrix.multiply_translation((10.0, 5.0, -2.0));
        let new_pos = matrix.apply(pos);
        assert!(close_tuple(new_pos, (10.0, 10.0, 10.0)));
        assert_eq!(
            matrix.matrix(),
            TransformationMatrix::translation(10.0, 10.0, 10.0).matrix()
        );
    }

    #[test]
    fn matrix() {
        let normal = TransformationMatrix::rotation_x(45.0);
        let raw = normal.matrix();
        let from_matrix = TransformationMatrix::from_matrix(raw);
        let mut set = TransformationMatrix::identity();
        set.set_matrix(raw);
        assert_eq!(normal, from_matrix);
        assert_eq!(from_matrix, set);
        assert_eq!(normal, set);
    }

    fn close_tuple(a: (f64, f64, f64), b: (f64, f64, f64)) -> bool {
        close(a.0, b.0) && close(a.1, b.1) && close(a.2, b.2)
    }

    fn close(a: f64, b: f64) -> bool {
        #[allow(clippy::float_cmp)]
        if a == b {
            true
        } else if a == 0.0 || b == 0.0 {
            (a - b) > -0.000_000_000_000_01 && (b - a) < 0.000_000_000_000_01
        } else {
            let dif = a / b;
            (1.0 - dif) > -0.000_000_000_000_01 && (dif - 1.0) < 0.000_000_000_000_01
        }
    }
}
