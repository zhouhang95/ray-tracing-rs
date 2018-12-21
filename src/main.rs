#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_assignments)]

extern crate rand;
extern crate num_cpus;
extern crate threadpool;

mod vec3;
mod ray;
mod hitable;
mod hitable_list;
mod sphere;
mod camera;
mod material;
mod image;

use crate::vec3::{Vec3, dot, ele_mul};
use crate::ray::Ray;
use crate::hitable_list::HitableList;
use crate::hitable::{HitRecord, Hitable};
use crate::camera::Camera;
use crate::sphere::{Sphere, MovingSphere};
use crate::material::{Material, Lambertian, Metal, Dielectric};
use crate::image::Image;

use std::f32;
use std::sync::mpsc::channel;
use std::sync::Arc;
use rand::Rng;
use threadpool::ThreadPool;


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
    let nx = 600;
    let ny = 300;
    let ns = 1000;
    let mut image = Image::new(nx, ny);
    
    let mut materials : Vec<Box<dyn Material>> = Vec::new();
    materials.push(Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))));
    materials.push(Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))));
    materials.push(Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)));
    materials.push(Box::new(Dielectric::new(1.5)));


    
    let mut list: HitableList = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, 0)));
    list.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100., 1)));
    list.push(Box::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, 2)));
    list.push(Box::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, 3)));

    let (list, materials) = random_scene();
    let (list, materials) = (Arc::new(list), Arc::new(materials));
    let camera = Camera::default();
    let (tx, rx) = channel();
    let pool = ThreadPool::new(num_cpus::get());




    for j in (0..ny).rev() {
        let tx = tx.clone();
        let list = list.clone();
        let materials = materials.clone();

        pool.execute(move || {
            let mut rng = rand::thread_rng();
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
                tx.send((i, j, col)).unwrap();
            }
        });
    }
    drop(tx);
    while let Ok((i, j, col)) = rx.recv() {
        image.set_pixel(i, j, col);
    }
    image.dump();
}

fn random_scene() -> (HitableList, Vec<Box<dyn Material>>) {
    let mut rng = rand::thread_rng();
    let mut i = 0;
    let mut list: HitableList = Vec::new();
    let mut materials : Vec<Box<dyn Material>> = Vec::new();
    materials.push(Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))));
    list.push(Box::new(Sphere::new(Vec3::new(0., -1000., -1.), 1000., i)));
    i += 1;
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    materials.push(Box::new(Lambertian::new(ele_mul(Vec3::random(), Vec3::random()))));
                    //list.push(Box::new(Sphere::new(center, 0.2, i)));
                    let center0 = center;
                    let center1 = center + Vec3::new(0., 0.5 * rng.gen::<f32>(), 0.);
                    list.push(Box::new(MovingSphere::new(center0, center1, 0., 1., 0.2, i)));

                } else if choose_mat < 0.95 {
                    materials.push(Box::new(Metal::new( (Vec3::random() + Vec3::ones()) * 0.5, 0.5 * rng.gen::<f32>() )));
                    list.push(Box::new(Sphere::new(center, 0.2, i)));
                } else {
                    materials.push(Box::new(Dielectric::new(1.5)));
                    list.push(Box::new(Sphere::new(center, 0.2, i)));
                }
                i += 1;
            }
        }
    }
    materials.push(Box::new(Dielectric::new(1.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1., i)));
    i += 1;

    materials.push(Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))));
    list.push(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., i)));
    i += 1;

    materials.push(Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)));
    list.push(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., i)));
    i += 1;

    (list, materials)

}