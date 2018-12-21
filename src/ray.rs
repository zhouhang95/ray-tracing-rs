use crate::vec3::Vec3;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    o: Vec3,
    d: Vec3,
    t: f32,
}


impl Ray {
    pub fn default() -> Ray {
        Ray {
            o: Vec3::zeros(),
            d: Vec3::zeros(),
            t: 0.,
        }
    }
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            o: origin,
            d: direction,
            t: time,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.o
    }

    pub fn direction(&self) -> Vec3 {
        self.d
    }
    pub fn time(&self) -> f32 {
        self.t
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.o + self.d * t
    }
}