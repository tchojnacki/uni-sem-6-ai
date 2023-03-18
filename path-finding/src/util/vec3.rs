use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn len_2(self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn len(self) -> f64 {
        self.len_2().sqrt()
    }

    pub fn cross(self, other: Self) -> Self {
        // https://en.wikipedia.org/wiki/Cross_product
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

#[cfg(test)]
mod tests {
    use super::Vec3;
    const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);

    #[test]
    fn len_and_len_2_return_correct_result() {
        assert_eq!(Vec3(1.0, 2.0, -2.0).len(), 3.0);
        assert_eq!(Vec3(2.0, -10.0, 11.0).len(), 15.0);
        assert_eq!(ZERO.len(), 0.0);
        assert_eq!(ZERO.len_2(), 0.0);
        assert_eq!(Vec3(1.0, 2.0, 3.0).len_2(), 14.0);
        assert_eq!(Vec3(-10.0, 10.0, -10.0).len_2(), 300.0);
    }

    #[test]
    fn cross_product_uses_the_valid_formula() {
        assert_eq!(
            Vec3(2.0, 3.0, 4.0).cross(Vec3(5.0, 6.0, 7.0)),
            Vec3(-3.0, 6.0, -3.0)
        );
        assert_eq!(ZERO.cross(ZERO), ZERO);
    }

    #[test]
    fn cross_product_returns_perpendicular_vector() {
        let a = Vec3(0.1, 3.15, 0.0);
        let b = Vec3(2.0, 1.0, -3.0);
        let c = a.cross(b);
        assert!(c.dot(a) < 0.01);
        assert!(c.dot(b) < 0.01);
    }

    #[test]
    fn normalized_vector_is_a_unit_vector() {
        let vecs = [
            Vec3(1.23, 8.12, 5.01),
            Vec3(1.0, 0.0, 0.0),
            Vec3(-8.0, -8.0, -8.0),
            Vec3(0.0, 0.0, 0.01),
        ];

        for vec in vecs {
            assert!((vec.normalized().len() - 1.0).abs() < 0.01);
        }
    }
}
