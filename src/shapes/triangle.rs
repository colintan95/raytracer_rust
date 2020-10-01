use crate::geometry::{Point3, Ray, Vec3};
use crate::shapes::Shape;

pub struct Triangle {
    pub p0: Point3,
    pub p1: Point3,
    pub p2: Point3,
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<(f32, Point3, Vec3)> {
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
            Some((t, pi, n))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
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
            Some((_, p, n)) => {
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
    fn intersect_no_intersection() {
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
}
