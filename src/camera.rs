use super::ray;
use super::vec3;

pub struct Camera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        ray::Ray{ a: self.origin,
                  b: self.lower_left_corner + u*self.horizontal
                     + v*self.vertical - self.origin}
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            lower_left_corner: vec3::Vec3{ e: [-2.0, -1.0, -1.0]},
            horizontal: vec3::Vec3{ e: [4.0, 0.0, 0.0]},
            vertical: vec3::Vec3{ e: [0.0, 2.0, 0.0]},
            origin: vec3::Vec3{ e: [0.0, 0.0, 0.0]},
        }
    }
}
