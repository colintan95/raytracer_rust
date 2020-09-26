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
    
        println!("vp: {:?}", vp);
        println!("v1: {:?}", v1);
        println!("v2: {:?}", v2); 
    
        let v_tmpd = Vec3::cross(self.d, v2); 
        let v_tmpp = Vec3::cross(vp, v1); 
    
        println!("v_tmpd: {:?}", v_tmpd);
        println!("v_tmpp: {:?}", v_tmpp); 
    
        let t = (1.0 / Vec3::dot(v_tmpd, v1)) * Vec3::dot(v_tmpp, v2); 
        
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

fn main() {
    let buffer: [u8; 30000] = [100; 30000];


    image::save_buffer("/mnt/disk2/rust/image.png", &buffer, 100, 100, image::ColorType::Rgb8)
        .unwrap(); 
}
