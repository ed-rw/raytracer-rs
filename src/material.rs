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
        let reflected = self.reflect(vec3::unit_vector(r_in.direction()), rec.normal);
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

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    fn reflect(&self, v: vec3::Vec3, n: vec3::Vec3) -> vec3::Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    fn refract(&self, v: vec3::Vec3, n: vec3::Vec3, ni_over_nt: f32) -> Option<vec3::Vec3> {
        let uv = vec3::unit_vector(v);

        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            return Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt());
        } else {
            return None;
        }
    }

    fn schlick(&self, cosine: f32) -> f32 {
        let r0 = ((1.0 - self.refraction_index) / (1.0 + self.refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: ray::Ray, rec: hitable::HitRecord) -> (vec3::Vec3, ray::Ray, bool) {
        let attenuation = vec3::Vec3 { e: [1.0, 1.0, 1.0] };

        let outward_normal: vec3::Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        if r_in.direction().dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * r_in.direction().dot(rec.normal)
                / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -1.0 * (r_in.direction().dot(rec.normal) / r_in.direction().length());
        }

        let wrapped_refracted = self.refract(r_in.direction(), outward_normal, ni_over_nt);
        if !wrapped_refracted.is_none() {
            let refracted = wrapped_refracted.unwrap();
            if rand::random::<f32>() > self.schlick(cosine) {
                return (
                    attenuation,
                    ray::Ray {
                        a: rec.p,
                        b: refracted,
                    },
                    true,
                );
            }
        }

        let reflected = self.reflect(r_in.direction(), rec.normal);
        return (
            attenuation,
            ray::Ray {
                a: rec.p,
                b: reflected,
            },
            true,
        );
    }
}
