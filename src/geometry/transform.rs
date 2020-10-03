use crate::geometry::{Mat4, Point3, Vec3};

#[derive(Debug)]
pub struct Transform {
    mat: Mat4,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            mat: Mat4::new(1.0, 0.0, 0.0, 0.0,
                           0.0, 1.0, 0.0, 0.0,
                           0.0, 0.0, 1.0, 0.0,
                           0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn translate(delta: Vec3) -> Transform {
        Transform {
            mat: Mat4::new(1.0, 0.0, 0.0, delta.x,
                           0.0, 1.0, 0.0, delta.y,
                           0.0, 0.0, 1.0, delta.z,
                           0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn translate_pt(pt: Point3) -> Transform {
        Transform {
            mat: Mat4::new(1.0, 0.0, 0.0, pt.x,
                           0.0, 1.0, 0.0, pt.y,
                           0.0, 0.0, 1.0, pt.z,
                           0.0, 0.0, 0.0, 1.0),
        }
    }

    // |theta| is in degrees.
    pub fn rotate(theta: f32, axis: Vec3) -> Transform {
        let axis = Vec3::normalize(axis);

        let sin_theta = theta.to_radians().sin();
        let cos_theta = theta.to_radians().cos();
    
        Transform {
            mat: Mat4::new(
                axis.x * axis.x + (1.0 - axis.x * axis.x) * cos_theta,
                axis.x * axis.y * (1.0 - cos_theta) - axis.z * sin_theta,
                axis.x * axis.z * (1.0 - cos_theta) + axis.y * sin_theta,
                0.0,

                axis.x * axis.y * (1.0 - cos_theta) + axis.z * sin_theta,
                axis.y * axis.y + (1.0 - axis.y * axis.y) * cos_theta,
                axis.y * axis.z * (1.0 - cos_theta) - axis.x * sin_theta,
                0.0,

                axis.x * axis.z * (1.0 - cos_theta) - axis.y * sin_theta,
                axis.y * axis.z * (1.0 - cos_theta) + axis.x * sin_theta,
                axis.z * axis.z + (1.0 - axis.z * axis.z) * cos_theta,
                0.0,

                0.0,
                0.0,
                0.0,
                1.0),
        }
    }

    pub fn compose(&self, other: &Transform) -> Transform {
        Transform {
            mat: &self.mat * &other.mat,
        }
    }

    pub fn apply_vec(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.mat.a00 * v.x + self.mat.a01 * v.y + self.mat.a02 * v.z,
            y: self.mat.a10 * v.x + self.mat.a11 * v.y + self.mat.a12 * v.z,
            z: self.mat.a20 * v.x + self.mat.a21 * v.y + self.mat.a22 * v.z,
        }
    }

    pub fn apply_pt(&self, p: &Point3) -> Point3 {
        Point3 {
            x: self.mat.a00 * p.x + self.mat.a01 * p.y + self.mat.a02 * p.z + self.mat.a03,
            y: self.mat.a10 * p.x + self.mat.a11 * p.y + self.mat.a12 * p.z + self.mat.a13,
            z: self.mat.a20 * p.x + self.mat.a21 * p.y + self.mat.a22 * p.z + self.mat.a23,
        }
    }
}
