use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::f32::consts::PI;

pub fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    let mut rng = thread_rng();
    while p.dot(&p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius: f32 = aperture / 2.0;
        let theta: f32 = vfov * PI / 180.0;
        let half_height: f32 = (theta / 2.0).tan();
        let half_width: f32 = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: origin
                - u * half_width * focus_dist
                - v * half_height * focus_dist
                - w * focus_dist,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
