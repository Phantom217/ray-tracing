//!

use rand::prelude::*;

/// A three-vector of floats, used as a color, coordinate, etc.
///
/// The components of the vector can be accessed in three ways:
///
/// 1. Tuple-style: `v.0`, `v.1`, `v.2`.
/// 2. Using the `Axis` enum: `v[X]`, `v[Y]`, `v[Z]`. This requires a `use
///    ray_tracing::vec3::Axis::*` statement.
/// 3. Using the `Channel` enum: `v[R]`, `v[G]`, `v[B]`. This requires a `use
///    ray_tracing::vec3::Channel::*` statement.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    /// Generates a random `Vec3` inside a sphere with unit radius. The length of the result is
    /// between 0 and 1.
    #[inline]
    pub fn in_unit_sphere(rng: &mut impl Rng) -> Self {
        loop {
            let v = 2. * rng.gen::<Vec3>() - Vec3::from(1.);
            if v.dot(v) < 1. {
                return v;
            }
        }
    }

    /// Generates a random `Vec3` inside a disc with unit radius in the XY plane. The length of the
    /// result is between 0 and 1, and the Z component is 0.
    #[inline]
    pub fn in_unit_disc(rng: &mut impl Rng) -> Self {
        loop {
            let v = 2. * Vec3(rng.gen(), rng.gen(), 0.) - Vec3(1., 1., 0.);
            if v.dot(v) < 1. {
                return v;
            }
        }
    }

    /// Computes the dot product of two vectors.
    #[inline]
    pub fn dot(&self, rhs: Self) -> f64 {
        self.zip_with(rhs, core::ops::Mul::mul)
            .reduce(core::ops::Add::add)
    }

    /// Computes the cross product of two vectors.
    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    /// Gets the length/magnitude of a vector.
    #[inline]
    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    /// Produces a vector collinear with `self` but with unit length. That is, the result points
    /// the same direction as `self` relative to the origin.
    #[inline]
    pub fn into_unit(self) -> Self {
        self / self.length()
    }

    /// Applies `f` to each element of the vector in turn, giving a new vector.
    #[inline]
    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }

    /// Combines each corresponding element of `self` and `rhs` by giving them as arguments to
    /// function `f`. The results are collected into a new vector.
    #[inline]
    pub fn zip_with(self, rhs: Vec3, mut f: impl FnMut(f64, f64) -> f64) -> Self {
        Self(f(self.0, rhs.0), f(self.1, rhs.1), f(self.2, rhs.2))
    }

    #[inline]
    pub fn zip_with3(
        self,
        other1: Vec3,
        other2: Vec3,
        mut f: impl FnMut(f64, f64, f64) -> f64,
    ) -> Self {
        Self(
            f(self.0, other1.0, other2.0),
            f(self.1, other1.1, other2.1),
            f(self.2, other1.2, other2.2),
        )
    }

    /// Combines the elements of `self` using `f` until only one result remains.
    #[inline]
    pub fn reduce(self, f: impl Fn(f64, f64) -> f64) -> f64 {
        f(f(self.0, self.1), self.2)
    }
}

/// Broadcasts a single value to all vector lanes.
impl From<f64> for Vec3 {
    #[inline]
    fn from(v: f64) -> Self {
        Self(v, v, v)
    }
}

/// Element-wise multiplication (Hadamard product).
impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Mul::mul)
    }
}

/// `scalar * vector`
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self) * rhs
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        self.zip_with(rhs, std::ops::Div::div)
    }
}

/// `vector / scalar`
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

/// `vector + vector`
impl std::ops::Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Add::add)
    }
}

/// `scalar + vector`
impl std::ops::Add<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| self + x)
    }
}

/// `vector - vector`
impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Sub::sub)
    }
}

/// `vector *= vector`
impl std::ops::MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = self.zip_with(rhs, std::ops::Mul::mul);
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        *self = self.zip_with(rhs, std::ops::Add::add);
    }
}

/// `-vector`
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(std::ops::Neg::neg)
    }
}

/// Allow accumulation of vectors from an iterator.
impl std::iter::Sum for Vec3 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::default(), std::ops::Add::add)
    }
}

/// Allow `Vec3` to be produced by `Rng::gen`.
///
/// The resulting vector has each component in the half-open range `[0,1)`. Note
/// that this is *not* a unit vector.
impl Distribution<Vec3> for rand::distributions::Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

/// Names for vector lanes when used as a color.
///
/// `Vec3` has an `Index` impl for `Channel`, so you can use `Channel` values to select components
/// from a `Vec3`:
///
/// ```
/// use ray_tracing::vec3::{Vec3, Channel::*};
///
/// let v = Vec3(1., 2., 3.);
/// assert_eq!(v[R], 1.);
/// assert_eq!(v[G], 2.);
/// assert_eq!(v[B], 3.);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Channel {
    /// Red.
    R,
    /// Green.
    G,
    /// Blue.
    B,
}

use Channel::*;

impl ::std::ops::Index<Channel> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: Channel) -> &Self::Output {
        match index {
            R => &self.0,
            G => &self.1,
            B => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<Channel> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: Channel) -> &mut Self::Output {
        match index {
            R => &mut self.0,
            G => &mut self.1,
            B => &mut self.2,
        }
    }
}

/// Names for vector lanes when used as a coordinate.
///
/// `Vec3` has an `Index` impl for `Axis`, so you can use `Axis` values to select components from a
/// `Vec3`:
///
/// ```
/// use ray_tracing::vec3::{Vec3, Axis::*};
///
/// let v = Vec3(1., 2., 3.);
/// assert_eq!(v[X], 1.);
/// assert_eq!(v[Y], 2.);
/// assert_eq!(v[Z], 3.);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

use Axis::*;

impl ::std::ops::Index<Axis> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            X => &self.0,
            Y => &self.1,
            Z => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<Axis> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            X => &mut self.0,
            Y => &mut self.1,
            Z => &mut self.2,
        }
    }
}

/// Reflects a vector `v` around a surface normal `n`.
#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

/// Refracts `v` into a material with surface normal `n`. `ni_over_nt` is the refractive index if
/// the ray is exiting the material, or its reciprocal if it's entering.
#[inline]
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.into_unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}
