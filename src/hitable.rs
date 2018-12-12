use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord{
            t, 
            p, 
            normal,
        }
    }
    pub fn default() -> HitRecord {
        HitRecord{
            t: 0., 
            p: Vec3::zeros(), 
            normal: Vec3::zeros(),
        }
    }
}

pub trait Hitable{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
