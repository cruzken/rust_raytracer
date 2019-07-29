extern crate rust_raytracer;
use rust_raytracer::hitable::{HitList, Hitable};
use rust_raytracer::ray::Ray;
use rust_raytracer::sphere::Sphere;
use rust_raytracer::vec3::Vec3;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

fn color<T: Hitable>(r: Ray, world: &T) -> Vec3 {
    let res = world.hit(&r, 0.0, std::f32::MAX);
    if res.0 {
        let n = res.1.normal;
        0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    } else {
        let unit_direction = r.direction().unit();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let mut output = format!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let s1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    let s2 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    let world: HitList = HitList { list: vec![s1, s2] };
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r, &world);
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
