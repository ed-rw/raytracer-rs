use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn x(self) -> f32 {
        self.e[0]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn r(self) -> f32 {
        self.e[0]
    }

    pub fn g(self) -> f32 {
        self.e[1]
    }

    pub fn b(self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (&self.e[0]*&self.e[0]
         + &self.e[1]*&self.e[1]
         + &self.e[2]*&self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        &self.e[0]*&self.e[0] + &self.e[1]*&self.e[1] + &self.e[2]*&self.e[2]
    }

    pub fn make_unit_vector(mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn dot(&self, v2: Vec3) -> f32 {
        &self.e[0]*v2.e[0] + &self.e[1]*v2.e[1] + &self.e[2]*v2.e[2]
    }

    pub fn cross(&self, v2: Vec3) -> Vec3 {
        Vec3 { e: [&self.e[1]*v2.e[2] - &self.e[2]*v2.e[1],
                   -(&self.e[0]*v2.e[2] - &self.e[2]*v2.e[0]),
                   &self.e[0]*v2.e[1] - &self.e[1]*v2.e[0]] }
    }
}

// overload operators
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self { e: [self.e[0]+rhs.e[0], self.e[1]+rhs.e[1], self.e[2]+rhs.e[2]] }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] + rhs.e[0],
                           self.e[1] + rhs.e[1],
                           self.e[2] + rhs.e[2],]
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self { e: [self.e[0]-rhs.e[0], self.e[1]-rhs.e[1], self.e[2]-rhs.e[2]] }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] - rhs.e[0],
                           self.e[1] - rhs.e[1],
                           self.e[2] - rhs.e[2],]
        };
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self { e: [rhs.e[0]*self.e[0], rhs.e[1]*self.e[1], rhs.e[2]*self.e[2]] }
    }
}

// Vec3 * float;
impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self { e: [rhs*self.e[0], rhs*self.e[1], rhs*self.e[2]] }
    }
}

// float * Vec3
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output { e: [self*rhs.e[0], self*rhs.e[1], self*rhs.e[2]] }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] * rhs.e[0],
                           self.e[1] * rhs.e[1],
                           self.e[2] * rhs.e[2],]
        };
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self { e: [self.e[0] * rhs,
                           self.e[1] * rhs,
                           self.e[2] * rhs,]
        };
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Self::Output {
        Self { e: [self.e[0]/rhs.e[0], self.e[1]/rhs.e[1], self.e[2]/rhs.e[2]] }
    }
}

//divide for Vec3, float
impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self { e: [self.e[0]/rhs, self.e[1]/rhs, self.e[2]/rhs] }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self { e: [self.e[0] / rhs.e[0],
                           self.e[1] / rhs.e[1],
                           self.e[2] / rhs.e[2],]
        };
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let k = 1.0/rhs;
        *self = Self { e: [self.e[0] / k,
                           self.e[1] / k,
                           self.e[2] / k,]
        };
    }
}
