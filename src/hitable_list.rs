use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::ray::Ray;

pub type HitableList = Vec<Box<dyn Hitable>>;

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        for item in self.iter() {
            let ret = item.hit(r, t_min, closest_so_far, &mut temp_rec);
            if ret == true {                
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;                
            }
        }
        hit_anything
    }
}