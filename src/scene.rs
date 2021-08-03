use rand::prelude::*;

use crate::camera::Camera;
use crate::hitable::HitList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::color;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene {
    width: u32,
    height: u32,
    cam: Camera,
    world: HitList<Sphere>,
}

#[wasm_bindgen]
impl Scene {
    pub fn new(width: u32, height: u32, world_obj: JsValue) -> Scene {
        let lookfrom = Vec3::new(16.0, 2.0, 4.0);
        let lookat = Vec3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus: f32 = (lookfrom - lookat).length();
        let aperture: f32 = 0.2;
        let world: HitList<Sphere> = world_obj.into_serde().unwrap();

        let cam: Camera = Camera::new(
            lookfrom,
            lookat,
            vup,
            15.0,
            width as f32 / height as f32,
            aperture,
            dist_to_focus,
        );

        Scene {
            width,
            height,
            cam,
            world
        }
    }

    pub fn image_row(&self, y: u32) -> Vec<u8> {
        console_error_panic_hook::set_once();
        (0..self.width)
            .into_iter()
            .map(|x| {
                let mut rng = thread_rng();
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                let ns = 100;
                for _ in 0..ns {
                    let pu: f32 = rng.gen();
                    let pv: f32 = rng.gen();
                    let u: f32 = (x as f32 + pu) / self.width as f32;
                    let v: f32 = (y as f32 + pv) / self.height as f32;
                    let r = self.cam.get_ray(u, v);
                    col += color(r, &self.world, 0);
                }
                col /= ns as f32;
                col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt()); // Raise gamma to 2
                let ir = (255.99 * col.r()) as u8;
                let ig = (255.99 * col.g()) as u8;
                let ib = (255.99 * col.b()) as u8;
                vec![ir, ig, ib, 255]
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}
