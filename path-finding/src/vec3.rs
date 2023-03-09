use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn len_2(self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn len(self) -> f64 {
        self.len_2().sqrt()
    }

    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn normalized(self) -> Self {
        self / self.len()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Self::Output {
        Self(self.0 * k, self.1 * k, self.2 * k)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, k: f64) -> Self::Output {
        self * (1.0 / k)
    }
}
