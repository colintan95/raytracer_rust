use std::ops::{Add, AddAssign, Sub, Neg, Index, IndexMut};
use std::ops::{Mul, MulAssign, Div, DivAssign};

use crate::geometry::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Point3 {
    fn default() -> Self {
        Self::zeroes()
    }
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zeroes() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Point3 {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Point3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl Div<f32> for Point3 {
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

impl DivAssign<f32> for Point3 {
    fn div_assign(&mut self, rhs: f32) {
	assert!(rhs != 0.0);
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl Neg for Point3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Point3 {
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

impl IndexMut<usize> for Point3 {
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

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, v: Vec3) -> Self::Output {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p = Point3::new(0.0, 0.0, 0.0);
        assert_eq!(p, Point3::zeroes());
    }
}
