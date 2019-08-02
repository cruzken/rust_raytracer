extern crate rust_raytracer;
use rand::prelude::*;
use rust_raytracer::camera::Camera;
use rust_raytracer::hitable::{HitList, Hitable};
use rust_raytracer::material::{Lambertian, Material, Metal};
use rust_raytracer::ray::Ray;
use rust_raytracer::sphere::Sphere;
use rust_raytracer::vec3::Vec3;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

fn color<T: Hitable>(r: Ray, world: &T, depth: u32) -> Vec3 {
    match world.hit(&r, 0.001, std::f32::MAX) {
        Some(x) => {
            if depth < 50 {
                match Material::scatter(x.material, &r, &x) {
                    Some((attenuation, scattered)) => {
                        attenuation * color(scattered, world, depth + 1)
                    },
                    None => Vec3::new(0.0, 0.0, 0.0) 
                }
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        },
        None => {
            let unit_direction = r.direction().unit();
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;
    let mut output = format!("P3\n{} {}\n255\n", nx, ny);
    let s1_mat: Material = Material::Lambertian {
        mat: Lambertian::new(0.8, 0.3, 0.3),
    };
    let s2_mat: Material = Material::Lambertian {
        mat: Lambertian::new(0.8, 0.8, 0.0),
    };
    let s3_mat: Material = Material::Metal {
        mat: Metal::new(0.8, 0.6, 0.2),
    };
    let s4_mat: Material = Material::Metal {
        mat: Metal::new(0.8, 0.8, 0.8),
    };

    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, s1_mat);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, s2_mat);
    let s3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, s3_mat);
    let s4 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, s4_mat);
    let world: HitList<Sphere> = HitList { list: vec![s1, s2, s3, s4] };
    let cam: Camera = Camera::default();
    let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let pu: f32 = rng.gen();
                let pv: f32 = rng.gen();
                let u: f32 = (i as f32 + pu) / nx as f32;
                let v: f32 = (j as f32 + pv) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt()); // Raise gamma to 2
            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;
            output = format!("{}{} {} {}\n", output, ir, ig, ib);
        }
    }

    let dir = Path::new("out/");
    let path = Path::new("out/image.ppm");
    let display = path.display();

    match create_dir_all(&dir) {
        Err(why) => panic!("couldn't create dir {:?}: {}", &dir, why.description()),
        Ok(_) => println!("successfully created directory {:?}", &dir),
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
