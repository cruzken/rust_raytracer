extern crate rust_raytracer;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;
use rust_raytracer::vec3::Vec3;
use rust_raytracer::ray::Ray;

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> bool {
    let oc: Vec3 = r.origin() - center;
    let a: f32 = r.direction().dot(&r.direction());
    let b: f32 = 2.0 * oc.dot(&r.direction());
    let c: f32 = oc.dot(&oc) - radius * radius;
    let discriminant: f32 = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(r: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    
    let unit_direction = r.direction().unit();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let mut output = format!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r);
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
