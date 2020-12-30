use super::material;
use super::ray;
use super::vec3;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub material: &'a Box<dyn material::Material>,
}

pub trait Hitable {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;

        let mut closest_so_far = t_max;
        for obj in self.list.iter() {
            let curr_rec = obj.hit(r, t_min, closest_so_far);
            if !curr_rec.is_none() {
                closest_so_far = curr_rec.unwrap().t;
                rec = curr_rec;
            }
        }
        return rec;
    }
}
