#![allow(dead_code)]

/// A 3D affine transformation matrix
#[derive(Debug)]
pub struct TransformationMatrix {
    matrix: [[f64; 4]; 4],
}

impl TransformationMatrix {
    /// Get the raw matrix (row major order)
    pub fn matrix(&self) -> [[f64; 4]; 4] {
        self.matrix
    }

    /// Set the raw matrix (row major order), the user needs to make sure the matrix is valid
    pub fn set_matrix(&mut self, new_matrix: [[f64; 4]; 4]) {
        self.matrix = new_matrix;
    }

    /// Create a matrix defining identity, so no transformation
    pub fn identity() -> Self {
        TransformationMatrix {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a matrix with the given matrix
    pub fn from_matrix(matrix: [[f64; 4]; 4]) -> Self {
        TransformationMatrix { matrix: matrix }
    }

    /// Create a matrix defining a rotation around the X axis
    /// ## Arguments
    /// * `deg` the rotation in degrees
    /// ## Panics
    /// It panics if `deg` is not finite (`f64.is_finite()`)
    pub fn rotation_x(deg: f64) -> Self {
        if !deg.is_finite() {
            panic!("The amount of degrees is not finite");
        }
        let c = deg.to_radians().cos();
        let s = deg.to_radians().sin();
        TransformationMatrix {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a matrix defining a rotation around the Y axis
    /// ## Arguments
    /// * `deg` the rotation in degrees
    /// ## Panics
    /// It panics if `deg` is not finite (`f64.is_finite()`)
    pub fn rotation_y(deg: f64) -> Self {
        if !deg.is_finite() {
            panic!("The amount of degrees is not finite");
        }
        let c = deg.to_radians().cos();
        let s = deg.to_radians().sin();
        TransformationMatrix {
            matrix: [
                [c, 0.0, s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a matrix defining a rotation around the Z axis
    /// ## Arguments
    /// * `deg` the rotation in degrees
    /// ## Panics
    /// It panics if `deg` is not finite (`f64.is_finite()`)
    pub fn rotation_z(deg: f64) -> Self {
        if !deg.is_finite() {
            panic!("The amount of degrees is not finite");
        }
        let c = deg.to_radians().cos();
        let s = deg.to_radians().sin();
        TransformationMatrix {
            matrix: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a matrix defining a translation
    /// ## Panics
    /// It panics if any of the arguments is not finite (`f64.is_finite()`)
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            panic!("One or more of the arguments is not finite");
        }
        TransformationMatrix {
            matrix: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Create a matrix defining a magnification
    /// ## Arguments
    /// * `f` the factor where 1.0 is the original size
    /// ## Panics
    /// It panics if `f` is not finite (`f64.is_finite()`)
    pub fn magnify(f: f64) -> Self {
        if !f.is_finite() {
            panic!("The amount of degrees is not finite");
        }
        TransformationMatrix {
            matrix: [
                [f, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, f, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Apply this transformation to the given position.
    /// It returns the new position.
    /// ## Arguments
    /// * `pos` the position (x, y, z)
    pub fn apply(&self, pos: (f64, f64, f64)) -> (f64, f64, f64) {
        (
            pos.0 * self.matrix[0][0]
                + pos.1 * self.matrix[0][1]
                + pos.2 * self.matrix[0][2]
                + self.matrix[0][3],
            pos.0 * self.matrix[1][0]
                + pos.1 * self.matrix[1][1]
                + pos.2 * self.matrix[1][2]
                + self.matrix[1][3],
            pos.0 * self.matrix[2][0]
                + pos.1 * self.matrix[2][1]
                + pos.2 * self.matrix[2][2]
                + self.matrix[2][3],
        )
    }

    /// Combine this transformation with another transformation to deliver a new transformation.
    /// This transformation is applied before the other transformation.
    pub fn combine(&self, other: &Self) -> Self {
        TransformationMatrix {
            matrix: [
                [
                    other.matrix[0][0] * self.matrix[0][0]
                        + other.matrix[0][1] * self.matrix[1][0]
                        + other.matrix[0][2] * self.matrix[2][0]
                        + other.matrix[0][3] * self.matrix[3][0],
                    other.matrix[0][0] * self.matrix[0][1]
                        + other.matrix[0][1] * self.matrix[1][1]
                        + other.matrix[0][2] * self.matrix[2][1]
                        + other.matrix[0][3] * self.matrix[3][1],
                    other.matrix[0][0] * self.matrix[0][2]
                        + other.matrix[0][1] * self.matrix[1][2]
                        + other.matrix[0][2] * self.matrix[2][2]
                        + other.matrix[0][3] * self.matrix[3][2],
                    other.matrix[0][0] * self.matrix[0][3]
                        + other.matrix[0][1] * self.matrix[1][3]
                        + other.matrix[0][2] * self.matrix[2][3]
                        + other.matrix[0][3] * self.matrix[3][3],
                ],
                [
                    other.matrix[1][0] * self.matrix[0][0]
                        + other.matrix[1][1] * self.matrix[1][0]
                        + other.matrix[1][2] * self.matrix[2][0]
                        + other.matrix[1][3] * self.matrix[3][0],
                    other.matrix[1][0] * self.matrix[0][1]
                        + other.matrix[1][1] * self.matrix[1][1]
                        + other.matrix[1][2] * self.matrix[2][1]
                        + other.matrix[1][3] * self.matrix[3][1],
                    other.matrix[1][0] * self.matrix[0][2]
                        + other.matrix[1][1] * self.matrix[1][2]
                        + other.matrix[1][2] * self.matrix[2][2]
                        + other.matrix[1][3] * self.matrix[3][2],
                    other.matrix[1][0] * self.matrix[0][3]
                        + other.matrix[1][1] * self.matrix[1][3]
                        + other.matrix[1][2] * self.matrix[2][3]
                        + other.matrix[1][3] * self.matrix[3][3],
                ],
                [
                    other.matrix[2][0] * self.matrix[0][0]
                        + other.matrix[2][1] * self.matrix[1][0]
                        + other.matrix[2][2] * self.matrix[2][0]
                        + other.matrix[2][3] * self.matrix[3][0],
                    other.matrix[2][0] * self.matrix[0][1]
                        + other.matrix[2][1] * self.matrix[1][1]
                        + other.matrix[2][2] * self.matrix[2][1]
                        + other.matrix[2][3] * self.matrix[3][1],
                    other.matrix[2][0] * self.matrix[0][2]
                        + other.matrix[2][1] * self.matrix[1][2]
                        + other.matrix[2][2] * self.matrix[2][2]
                        + other.matrix[2][3] * self.matrix[3][2],
                    other.matrix[2][0] * self.matrix[0][3]
                        + other.matrix[2][1] * self.matrix[1][3]
                        + other.matrix[2][2] * self.matrix[2][3]
                        + other.matrix[2][3] * self.matrix[3][3],
                ],
                [
                    other.matrix[3][0] * self.matrix[0][0]
                        + other.matrix[3][1] * self.matrix[1][0]
                        + other.matrix[3][2] * self.matrix[2][0]
                        + other.matrix[3][3] * self.matrix[3][0],
                    other.matrix[3][0] * self.matrix[0][1]
                        + other.matrix[3][1] * self.matrix[1][1]
                        + other.matrix[3][2] * self.matrix[2][1]
                        + other.matrix[3][3] * self.matrix[3][1],
                    other.matrix[3][0] * self.matrix[0][2]
                        + other.matrix[3][1] * self.matrix[1][2]
                        + other.matrix[3][2] * self.matrix[2][2]
                        + other.matrix[3][3] * self.matrix[3][2],
                    other.matrix[3][0] * self.matrix[0][3]
                        + other.matrix[3][1] * self.matrix[1][3]
                        + other.matrix[3][2] * self.matrix[2][3]
                        + other.matrix[3][3] * self.matrix[3][3],
                ],
            ],
        }
    }
}

impl Clone for TransformationMatrix {
    fn clone(&self) -> Self {
        let mut orig = TransformationMatrix::identity();

        orig.matrix = self.matrix.clone();

        orig
    }
}

#[cfg(test)]
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
        let pos = (0.0, 10.0, 0.0);
        let new_pos = TransformationMatrix::rotation_x(90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 10.0)));
        // 90 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_x(90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, -10.0, 0.0)));
        // -90 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_x(-90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 10.0, 0.0)));
        // 180 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_x(180.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // -180 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_x(-180.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // 360 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_x(360.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 10.0)));
        // 44.5 deg z
        let pos = (0.0, 0.0, -1.0);
        let new_pos = TransformationMatrix::rotation_x(44.5).apply(pos);
        let end = (
            0.0,
            (44.5 as f64).to_radians().sin(),
            -(44.5 as f64).to_radians().cos(),
        );
        println!("{:?} vs {:?}", new_pos, end);
        assert!(close_tuple(new_pos, end));
        // 44.5 + 45.5 deg z
        let pos = (0.0, 0.0, -1.0);
        let new_pos = TransformationMatrix::rotation_x(44.5)
            .combine(&TransformationMatrix::rotation_x(45.5))
            .apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 1.0, 0.0)));
    }
    #[test]
    fn rot_y() {
        // 90 deg x
        let pos = (10.0, 0.0, 0.0);
        let new_pos = TransformationMatrix::rotation_y(90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // 90 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_y(90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (10.0, 0.0, 0.0)));
        // -90 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_y(-90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (-10.0, 0.0, 0.0)));
        // 180 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_y(180.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // -180 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_y(-180.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, -10.0)));
        // 360 deg z
        let pos = (0.0, 0.0, 10.0);
        let new_pos = TransformationMatrix::rotation_y(360.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 10.0)));
        // 44.5 deg z
        let pos = (0.0, 0.0, -1.0);
        let new_pos = TransformationMatrix::rotation_y(44.5).apply(pos);
        let end = (
            -(44.5 as f64).to_radians().sin(),
            0.0,
            -(44.5 as f64).to_radians().cos(),
        );
        println!("{:?} vs {:?}", new_pos, end);
        assert!(close_tuple(new_pos, end));
        // 44.5 + 45.5 deg z
        let pos = (0.0, 0.0, -1.0);
        let new_pos = TransformationMatrix::rotation_y(44.5)
            .combine(&TransformationMatrix::rotation_y(45.5))
            .apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (-1.0, 0.0, 0.0)));
    }

    #[test]
    fn rot_z() {
        // 90 deg x
        let pos = (10.0, 0.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, 10.0, 0.0)));
        // 90 deg y
        let pos = (0.0, 10.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (-10.0, 0.0, 0.0)));
        // -90 deg y
        let pos = (0.0, 10.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(-90.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (10.0, 0.0, 0.0)));
        // 180 deg y
        let pos = (0.0, 10.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(180.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, -10.0, 0.0)));
        // -180 deg y
        let pos = (0.0, 10.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(-180.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (0.0, -10.0, 0.0)));
        // 360 deg x
        let pos = (10.0, 0.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(360.0).apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (10.0, 0.0, 0.0)));
        // 44.5 deg y
        let pos = (0.0, -1.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(44.5).apply(pos);
        let end = (
            (44.5 as f64).to_radians().sin(),
            -(44.5 as f64).to_radians().cos(),
            0.0,
        );
        println!("{:?} vs {:?}", new_pos, end);
        assert!(close_tuple(new_pos, end));
        // 44.5 + 45.5 deg y
        let pos = (0.0, -1.0, 0.0);
        let new_pos = TransformationMatrix::rotation_z(44.5)
            .combine(&TransformationMatrix::rotation_z(45.5))
            .apply(pos);
        println!("{:?}", new_pos);
        assert!(close_tuple(new_pos, (1.0, 0.0, 0.0)));
    }

