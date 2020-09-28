#![allow(dead_code)]

mod geometry;

use geometry::{Point3, Vec3};

struct Ray {
    p: Point3,
    d: Vec3,
}

trait Shape {
    // If there's an intersection, returns the point of intersection and the normal.
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vec3)>;
}

struct Triangle {
    p0: Point3,
    p1: Point3,
    p2: Point3,
}

impl Triangle {
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vec3)> {
        let vp = ray.p - self.p0;
        let v1 = self.p1 - self.p0;
        let v2 = self.p2 - self.p0;
    
        let v_tmpd = Vec3::cross(ray.d, v2); 
        let v_tmpp = Vec3::cross(vp, v1); 

        let s = 1.0 / Vec3::dot(v_tmpd, v1);

        let u = s * Vec3::dot(v_tmpd, vp);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let v = s * Vec3::dot(v_tmpp, ray.d);
        if v < 0.0 || v > 1.0 {
            return None;
        }

        if u + v < 0.0 || u + v > 1.0 {
            return None;
        }
    
        let t = s * Vec3::dot(v_tmpp, v2); 
        
        if t >= 0.0 { 
            let pi = ray.p + t * ray.d;

            let mut n = Vec3::normalize(Vec3::cross(v1, v2));
            if Vec3::dot(-ray.d, n) < 0.0 {
                n = -n;
            }
            // TODO: Test that the normal returned is correct.
            Some((pi, n))
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

    match triangle1.intersect(&ray1) {
        Some((p, n)) => {
            assert_eq!(p, Point3::new(0.0, 0.0, 2.0));
            assert_eq!(n, Vec3::new(0.0, 0.0, -1.0));
        },
        None => assert!(false),
    }

    let triangle2 = Triangle {
        p0: Point3::new(-1.0, -1.0, -2.0),
        p1: Point3::new(1.0, -1.0, -2.0),
        p2: Point3::new(0.0, 1.0, -2.0),
    };

    match triangle2.intersect(&ray1) {
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

    match triangle.intersect(&ray1) {
        Some(_) => assert!(false),
        None => assert!(true),
    }

    let ray2 = Ray {
        p: Point3::new(2.0, 2.0, 0.0),
        d: Vec3::new(0.0, 0.0, 1.0),
    };

    match triangle.intersect(&ray2) {
        Some(_) => assert!(false),
        None => assert!(true),
    }

    let ray3 = Ray {
        p: Point3::new(0.0, -2.0, 0.0),
        d: Vec3::new(0.0, 0.0, 1.0),
    };

    match triangle.intersect(&ray3) {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

struct Sphere {
   c: Point3, // center
   r: f32, // radius 
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vec3)> {
        let v = ray.p - self.c;

        let a = Vec3::dot(ray.d, ray.d);
        let b = 2.0 * Vec3::dot(ray.d, v);
        let c = Vec3::dot(v, v) - self.r.powi(2);

        let discrm = b * b - 4.0 * a * c;
        
        if discrm < 0.0 {
            return None;
        }

        let t = if discrm > 0.0 {
            let t1 = (-b + discrm.sqrt()) / (2.0 * a);
            let t2 = (-b - discrm.sqrt()) / (2.0 * a);
            t1.min(t2)
        } else {
            -b / (2.0 * a)
        };

        let pi = ray.p + t * ray.d;
        let n = Vec3::normalize(pi - self.c);

        Some((pi, n))
    }
}

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
