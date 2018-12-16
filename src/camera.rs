use crate::vec3::{Vec3, cross};
use crate::ray::Ray;
use std::f32;
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * PI / 180.;
        let half_height = f32::tan(theta / 2.);
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = cross(vup, w).unit_vector();
        let v = cross(w, u);
        let lower_left_corner = origin - u * half_width - v * half_height - w;
        let horizontal = u * half_width * 2.;
        let vertical = v * half_height * 2.;
        
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn default() -> Camera {
        let lookfrom = Vec3::new(-2., 2., 1.);
        let lookat = Vec3::new(0., 0., -1.);
        let vup = Vec3::new(0., 1., 0.);
        Camera::new(lookfrom, lookat, vup, 45., 2.)
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        Ray::new(self.origin, direction)
    }
}