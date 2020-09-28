#![allow(dead_code)]

mod geometry;
mod shapes;

use geometry::{Point3, Ray, Vec3};
use shapes::{Shape, Sphere};

fn main() {
    let img_width = 800;
    let img_height = 800;
    let num_pixels = img_width * img_height;

    let mut buffer = vec![100u8; num_pixels];
    let mut rays = Vec::<Ray>::with_capacity(num_pixels);

    for i in 0..img_width {
        for j in 0..img_height {
            // Converts the (i, j) coordinates of the screen to the (x, y) coordinates of the world
            // space.
            let x = (j as f32) / (img_width as f32) * 2.0 - 1.0;
            let y = -((i as f32) / (img_height as f32) * 2.0 - 1.0);

            let ray = Ray {
                p: Point3::new(x, y, 1.0),
                d: Vec3::normalize(Vec3::new(x, y, 1.0)),
            };

            rays.push(ray);
        }
    } 

    let sphere = Sphere {
        c: Point3::new(0.0, 0.0, 10.0),
        r: 5.0,
    };

    let light_pos = Point3::new(0.0, 5.0, 2.5);

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
