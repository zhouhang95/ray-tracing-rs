use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    o: Vec3,
    d: Vec3,
}


impl Ray {
    pub fn default() -> Ray {
        Ray {
            o: Vec3::zeros(),
            d: Vec3::zeros(),
        }
    }
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            o: origin,
            d: direction,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.o
    }

    pub fn direction(&self) -> Vec3 {
        self.d
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.o + self.d * t
    }
}