use super::hitable;
use super::vec3;
use super::ray;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: vec3::Vec3,
    pub radius: f32
}

impl hitable::Hitable for Sphere {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32) -> Option<hitable::HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt())/ a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(hitable::HitRecord{
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(hitable::HitRecord{
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            }
        }
        return None;
    }
}
