use crate::vec3::Vec3;

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(small: Vec3, big: Vec3) -> AABB {
        AABB {
            min: small,
            max: big,
        }
    }
    fn hit(&self, r: Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for i in 0..3 {
            let invD = 1. / r.direction().e[i];
            let t0 = (self.min.e[i] - r.origin().e[i]) * invD;
            let t1 = (self.max.e[i] - r.origin().e[i]) * invD;
            let (t0, t1) = if invD < 0.{(t1, t0)} else {(t0, t1)};
            tmin = if t0 > tmin {t0} else {tmin};
            tmax = if t1 < tmax {t1} else {tmax};
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
}