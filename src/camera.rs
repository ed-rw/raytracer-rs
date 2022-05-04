use super::ray;
use super::vec3;
use std::f32::consts::PI;

pub trait Camera {
    fn get_ray(&self, u: f32, v: f32) -> ray::Ray;
}

#[derive(Copy, Clone)]
pub struct NoBlurCamera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}

impl NoBlurCamera {
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
}

impl Camera for NoBlurCamera {
    fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        ray::Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DefocusBlurCamera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
    u: vec3::Vec3,
    v: vec3::Vec3,
    w: vec3::Vec3,
    lens_radius: f32,
}

impl DefocusBlurCamera {
    pub fn new(
        lookfrom: vec3::Vec3,
        lookat: vec3::Vec3,
        vup: vec3::Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        Self {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
        }
    }

    fn random_in_unit_disk() -> vec3::Vec3 {
        let mut p: vec3::Vec3;
        loop {
            p = 2.0 * vec3::Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0)
                - vec3::Vec3::new(1, 1, 0);
            if !(p.dot(p) >= 1.0) {
                break;
            }
        }
        return p;
    }
}

impl Camera for DefocusBlurCamera {
    fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        let rd = self.lens_radius * DefocusBlurCamera::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        ray::Ray {
            a: self.origin + offset,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
        }
    }
}
