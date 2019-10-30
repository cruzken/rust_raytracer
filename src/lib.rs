use rand::prelude::*;

mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod utils;

use crate::camera::Camera;
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
pub fn gen_image() -> Vec<u8> {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;

    let world: HitList<Sphere> = random_scene();

    let lookfrom = Vec3::new(16.0, 2.0, 4.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f32 = (lookfrom - lookat).length();
    let aperture: f32 = 0.2;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        15.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );

    (0..ny)
        .into_iter()
        .rev()
        .map(|j| {
            (0..nx)
                .into_iter()
                .map(|i| {
                    let mut rng = thread_rng();
                    let mut col = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..ns {
                        let pu: f32 = rng.gen();
                        let pv: f32 = rng.gen();
                        let u: f32 = (i as f32 + pu) / nx as f32;
                        let v: f32 = (j as f32 + pv) / ny as f32;
                        let r = cam.get_ray(u, v);
                        col += color(r, &world, 0);
                    }
                    col /= ns as f32;
                    col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt()); // Raise gamma to 2
                    let ir = (255.99 * col.r()) as u8;
                    let ig = (255.99 * col.g()) as u8;
                    let ib = (255.99 * col.b()) as u8;
                    vec![ir, ig, ib]
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}
