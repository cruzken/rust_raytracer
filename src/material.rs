use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, reflect, refract, Vec3};
use rand::prelude::*;
use serde::{Serialize, Deserialize};

pub trait MaterialRay {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Material {
    Lambertian { mat: Lambertian },
    Metal { mat: Metal },
    Dielectric { mat: Dielectric },
}

impl Material {
    pub fn scatter(m: Material, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match m {
            Material::Lambertian { mat } => mat.scatter(r, rec),
            Material::Metal { mat } => mat.scatter(r, rec),
            Material::Dielectric { mat } => mat.scatter(r, rec),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(x: f32, y: f32, z: f32, f: f32) -> Metal {
        let fuzz = match f {
            x if x < 1.0 => x,
            _ => 1.0,
        };

        Metal {
            albedo: Vec3::new(x, y, z),
            fuzz,
        }
    }
}

impl MaterialRay for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r.direction().unit(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        Some((self.albedo, scattered))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }

    pub fn schlick(&self, cosine: f32) -> f32 {
        let mut r0: f32 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl MaterialRay for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let cosine: f32;
        let reflected: Vec3 = reflect(r.direction(), rec.normal);
        if r.direction().dot(&rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r.direction().dot(&rec.normal) / r.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r.direction().dot(&rec.normal) / r.direction().length();
        }
        let reflect_prob: f32;
        let scattered: Ray;
        let refracted: Vec3;
        match refract(&r.direction(), &outward_normal, ni_over_nt) {
            Some(x) => {
                reflect_prob = self.schlick(cosine);
                refracted = x;
                let mut rng = thread_rng();
                if rng.gen::<f32>() < reflect_prob {
                    scattered = Ray::new(rec.p, reflected);
                    return Some((attenuation, scattered));
                } else {
                    scattered = Ray::new(rec.p, refracted);
                    return Some((attenuation, scattered));
                }
            }
            None => {
                scattered = Ray::new(rec.p, reflected);
                return Some((attenuation, scattered));
            }
        }
    }
}
