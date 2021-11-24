use super::{Point, Quaternion, Vector};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub values: [[f32; 4]; 4],
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            values: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn scale(s: f32) -> Matrix {
        Matrix {
            values: [
                [s, 0., 0., 0.],
                [0., s, 0., 0.],
                [0., 0., s, 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn translate(v: Vector) -> Matrix {
        Matrix {
            values: [
                [0., 0., 0., v.x],
                [0., 0., 0., v.y],
                [0., 0., 0., v.z],
                [0., 0., 0., 1.],
            ],
        }
    }

    #[allow(unused_variables)]
    pub fn from_quaternion(q: Quaternion) -> Matrix {
        unimplemented!();
    }

    #[allow(unused_variables)]
    pub fn rotate_along(axis: Vector, angle: f32) -> Matrix {
        unimplemented!();
    }

    #[allow(unused_variables)]
    pub fn lookat(pos: Point, dir: Vector, up: Vector) -> Matrix {
        unimplemented!();
    }
}
