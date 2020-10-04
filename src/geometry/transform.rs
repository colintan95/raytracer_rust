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

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: How to deal with numerical inaccuracies more generally.
    fn vec_equal(v1: &Vec3, v2: &Vec3) -> bool {
        let epsilon = 0.0001;
        let is_equal = (v1.x - v2.x).abs() < epsilon &&     
                       (v1.y - v2.y).abs() < epsilon &&  
                       (v1.z - v2.z).abs() < epsilon;
        is_equal
    }

    #[test]
    fn transform_test() {
        let transform1 = Transform::rotate(90.0, Vec3::new(0.0, 1.0, 0.0));
        let v1 = Vec3::new(0.0, 0.0, 1.0); 
        assert!(vec_equal(&transform1.apply_vec(&v1), &Vec3::new(1.0, 0.0, 0.0)));

        let transform2 = Transform::rotate(90.0, Vec3::new(1.0, 0.0, 0.0));
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        assert!(vec_equal(&transform2.apply_vec(&v2), &Vec3::new(0.0, 0.0, 1.0)));

        let transform3 = Transform::rotate(90.0, Vec3::new(0.0, 0.0, 1.0));
        let v3 = Vec3::new(1.0, 0.0, 0.0);
        assert!(vec_equal(&transform3.apply_vec(&v3), &Vec3::new(0.0, 1.0, 0.0)));
    }
}
