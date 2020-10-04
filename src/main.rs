#![allow(dead_code)]

mod geometry;
mod shapes;

use geometry::{Point3, Ray, Transform, Vec3};
use shapes::{Shape, Sphere, Triangle};

use std::ops::{Add, AddAssign, Mul, MulAssign};

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

#[derive(Debug, Copy, Clone)]
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
    fn new(r: f32, g: f32, b: f32) -> Self {
        Rgb { r, g, b, }
    }

    // Truncates all components so that their values are within the range [0.0, 1.0].
    fn clamp_to_unit(self) -> Self {
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

struct Material {
    ambient: Rgb, 
    diffuse: Rgb,
    specular: Rgb,
}

struct Object {
    shape: Box<dyn Shape>,
    material: Material,
}

fn main() {
    let img_width = 800;
    let img_height = 800;
    let num_pixels = img_width * img_height;
    let num_channels = 3;

    let mut buffer = vec![100u8; num_pixels * num_channels];
    let mut rays = Vec::<Ray>::with_capacity(num_pixels);

    let camera_pos = Point3::new(0.0, 10.0, -10.0);

    let rotate = Transform::rotate(30.0, Vec3::new(1.0, 0.0, 0.0));
    let translate = Transform::translate_pt(camera_pos);
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

    let plane_width = 20.0;
    let plane_depth = 20.0;

    let objs = vec![
        Object {
            shape: Box::new(
                Sphere {
                    c: Point3::new(-3.5, 2.5, 7.5),
                    r: 2.5, 
                }),
            material:
                Material {
                    ambient: Rgb::new(0.1, 0.0, 0.0),
                    diffuse: Rgb::new(0.5, 0.0, 0.0),
                    specular: Rgb::new(1.0, 1.0, 1.0),
                },
        },
        Object {
            shape: Box::new(
                Sphere {
                    c: Point3::new(3.5, 2.5, 7.5),
                    r: 2.5, 
                }),
            material:
                Material {
                    ambient: Rgb::new(0.0, 0.0, 0.1),
                    diffuse: Rgb::new(0.0, 0.0, 0.5),
                    specular: Rgb::new(1.0, 1.0, 1.0),
                },
        },
        Object {
            shape: Box::new(
                Triangle {
                    p0: Point3::new(-plane_width / 2.0, 0.0, 0.0),
                    p1: Point3::new(-plane_width / 2.0, 0.0, plane_depth),
                    p2: Point3::new(plane_width / 2.0, 0.0, plane_depth),
                }),
            material:
                Material {
                    ambient: Rgb::new(0.1, 0.1, 0.1),
                    diffuse: Rgb::new(0.5, 0.5, 0.5),
                    specular: Rgb::new(1.0, 1.0, 1.0),
                },
        },
        Object {
            shape: Box::new(
                Triangle {
                    p0: Point3::new(-plane_width / 2.0, 0.0, 0.0),
                    p1: Point3::new(plane_width / 2.0, 0.0, plane_depth),
                    p2: Point3::new(plane_width / 2.0, 0.0, 0.0),
                }),
            material:
                Material {
                    ambient: Rgb::new(0.1, 0.1, 0.1),
                    diffuse: Rgb::new(0.5, 0.5, 0.5),
                    specular: Rgb::new(1.0, 1.0, 1.0),
                },
        },
    ];

    let lights = vec![Point3::new(0.0, 10.0, 10.0), Point3::new(0.0, 10.0, 5.0)];

    for i in 0..img_width {
        for j in 0..img_height {
            let ray = &rays[i * img_width + j];

            let mut pixel_val = Rgb::new(0.0, 0.0, 0.0);
            let mut min_t = f32::MAX;
            let mut hit_res: Option<(&Object, Vec3)> = None; 

            for obj in &objs {
                match obj.shape.intersect(ray) {
                    Some((t,_,n)) => {
                        if t < min_t {
                            min_t = t;
                            hit_res = Some((&obj, n)); 
                        }
                    },
                    None => (), 
                }
            }

            match hit_res {
                Some((obj, n)) => { 
                    let mut total_int = obj.material.ambient;

                    for light_pos in &lights {
                        let p = ray.p + min_t * ray.d;
                        let n = Vec3::normalize(n);

                        // Offset to prevent aliasing.
                        let diff_ray = Ray {
                            p: p + 0.001 * n,
                            d: *light_pos - (p + 0.001 * n),
                        };

                        let mut is_blocked = false;

                        // Check if any object is blocking the light source.
                        for obj in &objs {
                            match obj.shape.intersect(&diff_ray) {
                                Some(_) => {
                                    is_blocked = true;
                                    break;
                                },
                                None => is_blocked = false,
                            }
                        }

                        if !is_blocked {
                            let l = Vec3::normalize(*light_pos - p);
                            let diff_coeff = (Vec3::dot(l, n)).max(0.0);

                            let v = Vec3::normalize(camera_pos - p);
                            let h = Vec3::normalize(l + v);
                            let spec_coeff = (Vec3::dot(n, h)).max(0.0).powf(100.0);
                            
                            total_int += diff_coeff * obj.material.diffuse +
                                         spec_coeff * obj.material.specular; 
                        }
                    }

                    pixel_val = total_int.clamp_to_unit(); 
                },
                None => (),
            }

            let base_idx = (i * img_width + j) * 3; 
            
            buffer[base_idx + 0] = (255.0 * pixel_val.r) as u8;
            buffer[base_idx + 1] = (255.0 * pixel_val.g) as u8;
            buffer[base_idx + 2] = (255.0 * pixel_val.b) as u8;
        }
    }

    image::save_buffer("/mnt/disk2/rust/image.png", &buffer, img_width as u32, img_height as u32, 
                       image::ColorType::Rgb8)
        .unwrap(); 
}
