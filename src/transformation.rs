#![allow(dead_code)]

#[derive(Debug)]
pub struct TransformationMatrix {
    matrix: [[f64; 4]; 4],
}

impl TransformationMatrix {
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
    pub fn rotation_x(deg: f64) -> Self {
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
    pub fn rotation_y(deg: f64) -> Self {
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
    pub fn rotation_z(deg: f64) -> Self {
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
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        TransformationMatrix {
            matrix: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn magnify(f: f64) -> Self {
        TransformationMatrix {
            matrix: [
                [f, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, f, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
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
