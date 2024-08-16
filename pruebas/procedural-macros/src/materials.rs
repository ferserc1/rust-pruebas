
pub mod materials {
    pub trait Material {
        fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    }
}