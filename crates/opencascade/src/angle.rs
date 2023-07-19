use glam::{dvec3, DVec3};
use std::{
    f64::consts::PI,
    ops::{Div, Mul},
};

#[derive(Debug, Copy, Clone)]
pub struct Radians(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct Degrees(pub f64);

impl From<Degrees> for Radians {
    fn from(Degrees(d): Degrees) -> Self {
        Self(d * PI / 180.0)
    }
}

impl From<Radians> for Degrees {
    fn from(Radians(r): Radians) -> Self {
        Self(r * 180.0 / PI)
    }
}

impl From<Radians> for f64 {
    fn from(Radians(r): Radians) -> Self {
        r
    }
}

impl From<Degrees> for f64 {
    fn from(Degrees(d): Degrees) -> Self {
        d
    }
}

impl Mul<f64> for Radians {
    type Output = Radians;

    fn mul(self, multiplier: f64) -> Self::Output {
        Radians(self.0 * multiplier)
    }
}

impl Mul<f64> for Degrees {
    type Output = Degrees;

    fn mul(self, multiplier: f64) -> Self::Output {
        Degrees(self.0 * multiplier)
    }
}

impl Div<f64> for Radians {
    type Output = Radians;

    fn div(self, divisor: f64) -> Self::Output {
        Radians(self.0 / divisor)
    }
}

impl Div<f64> for Degrees {
    type Output = Degrees;

    fn div(self, divisor: f64) -> Self::Output {
        Degrees(self.0 / divisor)
    }
}

pub trait ToAngle {
    fn degrees(&self) -> Degrees;
    fn radians(&self) -> Radians;
}

impl<T: Into<f64> + Copy> ToAngle for T {
    fn degrees(&self) -> Degrees {
        Degrees((*self).into())
    }

    fn radians(&self) -> Radians {
        Radians((*self).into())
    }
}

/// Represents rotation on the X, Y, and Z axes. Also known
/// as Euler angle representation.
#[derive(Debug, Copy, Clone)]
pub struct RVec {
    pub x: Radians,
    pub y: Radians,
    pub z: Radians,
}

impl RVec {
    pub fn x(x: impl Into<Radians>) -> Self {
        RVec { x: x.into(), y: 0.radians(), z: 0.radians() }
    }

    pub fn y(y: impl Into<Radians>) -> Self {
        RVec { x: 0.radians(), y: y.into(), z: 0.radians() }
    }

    pub fn z(z: impl Into<Radians>) -> Self {
        RVec { x: 0.radians(), y: 0.radians(), z: z.into() }
    }

    pub fn radians(&self) -> DVec3 {
        dvec3(self.x.into(), self.y.into(), self.z.into())
    }

    pub fn degrees(&self) -> DVec3 {
        dvec3(self.x.degrees().into(), self.y.degrees().into(), self.z.degrees().into())
    }
}

pub fn rvec(x: impl Into<Radians>, y: impl Into<Radians>, z: impl Into<Radians>) -> RVec {
    RVec { x: x.into(), y: y.into(), z: z.into() }
}
