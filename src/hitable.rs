use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord);
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub struct HitList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> (bool, HitRecord) {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;
        let mut rec: HitRecord = HitRecord::new(
            closest_so_far,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );

        for item in self.list.iter() {
            let res = item.hit(r, t_min, closest_so_far);
            if res.0 {
                hit_anything = true;
                closest_so_far = res.1.t;
                rec = res.1;
            }
        }
        (hit_anything, rec)
    }
}
