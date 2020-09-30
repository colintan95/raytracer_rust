#![allow(dead_code)]

mod geometry;
mod shapes;

use geometry::{Point3, Ray, Vec3};
use shapes::{Shape, Sphere, Triangle};

#[derive(Debug)]
pub struct Mat4 {
    pub a00: f32, pub a01: f32, pub a02: f32, pub a03: f32,
    pub a10: f32, pub a11: f32, pub a12: f32, pub a13: f32,
    pub a20: f32, pub a21: f32, pub a22: f32, pub a23: f32,
    pub a30: f32, pub a31: f32, pub a32: f32, pub a33: f32,
}

use std::ops::{Mul};

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

struct Transform {
    mat: Mat4,
}

impl Transform {
    fn identity() -> Transform {
        Transform {
            mat: Mat4::new(1.0, 0.0, 0.0, 0.0,
                           0.0, 1.0, 0.0, 0.0,
                           0.0, 0.0, 1.0, 0.0,
                           0.0, 0.0, 0.0, 1.0),
        }
    }

    fn translate(delta: Vec3) -> Transform {
        Transform {
            mat: Mat4::new(1.0, 0.0, 0.0, delta.x,
                           0.0, 1.0, 0.0, delta.y,
                           0.0, 0.0, 1.0, delta.z,
                           0.0, 0.0, 0.0, 1.0),
        }
    }

    // |theta| is in degrees.
    fn rotate(theta: f32, axis: Vec3) -> Transform {
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

    // TODO: Replace this with multiply operator or sth.
    fn compose(&self, other: &Transform) -> Transform {
        Transform {
            mat: &self.mat * &other.mat,
        }
    }

    fn apply_vec(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.mat.a00 * v.x + self.mat.a01 * v.y + self.mat.a02 * v.z,
            y: self.mat.a10 * v.x + self.mat.a11 * v.y + self.mat.a12 * v.z,
            z: self.mat.a20 * v.x + self.mat.a21 * v.y + self.mat.a22 * v.z,
        }
    }

    fn apply_pt(&self, p: &Point3) -> Point3 {
        Point3 {
            x: self.mat.a00 * p.x + self.mat.a01 * p.y + self.mat.a02 * p.z + self.mat.a03,
            y: self.mat.a10 * p.x + self.mat.a11 * p.y + self.mat.a12 * p.z + self.mat.a13,
            z: self.mat.a20 * p.x + self.mat.a21 * p.y + self.mat.a22 * p.z + self.mat.a23,
        }
    }
}

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
    assert!(vec_equal(&transform1.apply(&v1), &Vec3::new(1.0, 0.0, 0.0)));

    let transform2 = Transform::rotate(90.0, Vec3::new(1.0, 0.0, 0.0));
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    assert!(vec_equal(&transform2.apply(&v2), &Vec3::new(0.0, 0.0, 1.0)));

    let transform3 = Transform::rotate(90.0, Vec3::new(0.0, 0.0, 1.0));
    let v3 = Vec3::new(1.0, 0.0, 0.0);
    assert!(vec_equal(&transform3.apply(&v3), &Vec3::new(0.0, 1.0, 0.0)));
}

fn main() {
    let img_width = 800;
    let img_height = 800;
    let num_pixels = img_width * img_height;

    let mut buffer = vec![100u8; num_pixels];
    let mut rays = Vec::<Ray>::with_capacity(num_pixels);

    let rotate = Transform::rotate(30.0, Vec3::new(1.0, 0.0, 0.0));
    let translate = Transform::translate(Vec3::new(0.0, 10.0, -10.0));
    let transform = translate.compose(&rotate);

    for i in 0..img_width {
        for j in 0..img_height {
            // Converts the (i, j) coordinates of the screen to the (x, y) coordinates of the world
            // space.
            let x = (j as f32) / (img_width as f32) * 1.0 - 0.5;
            let y = -((i as f32) / (img_height as f32) * 1.0 - 0.5);

            let ray = Ray {
                p: transform.apply_pt(&Point3::new(x, y, 1.0)),
                d: transform.apply_vec(&Vec3::new(x, y, 1.0)),
            };

            rays.push(ray);
        }
    } 

    let sphere = Box::new(Sphere {
        c: Point3::new(0.0, 2.5, 5.0),
        r: 2.5,
    });

    let triangle1 = Box::new(Triangle {
        p0: Point3::new(-7.5, 0.0, 0.0),
        p1: Point3::new(-7.5, 0.0, 15.0),
        p2: Point3::new(7.5, 0.0, 15.0),
    });

    let triangle2 = Box::new(Triangle {
        p0: Point3::new(-7.5, 0.0, 0.0),
        p1: Point3::new(7.5, 0.0, 15.0),
        p2: Point3::new(7.5, 0.0, 0.0),
    });

    let mut objects: Vec<Box<dyn Shape>> = Vec::new();
    objects.push(sphere);
    objects.push(triangle1);
    objects.push(triangle2); 

    let light_pos = Point3::new(0.0, 10.0, 5.0);

    let amb_int = 0.3;

    for i in 0..img_width {
        for j in 0..img_height {
            let ray = &rays[i * img_width + j];

            let mut min_t = f32::MAX;
            let mut current_n = Vec3::new(0.0, 0.0, 0.0);

            for shape in &objects {
                match shape.intersect(ray) {
                    Some((t,_,n)) => {
                        if t < min_t {
                            min_t = t;
                            current_n = n;
                        }
                    },
                    None => (), 
                }
            }

            let mut total_int = amb_int;
            
            if min_t < f32::MAX {
                let p = ray.p + min_t * ray.d;
                let n = current_n;

                let l = Vec3::normalize(light_pos - p);
                let diff_int = (Vec3::dot(l, n)).max(0.0);
                
                total_int = num::clamp(amb_int + diff_int, 0.0, 1.0);
            } 
               
            buffer[i * img_width + j] = (255.0 * total_int) as u8;
        }
    }

    image::save_buffer("/mnt/disk2/rust/image.png", &buffer, img_width as u32, img_height as u32, 
                       image::ColorType::L8)
        .unwrap(); 
}
