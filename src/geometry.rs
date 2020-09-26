use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Index, IndexMut};
use std::ops::{Mul, MulAssign, Div, DivAssign};
use num::traits::{Zero, Float};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T: Float = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3::<T> { x, y, z }
    }

    pub fn zeroes() -> Self {
        let zero = Zero::zero();
        Self { x: zero, y: zero, z: zero }
    }

    pub fn dot(v1: Self, v2: Self) -> T {
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
        assert!(v.len() != T::zero());
        v / v.len()
    }

    pub fn len(&self) -> T {
        self.len_sq().sqrt()    
    }

    pub fn len_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}

impl<T: Float> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Float> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x; 
        self.y = self.y - other.y; 
        self.z = self.z - other.z; 
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Self::Output {
        Vec3::<f64> {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl<T: Float> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
	assert!(rhs != T::zero());
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Float> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
	assert!(rhs != T::zero());
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<T: Float> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Index<usize> for Vec3<T> {
    type Output = T;

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

impl<T: Float> IndexMut<usize> for Vec3<T> {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3<T: Float = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Default for Point3<T> {
    fn default() -> Self {
        Self::zeroes()
    }
}

impl<T: Float> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn zeroes() -> Self {
        let zero = Zero::zero();
        Self { x: zero, y: zero, z: zero }
    }
}

impl<T: Float> Add for Point3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> AddAssign for Point3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<T: Float> Sub for Point3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Float> Mul<T> for Point3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float> MulAssign<T> for Point3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<T: Float> Div<T> for Point3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
	assert!(rhs != T::zero());
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Float> DivAssign<T> for Point3<T> {
    fn div_assign(&mut self, rhs: T) {
	assert!(rhs != T::zero());
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<T: Float> Neg for Point3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Index<usize> for Point3<T> {
    type Output = T;

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

impl<T: Float> IndexMut<usize> for Point3<T> {
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

// Vec and Point operations
impl<T: Float> Add<Vec3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, v: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

// pub struct Ray {
//     pub o: Point3,
//     pub d: Vec3,
//     pub t_max: f64,
//     pub time: f64,
// }
// 
// impl Default for Ray {
//     fn default() -> Self {
//         Self {
//             t_max: f64::INFINITY,
//             time: 0.0,
//             ..Default::default()
//         }
//     }
// }
// 
// impl Ray {
//     fn new(o: Point3, d: Vec3, t_max: f64, time: f64) -> Self {
//         Self { o, d, t_max, time, }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checks() {
        let v = Vec3::new(0.0, 0.0, f64::NAN);
        assert!(v.has_nans());
    }

    #[test]
    fn test_lengths() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v.len_sq(), 0.0);
        assert_eq!(v.len(), 0.0);

        let v = Vec3::new(1.0, -2.0, 3.0);
        assert_eq!(v.len_sq(), 14.0);
        assert_eq!(v.len(), 14.0.sqrt());

        assert_eq!(Vec3::normalize(v).len(), 1.0);
    }

    #[test]
    fn test_arithmetic_ops() {
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
    fn test_scalar_ops() {
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
    fn test_index_ops() {
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
    fn test_cmp_ops() {
        let v1 = Vec3::new(0.0, 1.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 1.0);
        let v3 = Vec3::new(0.0, 1.0, 0.0);

        assert!(v1 != v2);
        assert!(v1 == v3);
    }

    #[test]
    fn test_dot_prod() {
        let v1 = Vec3::new(-1.0, 0.0, 1.0);
        let v2 = Vec3::new(1.0, 1.0, 2.0);

        assert_eq!(Vec3::dot(v1, v2), 1.0);
    }

    #[test]
    fn test_cross_prod() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(Vec3::cross(v1, v2), Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(Vec3::cross(v2, v1), Vec3::new(0.0, 1.0, 0.0));

        assert_eq!(Vec3::cross(v1, v1), Vec3::zeroes());
    }

    #[test]
    fn test_point() {
        let p = Point3::new(0.0, 0.0, 0.0);
        assert_eq!(p, Point3::zeroes());
    }
}
