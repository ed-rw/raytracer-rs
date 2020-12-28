use super::hitable;
use super::ray;
use super::sphere;
use super::vec3;

pub trait Material {
    fn scatter(&self, r_in: ray::Ray, rec: hitable::HitRecord) -> (vec3::Vec3, ray::Ray, bool);
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: vec3::Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: ray::Ray, rec: hitable::HitRecord) -> (vec3::Vec3, ray::Ray, bool) {
        let target = rec.p + rec.normal + sphere::random_in_unit_sphere();
        (
            self.albedo,
            ray::Ray {
                a: rec.p,
                b: target - rec.p,
            },
            true,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: vec3::Vec3,
    pub fuzz: f32,
}

impl Metal {
    fn reflect(&self, v: vec3::Vec3, n: vec3::Vec3) -> vec3::Vec3 {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: ray::Ray, rec: hitable::HitRecord) -> (vec3::Vec3, ray::Ray, bool) {
        let direction_unit = r_in.direction().clone();
        direction_unit.make_unit_vector();
        let reflected = self.reflect(direction_unit, rec.normal);
        let scattered = ray::Ray {
            a: rec.p,
            b: reflected + self.fuzz * sphere::random_in_unit_sphere(),
        };
        (
            self.albedo,
            scattered,
            (scattered.direction().dot(rec.normal)) > 0.0,
        )
    }
}
