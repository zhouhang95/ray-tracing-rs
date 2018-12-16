use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use std::f32;

use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32;3],
}

impl Vec3 {
    pub fn zeros() -> Vec3{
        Vec3 {
            e: [0., 0., 0.],
        }
    }

    pub fn ones() -> Vec3 {
        Vec3 {
            e: [1., 1., 1.,],
        }
    }
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
            let p = p * 2. - Vec3::ones();
            if p.length() < 1. {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.);
            let p = p * 2. - Vec3::new(1., 1., 0.);
            if p.length() < 1. {
                return p;
            }
        }
    }
    

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            e: [x, y, z,],
        }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn squared_length(&self) -> f32 {
        dot(*self, *self)
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn unit_vector_(&mut self) {
        *self = *self /self.length()
    }

}


impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e : [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e : [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ]
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other,
            ]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        self * (1. / other)
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        e : [
            v1.e[1] * v2.e[2] - v2.e[1] * v1.e[2],
            v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
            v1.e[0] * v2.e[1] - v2.e[0] * v1.e[1],
        ]
    }
}

pub fn ele_mul(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(v1.e[0] * v2.e[0], v1.e[1] * v2.e[1], v1.e[2] * v2.e[2])    
}


