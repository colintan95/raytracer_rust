use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Rgb { r, g, b, }
    }

    // Truncates all components so that their values are within the range [0.0, 1.0].
    pub fn clamp_to_unit(self) -> Self {
        Rgb { 
            r: num::clamp(self.r, 0.0, 1.0),
            g: num::clamp(self.g, 0.0, 1.0),
            b: num::clamp(self.b, 0.0, 1.0),
        }
    }
}

impl Add for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Rgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Mul for Rgb {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Rgb {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Rgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgb {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Rgb> for f32 {
    type Output = Rgb;

    fn mul(self, rhs: Rgb) -> Self::Output {
        Rgb {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl MulAssign for Rgb {
    fn mul_assign(&mut self, other: Self) {
        self.r *= other.r;
        self.g *= other.b;
        self.b *= other.b;
    }
}

