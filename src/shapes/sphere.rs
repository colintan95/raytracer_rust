use crate::geometry::{Point3, Vec3, Ray};
use crate::shapes::Shape;

pub struct Sphere {
   pub c: Point3, // center
   pub r: f32, // radius 
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(f32, Point3, Vec3)> {
        let d = Vec3::normalize(ray.d);
        let v = ray.p - self.c;

        let a = Vec3::dot(d, d);
        let b = 2.0 * Vec3::dot(d, v);
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

        let pi = ray.p + t * d;
        let n = Vec3::normalize(pi - self.c);

        if t < 0.0 {
            None
        } else {
            Some((t, pi, n))
        }
    }
}
