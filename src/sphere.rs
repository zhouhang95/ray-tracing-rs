use crate::vec3::{Vec3, dot};
use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::material::Material;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: usize,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: usize) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let mut temp;
            temp = (-b-discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.point_at_parameter(temp);
                *rec = HitRecord::new(temp, p, (p - self.center) / self.radius, Some(self.material));
                return true;
            }
            temp = (-b+discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.point_at_parameter(temp);
                *rec = HitRecord::new(temp, p, (p - self.center) / self.radius, Some(self.material));
                return true;
            }
        }
        return false;
    }
}