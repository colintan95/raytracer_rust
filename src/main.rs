#![allow(dead_code)]

mod geometry;
mod shapes;
mod rgb;

use geometry::{Point3, Ray, Transform, Vec3};
use shapes::{Shape, Sphere, Triangle};
use rgb::Rgb;

struct Material {
    ambient: Rgb, 
    diffuse: Rgb,
    specular: Rgb,
    reflect: bool,
}

struct Object {
    shape: Box<dyn Shape>,
    material: Material,
}

fn li(ray: &Ray, objs: &Vec::<Object>, camera_pos: Point3, lights: &Vec::<Point3>, 
      max_depth: i8) -> Rgb {

    let mut pixel_val = Rgb::new(0.0, 0.0, 0.0);

    let mut min_t = f32::MAX;
    let mut hit_res: Option<(&Object, Vec3)> = None; 

    for obj in objs {
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
            
            let p = ray.p + min_t * ray.d;

            for light_pos in lights {
                let n = Vec3::normalize(n);

                // Offset to prevent aliasing.
                let diff_ray = Ray {
                    p: p + 0.001 * n,
                    d: *light_pos - (p + 0.001 * n),
                };

                let mut is_blocked = false;

                // Check if any object is blocking the light source.
                for obj in objs {
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

            if obj.material.reflect && max_depth > 0 {
                let d = Vec3::normalize(ray.d);
                let r = d - 2.0 * Vec3::dot(d, n) * n;

                let reflect_ray = Ray {
                    p: p,
                    d: r,
                };

                total_int += 0.3 * li(&reflect_ray, objs, camera_pos, lights, max_depth - 1); 
            }

            pixel_val = total_int.clamp_to_unit(); 
        },
        None => (),
    }

    pixel_val
}

fn main() {
    let img_width = 800;
    let img_height = 800;
    let num_pixels = img_width * img_height;
    let num_channels = 3;

    let mut buffer = vec![0u8; num_pixels * num_channels];
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
                    reflect: true,
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
                    reflect: false,
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
                    reflect: false,
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
                    reflect: false,
                },
        },
    ];

    let lights = vec![Point3::new(0.0, 10.0, 10.0), Point3::new(0.0, 10.0, 5.0)];

    for i in 0..img_width {
        for j in 0..img_height {
            let ray = &rays[i * img_width + j];

            let pixel_val = li(&ray, &objs, camera_pos, &lights, 1);

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
