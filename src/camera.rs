use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn default() -> Camera {
        let lower_left_corner = Vec3::new(-2., -1., -1.);
        let horizontal = Vec3::new(4., 0., 0.);
        let vertical = Vec3::new(0., 2., 0.);
        let origin = Vec3::new(0., 0., 0.);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        Ray::new(self.origin, direction)
    }
}