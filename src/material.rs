use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, reflect, Vec3};

pub trait MaterialRay {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { mat: Lambertian },
    Metal { mat: Metal },
}

impl Material {
    pub fn scatter(m: Material, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match m {
            Material::Lambertian { mat } => mat.scatter(r, rec),
            Material::Metal { mat } => mat.scatter(r, rec),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(x: f32, y: f32, z: f32) -> Lambertian {
        Lambertian {
            albedo: Vec3::new(x, y, z),
        }
    }
}

impl MaterialRay for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        Some((self.albedo, scattered))
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(x: f32, y: f32, z: f32) -> Metal {
        Metal {
            albedo: Vec3::new(x, y, z),
        }
    }
}

impl MaterialRay for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r.direction().unit(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        Some((self.albedo, scattered))
    }
}
