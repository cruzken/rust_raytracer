use rand::prelude::*;

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod utils;
mod scene;

use crate::hitable::{HitList, Hitable};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-placeholder!");
}

fn random_scene() -> HitList<Sphere> {
    let mut hitlist = HitList { list: Vec::new() };
    hitlist.list.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            mat: Lambertian::new(0.5, 0.5, 0.5),
        },
    ));
    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                a as f32 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    hitlist.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian {
                            mat: Lambertian::new(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            ),
                        },
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    hitlist.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            mat: Metal::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * rng.gen::<f32>(),
                            ),
                        },
                    ));
                } else {
                    //glass
                    hitlist.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric {
                            mat: Dielectric::new(1.5),
                        },
                    ));
                }
            }
        }
    }
    hitlist.list.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric {
            mat: Dielectric::new(1.5),
        },
    ));

    hitlist.list.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            mat: Lambertian::new(0.4, 0.2, 0.1),
        },
    ));

    hitlist.list.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            mat: Metal::new(0.7, 0.6, 0.5, 0.0),
        },
    ));

    hitlist
}

fn color<T: Hitable>(r: Ray, world: &T, depth: u32) -> Vec3 {
    match world.hit(&r, 0.001, std::f32::MAX) {
        Some(x) => {
            if depth < 50 {
                match Material::scatter(x.material, &r, &x) {
                    Some((attenuation, scattered)) => {
                        attenuation * color(scattered, world, depth + 1)
                    }
                    None => Vec3::new(0.0, 0.0, 0.0),
                }
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
        None => {
            let unit_direction = r.direction().unit();
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

#[wasm_bindgen]
pub fn scene_gen_json() -> JsValue {
    JsValue::from_serde(&random_scene()).unwrap()
}

