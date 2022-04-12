use std::fmt;
use std::ops::{self, Range};

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.zip_with(rhs, core::ops::Mul::mul)
            .redude(core::ops::Add::add)
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn cross(&self, rhs: &Vec3) -> Self {
        Self {
            e: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ],
        }
    }

    pub fn map(&self, mut f: impl FnMut(f64) -> f64) -> Self {
        Self {
            e: [f(self[0]), f(self[1]), f(self[2])],
        }
    }

    pub fn zip_with(&self, rhs: Vec3, mut f: impl FnMut(f64, f64) -> f64) -> Self {
        Self {
            e: [f(self[0], rhs[0]), f(self[1], rhs[1]), f(self[2], rhs[2])],
        }
    }

    pub fn redude(&self, f: impl Fn(f64, f64) -> f64) -> f64 {
        f(f(self[0], self[1]), self[2])
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }

    /// Pick a random point where `x`, `y`, `z` are all in `range`.
    pub fn random(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            e: [
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                rng.gen_range(range),
            ],
        }
    }

    /// Pick a random point in the unit sphere where `x`, `y`, and `z` are all in range from `-1..1`.
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    /// Pick a random point in hemisphere.
    pub fn random_in_hemisphere(norm: Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(norm) > 0.0 {
            // In the same hemisphere as the norm
            in_unit_sphere
        } else {
            -1.0 * in_unit_sphere
        }
    }

    /// Generate random point inside unit disk.
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    /// Check if vector is very close to zero in all dimensions.
    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(self, norm: Vec3) -> Self {
        self - 2.0 * self.dot(norm) * norm
    }

    pub fn refract(self, norm: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-1.0 * self).dot(norm).min(1.0);
        let ray_out_perp = etai_over_etat * (self + cos_theta * norm);
        let ray_out_parallel = -(1.0 - ray_out_perp.length().powi(2)).abs().sqrt() * norm;

        ray_out_perp + ray_out_parallel
    }
}

impl Color {
    /// Get color of pixel by taking the average over the number of `samples`.
    pub fn format_color(self, samples: u32) -> String {
        let ir = (256.0 * (self[0] / f64::from(samples)).sqrt().clamp(0.0, 0.999)) as u32;
        let ig = (256.0 * (self[1] / f64::from(samples)).sqrt().clamp(0.0, 0.999)) as u32;
        let ib = (256.0 * (self[2] / f64::from(samples)).sqrt().clamp(0.0, 0.999)) as u32;

        format!("{} {} {}", ir, ig, ib)
    }
}

impl rand::distributions::Distribution<Vec3> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3 {
            e: [rng.gen(), rng.gen(), rng.gen()],
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Add::add)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.zip_with(rhs, std::ops::Add::add);
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Sub::sub)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = self.zip_with(rhs, std::ops::Sub::sub);
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Mul::mul)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|x| x * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| self * x)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.map(|x| x * rhs);
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Div::div)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| self / x)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = self.map(|x| x / rhs);
    }
}
