use glam::{dvec3, DVec3};
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone)]
pub enum Angle {
    Radians(f64),
    Degrees(f64),
}

impl Angle {
    pub fn radians(self) -> f64 {
        match self {
            Self::Radians(r) => r,
            Self::Degrees(d) => (d * std::f64::consts::PI) / 180.0,
        }
    }

    pub fn degrees(self) -> f64 {
        match self {
            Self::Radians(r) => (r * 180.0) / std::f64::consts::PI,
            Self::Degrees(d) => d,
        }
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, multiplier: f64) -> Self::Output {
        match self {
            Self::Radians(angle) => Self::Radians(angle * multiplier),
            Self::Degrees(angle) => Self::Degrees(angle * multiplier),
        }
    }
}

impl Div<f64> for Angle {
    type Output = Angle;

    fn div(self, divisor: f64) -> Self::Output {
        match self {
            Self::Radians(angle) => Self::Radians(angle / divisor),
            Self::Degrees(angle) => Self::Degrees(angle / divisor),
        }
    }
}

pub trait ToAngle {
    fn degrees(&self) -> Angle;
    fn radians(&self) -> Angle;
}

impl<T: Into<f64> + Copy> ToAngle for T {
    fn degrees(&self) -> Angle {
        Angle::Degrees((*self).into())
    }

    fn radians(&self) -> Angle {
        Angle::Radians((*self).into())
    }
}

/// Represents rotation on the X, Y, and Z axes. Also known
/// as Euler angle representation.
#[derive(Debug, Copy, Clone)]
pub struct RVec {
    pub x: Angle,
    pub y: Angle,
    pub z: Angle,
}

impl RVec {
    pub fn radians(&self) -> DVec3 {
        dvec3(self.x.radians(), self.y.radians(), self.z.radians())
    }

    pub fn degrees(&self) -> DVec3 {
        dvec3(self.x.degrees(), self.y.degrees(), self.z.degrees())
    }

    pub fn x(x: Angle) -> Self {
        RVec { x, y: 0.degrees(), z: 0.degrees() }
    }

    pub fn y(y: Angle) -> Self {
        RVec { x: 0.degrees(), y, z: 0.degrees() }
    }

    pub fn z(z: Angle) -> Self {
        RVec { x: 0.degrees(), y: 0.degrees(), z }
    }
}

pub fn rvec(x: Angle, y: Angle, z: Angle) -> RVec {
    RVec { x, y, z }
}
