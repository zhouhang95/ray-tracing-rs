use crate::vec3::{Vec3, cross};
use crate::ray::Ray;
use std::f32;
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let lens_radius = aperture / 2.;
        let theta = vfov * PI / 180.;
        let half_height = f32::tan(theta / 2.);
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = cross(vup, w).unit_vector();
        let v = cross(w, u);
        let lower_left_corner = origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * half_width * 2. * focus_dist;
        let vertical = v * half_height * 2. * focus_dist;
        
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }
    pub fn default() -> Camera {
        let lookfrom = Vec3::new(-2., 2., 1.);
        let lookat = Vec3::new(0., 0., -1.);
        let vup = Vec3::new(0., 1., 0.);
        let dist_to_focus = (lookfrom - lookat).length();
        let aperture = 1.;

        Camera::new(lookfrom, lookat, vup, 45., 2., aperture, dist_to_focus)
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let ray_start = self.origin + offset;
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v - ray_start;
        Ray::new(ray_start, direction)
    }
}