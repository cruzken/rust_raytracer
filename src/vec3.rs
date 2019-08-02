use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(&self) -> Vec3 {
        let k: f32 = 1.0 / self.length();
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            -(self.x * other.z - self.z * other.x),
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = thread_rng();
    loop {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

#[cfg(test)]
mod tests {

    use super::Vec3;

    #[test]
    fn length() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length(), 3.7416575);
    }

    #[test]
    fn squared_length() {
        let v = Vec3::new(4.0, 4.0, 1.0);
        assert_eq!(v.squared_length(), 33.0);
    }

    #[test]
    fn unit() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.unit(), Vec3::new(0.26726124, 0.5345225, 0.8017837));
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.dot(&v2), 14.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.cross(&v2), Vec3::new(0.0, -0.0, 0.0));
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v1 + v2, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn add_assign() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec += Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v1 - v2, Vec3::new(-2.0, -3.0, -4.0));
    }

    #[test]
    fn sub_assign() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec -= Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(vec, Vec3::new(-1.0, 1.0, 3.0));
    }

    #[test]
    fn mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v1 * v2, Vec3::new(3.0, 8.0, 15.0));
    }

    #[test]
    fn mul_f32() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 3.0, Vec3::new(3.0, 6.0, 9.0));
        assert_eq!(3.0 * v, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn mul_assign() {
        let mut vec = Vec3::new(3.0, 2.0, 3.0);
        vec *= Vec3::new(2.0, 4.0, 7.0);
        assert_eq!(vec, Vec3::new(6.0, 8.0, 21.0));
    }

    #[test]
    fn mul_assign_f32() {
        let mut vec = Vec3::new(3.0, 2.0, 1.0);
        vec *= 9.0;
        assert_eq!(vec, Vec3::new(27.0, 18.0, 9.0));
    }

    #[test]
    fn div() {
        let v1 = Vec3::new(15.0, 8.0, 20.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v1 / v2, Vec3::new(5.0, 2.0, 4.0));
    }

    #[test]
    fn div_f32() {
        let v = Vec3::new(10.0, 4.0, 8.0);
        assert_eq!(v / 2.0, Vec3::new(5.0, 2.0, 4.0));
    }

    #[test]
    fn div_assign() {
        let mut vec = Vec3::new(15.0, 8.0, 20.0);
        vec /= Vec3::new(3.0, 2.0, 4.0);
        assert_eq!(vec, Vec3::new(5.0, 4.0, 5.0));
    }

    #[test]
    fn div_assign_f32() {
        let mut vec = Vec3::new(18.0, 4.0, 24.0);
        vec /= 2.0;
        assert_eq!(vec, Vec3::new(9.0, 2.0, 12.0));
    }

    #[test]
    fn neg() {
        let vec = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(-vec, Vec3::new(-1.0, -1.0, -1.0));
    }
}
