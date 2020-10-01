#![allow(dead_code)]

mod geometry;
mod shapes;

use geometry::{Point3, Ray, Transform, Vec3};
use shapes::{Shape, Sphere, Triangle};

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

struct Object {
    shape: Box<dyn Shape>,
    color: (f32, f32, f32), // 0.0 to 1.0
}

fn main() {
    let img_width = 800;
    let img_height = 800;
    let num_pixels = img_width * img_height;
    let num_channels = 3;

    let mut buffer = vec![100u8; num_pixels * num_channels];
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

    let objs = vec![
        Object {
            shape: Box::new(
                Sphere {
                    c: Point3::new(0.0, 2.5, 7.5),
                    r: 2.5, 
                }),
            color: (1.0, 0.0, 0.0),
        },
        Object {
            shape: Box::new(
                Triangle {
                    p0: Point3::new(-7.5, 0.0, 0.0),
                    p1: Point3::new(-7.5, 0.0, 15.0),
                    p2: Point3::new(7.5, 0.0, 15.0),
                }),
            color: (1.0, 1.0, 1.0),
        },
        Object {
            shape: Box::new(
                Triangle {
                    p0: Point3::new(-7.5, 0.0, 0.0),
                    p1: Point3::new(7.5, 0.0, 15.0),
                    p2: Point3::new(7.5, 0.0, 0.0),
                }),
            color: (1.0, 1.0, 1.0),
        },
    ];

    let light_pos = Point3::new(0.0, 10.0, 7.5);

    let amb_int = 0.3;

    for i in 0..img_width {
        for j in 0..img_height {
            let ray = &rays[i * img_width + j];

            let mut min_t = f32::MAX;
            let mut curr_n = Vec3::new(0.0, 0.0, 0.0);
            let mut curr_color = (0.0, 0.0, 0.0);

            for obj in &objs {
                match obj.shape.intersect(ray) {
                    Some((t,_,n)) => {
                        if t < min_t {
                            min_t = t;
                            curr_n = n;
                            curr_color = obj.color;
                        }
                    },
                    None => (), 
                }
            }

            let mut total_int = amb_int;
            
            if min_t < f32::MAX {
                let p = ray.p + min_t * ray.d;
                let n = curr_n;
                let l = Vec3::normalize(light_pos - p);

                // Offset to prevent aliasing.
                let diff_ray = Ray {
                    p: p + 0.001 * n,
                    d: light_pos - (p + 0.001 * n),
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
                    let diff_int = (Vec3::dot(l, n)).max(0.0);
                    
                    total_int = num::clamp(amb_int + diff_int, 0.0, 1.0);
                }
            } 

            let base_idx = (i * img_width + j) * 3; 
            
            buffer[base_idx + 0] = (255.0 * total_int * curr_color.0) as u8;
            buffer[base_idx + 1] = (255.0 * total_int * curr_color.2) as u8;
            buffer[base_idx + 2] = (255.0 * total_int * curr_color.2) as u8;
        }
    }

    image::save_buffer("/mnt/disk2/rust/image.png", &buffer, img_width as u32, img_height as u32, 
                       image::ColorType::Rgb8)
        .unwrap(); 
}
