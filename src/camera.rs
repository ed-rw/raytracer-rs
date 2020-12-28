use super::ray;
use super::vec3;
use std::f32::consts::PI;

pub struct Camera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: vec3::Vec3,
        lookat: vec3::Vec3,
        vup: vec3::Vec3,
        vfov: f32,
        aspect: f32,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        Self {
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        ray::Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
