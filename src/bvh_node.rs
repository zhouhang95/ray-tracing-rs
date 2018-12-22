struct BVHNode {
    bbox: AABB,
}

impl Hitable for BVHNode {
    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool {

    }
    
}