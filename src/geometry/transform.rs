use super::{Matrix, Vector};

#[derive(Debug, Clone)]
pub struct Transform {
    pub m: Matrix,
    pub m_inv: Matrix,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            m: Matrix::identity(),
            m_inv: Matrix::identity(),
        }
    }

    pub fn scale(s: f32) -> Transform {
        Transform {
            m: Matrix::scale(s),
            m_inv: Matrix::scale(1. / s),
        }
    }

    pub fn translate(v: Vector) -> Transform {
        Transform {
            m: Matrix::translate(v),
            m_inv: Matrix::translate(-v),
        }
    }

    pub fn reverse(&self) -> Transform {
        Transform {
            m: self.m_inv.clone(),
            m_inv: self.m.clone(),
        }
    }
}
