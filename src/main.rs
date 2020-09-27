#![allow(dead_code)]

mod geometry;

use geometry::{Point3, Vec3};

struct Ray {
    p: Point3,
    d: Vec3,
}

struct Triangle {
    p0: Point3,
    p1: Point3,
    p2: Point3,
}

impl Ray {
    fn intersect_triangle(&self, triangle: &Triangle) -> Option<Point3> {
        let vp = self.p - triangle.p0;
        let v1 = triangle.p1 - triangle.p0;
        let v2 = triangle.p2 - triangle.p0;
    
        let v_tmpd = Vec3::cross(self.d, v2); 
        let v_tmpp = Vec3::cross(vp, v1); 

        let s = 1.0 / Vec3::dot(v_tmpd, v1);

        let u = s * Vec3::dot(v_tmpd, vp);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let v = s * Vec3::dot(v_tmpp, self.d);
        if v < 0.0 || v > 1.0 {
            return None;
        }

        if u + v < 0.0 || u + v > 1.0 {
            return None;
        }
    
        let t = s * Vec3::dot(v_tmpp, v2); 
        
        if t >= 0.0 { 
            Some(self.p + t * self.d)
        } else {
            None
        }
    }
}

#[test]
fn ray_triangle_intersection() {
    let ray1 = Ray {
        p: Point3::new(0.0, 0.0, 0.0),
        d: Vec3::new(0.0, 0.0, 1.0),
    };

    let triangle1 = Triangle {
        p0: Point3::new(-1.0, -1.0, 2.0),
        p1: Point3::new(1.0, -1.0, 2.0),
        p2: Point3::new(0.0, 1.0, 2.0),
    };

    match ray1.intersect_triangle(&triangle1) {
        Some(p) => assert_eq!(p, Point3::new(0.0, 0.0, 2.0)), 
        None => assert!(false),
    }

    let triangle2 = Triangle {
        p0: Point3::new(-1.0, -1.0, -2.0),
        p1: Point3::new(1.0, -1.0, -2.0),
        p2: Point3::new(0.0, 1.0, -2.0),
    };

    match ray1.intersect_triangle(&triangle2) {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn ray_triangle_intersection_no_intersection() {
    let triangle = Triangle {
        p0: Point3::new(-1.0, -1.0, 2.0),
        p1: Point3::new(1.0, -1.0, 2.0),
        p2: Point3::new(0.0, 1.0, 2.0),
    };

    let ray1 = Ray {
        p: Point3::new(-2.0, 2.0, 0.0),
        d: Vec3::new(0.0, 0.0, 1.0),
    };

    match ray1.intersect_triangle(&triangle) {
        Some(_) => assert!(false),
        None => assert!(true),
    }

    let ray2 = Ray {
        p: Point3::new(2.0, 2.0, 0.0),
        d: Vec3::new(0.0, 0.0, 1.0),
    };

    match ray2.intersect_triangle(&triangle) {
        Some(_) => assert!(false),
        None => assert!(true),
    }

    let ray3 = Ray {
        p: Point3::new(0.0, -2.0, 0.0),
        d: Vec3::new(0.0, 0.0, 1.0),
    };

    match ray3.intersect_triangle(&triangle) {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

fn main() {
    let img_width = 100;
    let img_height = 100;
    let img_num_pixels = img_width * img_height;

    let mut buffer = vec![100u8; img_num_pixels];
    let mut rays = Vec::<Ray>::with_capacity(img_num_pixels);

    for i in 0..100 {
        for j in 0..100 {
            let x = (i as f64) / 100.0 * 10.0 - 5.0;
            let y = (j as f64) / 100.0 * 10.0 - 5.0;

            let ray = Ray {
                p: Point3::new(x, y, 5.0),
                d: Vec3::new(0.0, 0.0, 1.0),
            };

            rays.push(ray);
        }
    } 

    let triangle = Triangle {
        p0: Point3::new(-2.5, -2.5, 10.0),
        p1: Point3::new(2.5, -2.5, 10.0),
        p2: Point3::new(0.0, 2.5, 10.0),
    };

    for i in 0..100 {
        for j in 0..100 {
            match rays[i * img_width + j].intersect_triangle(&triangle) {
                Some(_) => buffer[i * img_width + j] = 255,
                None => buffer[i * img_width + j] = 100,
            }
        }
    }

    image::save_buffer("/mnt/disk2/rust/image.png", &buffer, img_width as u32, img_height as u32, 
                       image::ColorType::L8)
        .unwrap(); 
}