    #[test]
    fn translation() {
        let pos = (10.0, 0.0, 0.0);
        let new_pos = TransformationMatrix::translation(-10.0, 0.0, 0.0).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 0.0)));
        let pos = (-897.0, 0.0023, 1.0);
        let new_pos = TransformationMatrix::translation(897.0, -0.0023, -1.0).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 0.0)));
        let pos = (0.0, 0.0, 0.0);
        let new_pos = TransformationMatrix::translation(0.0, 0.0, 0.0).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 0.0, 0.0)));
    }

    #[test]
    fn magnification() {
        let pos = (10.0, 0.0, 0.0);
        let new_pos = TransformationMatrix::magnify(10.0).apply(pos);
        assert!(close_tuple(new_pos, (100.0, 0.0, 0.0)));
        let pos = (-897.0, 0.0023, 1.0);
        let new_pos = TransformationMatrix::magnify(0.1).apply(pos);
        assert!(close_tuple(new_pos, (-89.7, 0.00023, 0.1)));
        let pos = (0.0, 1.0, 0.0);
        let new_pos = TransformationMatrix::magnify(2.5).apply(pos);
        assert!(close_tuple(new_pos, (0.0, 2.5, 0.0)));
    }

    fn close_tuple(a: (f64, f64, f64), b: (f64, f64, f64)) -> bool {
        close(a.0, b.0) && close(a.1, b.1) && close(a.2, b.2)
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b) > -0.0000001 && (a - b) < 0.0000001
    }
}
