use std::ops::{Mul};

#[derive(Debug)]
pub struct Mat4 {
    pub a00: f32, pub a01: f32, pub a02: f32, pub a03: f32,
    pub a10: f32, pub a11: f32, pub a12: f32, pub a13: f32,
    pub a20: f32, pub a21: f32, pub a22: f32, pub a23: f32,
    pub a30: f32, pub a31: f32, pub a32: f32, pub a33: f32,
}

impl Mat4 {
    pub fn new(a00: f32, a01: f32, a02: f32, a03: f32,
               a10: f32, a11: f32, a12: f32, a13: f32,
               a20: f32, a21: f32, a22: f32, a23: f32,
               a30: f32, a31: f32, a32: f32, a33: f32) -> Self {
        Mat4 {
            a00, a01, a02, a03,
            a10, a11, a12, a13,
            a20, a21, a22, a23,
            a30, a31, a32, a33,
        }
    }

    pub fn multiply(m1: &Mat4, m2: &Mat4) -> Self {
        Mat4 {
            a00 : m1.a00 * m2.a00 + m1.a01 * m2.a10 + m1.a02 * m2.a20 +
                  m1.a03 * m2.a30,
            a01 : m1.a00 * m2.a01 + m1.a01 * m2.a11 + m1.a02 * m2.a21 +
                  m1.a03 * m2.a31,
            a02 : m1.a00 * m2.a02 + m1.a01 * m2.a12 + m1.a02 * m2.a22 +
                  m1.a03 * m2.a32,
            a03 : m1.a00 * m2.a03 + m1.a01 * m2.a13 + m1.a02 * m2.a23 +
                  m1.a03 * m2.a33, 

            a10 : m1.a10 * m2.a00 + m1.a11 * m2.a10 + m1.a12 * m2.a20 +
                  m1.a13 * m2.a30,
            a11 : m1.a10 * m2.a01 + m1.a11 * m2.a11 + m1.a12 * m2.a21 +
                  m1.a13 * m2.a31,
            a12 : m1.a10 * m2.a02 + m1.a11 * m2.a12 + m1.a12 * m2.a22 +
                  m1.a13 * m2.a32,
            a13 : m1.a10 * m2.a03 + m1.a11 * m2.a13 + m1.a12 * m2.a23 +
                  m1.a13 * m2.a33, 

            a20 : m1.a20 * m2.a00 + m1.a21 * m2.a10 + m1.a22 * m2.a20 +
                  m1.a23 * m2.a30,
            a21 : m1.a20 * m2.a01 + m1.a21 * m2.a11 + m1.a22 * m2.a21 +
                  m1.a23 * m2.a31,
            a22 : m1.a20 * m2.a02 + m1.a21 * m2.a12 + m1.a22 * m2.a22 +
                  m1.a23 * m2.a32,
            a23 : m1.a20 * m2.a03 + m1.a21 * m2.a13 + m1.a22 * m2.a23 +
                  m1.a23 * m2.a33, 

            a30 : m1.a30 * m2.a00 + m1.a31 * m2.a10 + m1.a32 * m2.a20 +
                  m1.a33 * m2.a30,
            a31 : m1.a30 * m2.a01 + m1.a31 * m2.a11 + m1.a32 * m2.a21 +
                  m1.a33 * m2.a31,
            a32 : m1.a30 * m2.a02 + m1.a31 * m2.a12 + m1.a32 * m2.a22 +
                  m1.a33 * m2.a32,
            a33 : m1.a30 * m2.a03 + m1.a31 * m2.a13 + m1.a32 * m2.a23 +
                  m1.a33 * m2.a33, 
        }
    }
}

impl<'a, 'b>  Mul<&'b Mat4> for &'a Mat4 {
    type Output = Mat4;
    
    fn mul(self, rhs: &'b Mat4) -> Self::Output {
       Mat4::multiply(self, rhs)
    }
}
