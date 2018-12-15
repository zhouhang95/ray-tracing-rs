use crate::vec3::{Vec3, dot};
use crate::ray::Ray;
use crate::hitable::{HitRecord, Hitable};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo,
        }
    }   
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        return true;
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * dot(v, n) * 2.
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz,
        }
    }   
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;
        return dot(scattered.direction(), rec.normal) > 0.;
    }
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = v.unit_vector();
    let dt = dot(uv, n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        *refracted =  (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
        true
    }else{
        false
    }
}


pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {   
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric {
            ref_idx,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::ones();
        let (out_normal, ni_over_nt) = if dot(r_in.direction(), rec.normal) < 0. {
            (rec.normal, 1. / self.ref_idx)
        } else {
            (rec.normal * (-1.), self.ref_idx)
        };
        let mut refracted = Vec3::zeros();
        if refract(r_in.direction(), out_normal, ni_over_nt, &mut refracted) {
            *scattered = Ray::new(rec.p, refracted);
            true
        } else {
            let reflected = reflect(r_in.direction(), rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            false
        }
    }
}