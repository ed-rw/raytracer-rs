use super::vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub a: vec3::Vec3,
    pub b: vec3::Vec3,
}

impl Ray {
    pub fn origin(self) -> vec3::Vec3 {
        self.a
    }

    pub fn direction(self) -> vec3::Vec3 {
        self.b
    }

    pub fn point_at_parameter(self, t: f32) -> vec3::Vec3 {
        self.a + t*self.b
    }
}
