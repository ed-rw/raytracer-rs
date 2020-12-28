use super::hitable;
use super::material;
use super::ray;
use super::vec3;

#[derive(Copy, Clone)]
pub struct Sphere<'a> {
    pub center: vec3::Vec3,
    pub radius: f32,
    pub material: &'a dyn material::Material,
}

impl hitable::Hitable for Sphere<'_> {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32) -> Option<hitable::HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(hitable::HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: self.material,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(hitable::HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: self.material,
                });
            }
        }
        return None;
    }
}

pub fn random_in_unit_sphere() -> vec3::Vec3 {
    let mut p: vec3::Vec3;
    loop {
        p =
            2.0 * vec3::Vec3 {
                e: [
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                ],
            } - vec3::Vec3 { e: [1.0, 1.0, 1.0] };
        if !(p.squared_length() >= 1.0) {
            break;
        }
    }
    p
}
