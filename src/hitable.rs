use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::aabb::AABB;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<usize>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: Option<usize>) -> HitRecord {
        HitRecord{
            t, 
            p, 
            normal,
            material,
        }
    }
    pub fn default() -> HitRecord {
        HitRecord{
            t: 0., 
            p: Vec3::zeros(), 
            normal: Vec3::zeros(),
            material: None,
        }
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool;
}
