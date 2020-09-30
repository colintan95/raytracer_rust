#![allow(dead_code)]

mod geometry;
mod shapes;

use geometry::{Point3, Ray, Vec3};
use shapes::{Shape, Sphere};

struct Transform {
    a00: f32, a01: f32, a02: f32, a03: f32,
    a10: f32, a11: f32, a12: f32, a13: f32,
    a20: f32, a21: f32, a22: f32, a23: f32,
    a30: f32, a31: f32, a32: f32, a33: f32,
}

impl Transform {
    fn identity() -> Transform {
        Transform {
            a00: 1.0,
            a01: 0.0,
            a02: 0.0,
            a03: 0.0,

            a10: 0.0,
            a11: 1.0,
            a12: 0.0,
            a13: 0.0,

            a20: 0.0,
            a21: 0.0,
            a22: 1.0,
            a23: 0.0,

            a30: 0.0,
            a31: 0.0,
            a32: 0.0,
            a33: 1.0,
        }
    }

    fn translate(delta: Vec3) -> Transform {
        Transform {
            a00: 1.0,
            a01: 0.0,
            a02: 0.0,
            a03: delta.x,

            a10: 0.0,
            a11: 1.0,
            a12: 0.0,
            a13: delta.y,

            a20: 0.0,
            a21: 0.0,
            a22: 1.0,
            a23: delta.z,

            a30: 0.0,
            a31: 0.0,
            a32: 0.0,
            a33: 1.0,
        }
    }

    // |theta| is in degrees.
    fn rotate(theta: f32, axis: Vec3) -> Transform {
        let axis = Vec3::normalize(axis);

        let sin_theta = theta.to_radians().sin();
        let cos_theta = theta.to_radians().cos();
    
        Transform {
            a00: axis.x * axis.x + (1.0 - axis.x * axis.x) * cos_theta,
            a01: axis.x * axis.y * (1.0 - cos_theta) - axis.z * sin_theta,
            a02: axis.x * axis.z * (1.0 - cos_theta) + axis.y * sin_theta,
            a03: 0.0,

            a10: axis.x * axis.y * (1.0 - cos_theta) + axis.z * sin_theta,
            a11: axis.y * axis.y + (1.0 - axis.y * axis.y) * cos_theta,
            a12: axis.y * axis.z * (1.0 - cos_theta) - axis.x * sin_theta,
            a13: 0.0,

            a20: axis.x * axis.z * (1.0 - cos_theta) - axis.y * sin_theta,
            a21: axis.y * axis.z * (1.0 - cos_theta) + axis.x * sin_theta,
            a22: axis.z * axis.z + (1.0 - axis.z * axis.z) * cos_theta,
            a23: 0.0,

            a30: 0.0,
            a31: 0.0,
            a32: 0.0,
            a33: 1.0,
        }
    }

    // TODO: Replace this with multiply operator or sth.
    fn compose(&self, other: &Transform) -> Transform {
        Transform {
            a00 : self.a00 * other.a00 + self.a01 * other.a10 + self.a02 * other.a20 +
                  self.a03 * other.a30,
            a01 : self.a00 * other.a01 + self.a01 * other.a11 + self.a02 * other.a21 +
                  self.a03 * other.a31,
            a02 : self.a00 * other.a02 + self.a01 * other.a12 + self.a02 * other.a22 +
                  self.a03 * other.a32,
            a03 : self.a00 * other.a03 + self.a01 * other.a13 + self.a02 * other.a23 +
                  self.a03 * other.a33, 

            a10 : self.a10 * other.a00 + self.a11 * other.a10 + self.a12 * other.a20 +
                  self.a13 * other.a30,
            a11 : self.a10 * other.a01 + self.a11 * other.a11 + self.a12 * other.a21 +
                  self.a13 * other.a31,
            a12 : self.a10 * other.a02 + self.a11 * other.a12 + self.a12 * other.a22 +
                  self.a13 * other.a32,
            a13 : self.a10 * other.a03 + self.a11 * other.a13 + self.a12 * other.a23 +
                  self.a13 * other.a33, 

            a20 : self.a20 * other.a00 + self.a21 * other.a10 + self.a22 * other.a20 +
                  self.a23 * other.a30,
            a21 : self.a20 * other.a01 + self.a21 * other.a11 + self.a22 * other.a21 +
                  self.a23 * other.a31,
            a22 : self.a20 * other.a02 + self.a21 * other.a12 + self.a22 * other.a22 +
                  self.a23 * other.a32,
            a23 : self.a20 * other.a03 + self.a21 * other.a13 + self.a22 * other.a23 +
                  self.a23 * other.a33, 

            a30 : self.a30 * other.a00 + self.a31 * other.a10 + self.a32 * other.a20 +
                  self.a33 * other.a30,
            a31 : self.a30 * other.a01 + self.a31 * other.a11 + self.a32 * other.a21 +
                  self.a33 * other.a31,
            a32 : self.a30 * other.a02 + self.a31 * other.a12 + self.a32 * other.a22 +
                  self.a33 * other.a32,
            a33 : self.a30 * other.a03 + self.a31 * other.a13 + self.a32 * other.a23 +
                  self.a33 * other.a33, 
        }
    }

    fn apply_vec(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.a00 * v.x + self.a01 * v.y + self.a02 * v.z,
            y: self.a10 * v.x + self.a11 * v.y + self.a12 * v.z,
            z: self.a20 * v.x + self.a21 * v.y + self.a22 * v.z,
        }
    }

    fn apply_pt(&self, p: &Point3) -> Point3 {
        Point3 {
            x: self.a00 * p.x + self.a01 * p.y + self.a02 * p.z + self.a03,
            y: self.a10 * p.x + self.a11 * p.y + self.a12 * p.z + self.a13,
            z: self.a20 * p.x + self.a21 * p.y + self.a22 * p.z + self.a23,
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

    let rotate = Transform::rotate(45.0, Vec3::new(1.0, 0.0, 0.0));
    let translate = Transform::translate(Vec3::new(0.0, 10.0, 0.0));
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

    let sphere = Sphere {
        c: Point3::new(0.0, 0.0, 10.0),
        r: 2.5,
    };

    let light_pos = Point3::new(0.0, 5.0, 10.0);

    for i in 0..img_width {
        for j in 0..img_height {
            match sphere.intersect(&rays[i * img_width + j]) {
                Some((p, n)) => {
                    let l = Vec3::normalize(light_pos - p);
                    let int_d = (Vec3::dot(l, n)).max(0.0);
                    let int_a = 0.3;
                    
                    let int_total = num::clamp(int_a + int_d, 0.0, 1.0);
                    
                    buffer[i * img_width + j] = (255.0 * int_total) as u8;
                },
                None => buffer[i * img_width + j] = 100,
            }
        }
    }

    image::save_buffer("/mnt/disk2/rust/image.png", &buffer, img_width as u32, img_height as u32, 
                       image::ColorType::L8)
        .unwrap(); 
}
