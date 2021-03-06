use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Index, IndexMut};
use std::ops::{Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zeroes() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn dot(v1: Self, v2: Self) -> f32 {
        assert!(!v1.has_nans() && !v2.has_nans());
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: Self, v2: Self) -> Self {
        assert!(!v1.has_nans() && !v2.has_nans());
        Self {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z, 
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    pub fn normalize(v: Self) -> Self {
        assert!(v.len() != 0.0); 
        v / v.len()
    }

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()    
    }

    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x; 
        self.y = self.y - other.y; 
        self.z = self.z - other.z; 
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
	assert!(rhs != 0.0);
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
	assert!(rhs != 0.0);
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
	    assert!((0..3).contains(&index));
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Idx out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
	    assert!((0..3).contains(&index));
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Idx out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_nans() {
        let v = Vec3::new(0.0, 0.0, f32::NAN);
        assert!(v.has_nans());
    }

    #[test]
    fn lengths() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v.len_sq(), 0.0);
        assert_eq!(v.len(), 0.0);

        let v = Vec3::new(1.0, -2.0, 3.0);
        assert_eq!(v.len_sq(), 14.0);
        assert_eq!(v.len(), 14.0_f32.sqrt());

        // TODO: Deal with numerical inaccuracies - this test returns 0.99... instead of 1.0.
        // assert_eq!(Vec3::normalize(v).len(), 1.0);
    }

    #[test]
    fn arithmetic_ops() {
        let v1 = Vec3::new(1.0, -1.0, 0.0);
        let v2 = Vec3::new(-1.0, 1.0, 0.0);
        let v3 = -v1;

        let v = v1 + v2;
        assert_eq!(v, Vec3::zeroes());

        let mut v_mut = v1;
        v_mut += v2;
        assert_eq!(v_mut, Vec3::zeroes()); 

        let v = v1 - v1;
        assert_eq!(v, Vec3::zeroes());

        let mut v_mut = v1;
        v_mut -= v1;
        assert_eq!(v_mut, Vec3::zeroes());
        
        let v = v1 + v3;
        assert_eq!(v, Vec3::zeroes());
    }

    #[test]
    fn scalar_ops() {
        let v1 = Vec3::new(1.0, -1.0, 0.0);
        
        let v = v1 * 2.0;
        assert_eq!(v, Vec3::new(2.0, -2.0, 0.0));

        let mut v_mut = v1;
        v_mut *= 2.0;
        assert_eq!(v_mut, Vec3::new(2.0, -2.0, 0.0));

        let v = v1 / 2.0;
        assert_eq!(v, Vec3::new(0.5, -0.5, 0.0));

        let mut v_mut = v1;
        v_mut /= 2.0;
        assert_eq!(v, Vec3::new(0.5, -0.5, 0.0));
    }

    #[test]
    fn index_ops() {
        let v1 = Vec3::new(1.0, -1.0, 0.0);

        assert_eq!(v1[0], 1.0);
        assert_eq!(v1[1], -1.0);
        assert_eq!(v1[2], 0.0 );

        let mut v_mut = v1;
        v_mut[0] = 0.0;
        v_mut[1] = 1.0;
        v_mut[2] = -1.0;
        assert_eq!(v_mut[0], 0.0);
        assert_eq!(v_mut[1], 1.0);
        assert_eq!(v_mut[2], -1.0);
    }

    #[test]
    fn cmp_ops() {
        let v1 = Vec3::new(0.0, 1.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 1.0);
        let v3 = Vec3::new(0.0, 1.0, 0.0);

        assert!(v1 != v2);
        assert!(v1 == v3);
    }

    #[test]
    fn dot_prod() {
        let v1 = Vec3::new(-1.0, 0.0, 1.0);
        let v2 = Vec3::new(1.0, 1.0, 2.0);

        assert_eq!(Vec3::dot(v1, v2), 1.0);
    }

    #[test]
    fn cross_prod() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(Vec3::cross(v1, v2), Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(Vec3::cross(v2, v1), Vec3::new(0.0, 1.0, 0.0));

        assert_eq!(Vec3::cross(v1, v1), Vec3::zeroes());
    }
}
