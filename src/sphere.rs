use crate::vec3::{Vec3, dot};
use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::material::Material;
use crate::aabb::AABB;

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
    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool {
        let small = self.center - Vec3::new(self.radius, self.radius, self.radius);
        let big = self.center + Vec3::new(self.radius, self.radius, self.radius);
        AABB::new(small, big)
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: usize,
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32, material: usize) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f32) -> Vec3 {
        let ratio = (time - self.time0) / (self.time1 - self.time0);
        self.center0 + (self.center1 - self.center0) * ratio
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let mut temp;
            temp = (-b-discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.point_at_parameter(temp);
                *rec = HitRecord::new(temp, p, (p - self.center(r.time())) / self.radius, Some(self.material));
                return true;
            }
            temp = (-b+discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                let p = r.point_at_parameter(temp);
                *rec = HitRecord::new(temp, p, (p - self.center(r.time())) / self.radius, Some(self.material));
                return true;
            }
        }
        return false;
    }
}