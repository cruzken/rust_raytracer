use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub struct HitList<T: Hitable> {
    pub list: Vec<T>,
}

impl<T: Hitable> Hitable for HitList<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far: f32 = t_max;

        let mut rec = None;
        for item in &self.list {
            match item.hit(r, t_min, closest_so_far) {
                Some(x) => {
                    closest_so_far = x.t;
                    rec = Some(x);
                }
                None => (),
            }
        }
        rec
    }
}
