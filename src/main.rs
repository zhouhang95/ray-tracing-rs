#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod hitable_list;
mod sphere;
mod camera;

use crate::vec3::{Vec3, dot};
use crate::ray::Ray;
use crate::hitable_list::HitableList;
use crate::hitable::{HitRecord, Hitable};
use crate::camera::Camera;
use crate::sphere::Sphere;

use std::f32;
use rand::Rng;


fn random_in_unit_sphere() -> Vec3 {
    let mut p;
    loop {
        p = Vec3::random() * 2. - Vec3::ones();
        if p.length() < 1. {
            break;
        }
    }
    p
}

fn color(r: Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::default();
    //be lighter when using 0.001 instead of 0.
    //because more rays are missing, and then return background color
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) == true {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        color(Ray::new(rec.p, target - rec.p), world) * 0.5
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = unit_direction.y() * 0.5 + 0.5;
        Vec3::ones() * (1. - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    
    let mut list: HitableList = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));
    let camera = Camera::default();
    let mut rng = rand::thread_rng();
    //println!("{:?}", 0.5, 0.5);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zeros();
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / (nx as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (ny as f32);
                let r = camera.get_ray(u, v);
                col = col + color(r, &list);
            }
            col = col / (ns as f32);
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}