#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod hitable_list;
mod sphere;
mod camera;
mod material;

use crate::vec3::{Vec3, dot, ele_mul};
use crate::ray::Ray;
use crate::hitable_list::HitableList;
use crate::hitable::{HitRecord, Hitable};
use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::material::{Material, Lambertian, Metal};

use std::f32;
use rand::Rng;


fn color(r: Ray, world: &HitableList, materials: &Vec<Box<dyn Material>>, depth: i32) -> Vec3 {
    let mut rec = HitRecord::default();
    //be lighter when using 0.001 instead of 0.
    //because more rays are missing, and then return background color
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) == true {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::zeros();
        if depth < 50 && materials[rec.material.unwrap()].scatter(r, rec, &mut attenuation, &mut scattered) {
            return ele_mul(attenuation, color(scattered, world, materials, depth + 1));
        }else{
            return Vec3::zeros();
        }
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = unit_direction.y() * 0.5 + 0.5;
        Vec3::ones() * (1. - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let mut materials : Vec<Box<dyn Material>> = Vec::new();
    materials.push(Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))));
    materials.push(Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))));
    materials.push(Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)));
    materials.push(Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.8)));


    
    let mut list: HitableList = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, 0)));
    list.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100., 1)));
    list.push(Box::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, 2)));
    list.push(Box::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, 3)));
    
    let camera = Camera::default();
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zeros();
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / (nx as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (ny as f32);
                let r = camera.get_ray(u, v);
                col = col + color(r, &list, &materials, 0 );
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