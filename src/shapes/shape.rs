use crate::geometry::{Point3, Vec3, Ray};

pub trait Shape {
    // If there's an intersection, returns the point of intersection and the normal.
    fn intersect(&self, ray: &Ray) -> Option<(Point3, Vec3)>;
}
