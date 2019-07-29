use crate::hitable::{Hitable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let mut rec: HitRecord = HitRecord {
            t: t_max,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        };

        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().dot(&r.direction());
        let b: f32 = oc.dot(&r.direction());
        let c: f32 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;

        if discriminant > 0.0 {
            let temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return (true, rec);
            }
            let temp: f32 = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return (true, rec);
            }
        }
        (false, rec)
    }
}
