pub trait Material {
    fn scatter(ray: Ray, rec: HitRecord) -> (Vec3, Ray);
}