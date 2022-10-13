use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign, Range},
};

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3([f64; 3]);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3([e0, e1, e2])
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }

    pub fn map<F: Fn(f64) -> f64>(self, f: &F) -> Vec3 {
        Vec3([f(self[0]), f(self[1]), f(self[2])])
    }

    pub fn sqrt(self) -> Vec3 {
        self.map(&f64::sqrt)
    }

    pub fn near_zero(self) -> bool {
        const EPSILON: f64 = 1.0e-8;
        
        self[0].abs() < EPSILON 
        && self[1].abs() < EPSILON 
        && self[2].abs() < EPSILON
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3([
            rng.gen_range(r.clone()), 
            rng.gen_range(r.clone()), 
            rng.gen_range(r.clone()),
        ])
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]]);
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]]);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3([self * rhs[0], self * rhs[1], self * rhs[2]])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}


impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3([self[0] / rhs, self[1] / rhs, self[2] / rhs]);
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}
