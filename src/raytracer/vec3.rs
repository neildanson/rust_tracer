use std::f64;
use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const ZERO: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z       }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}


impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Vec3 {
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn length(&self) -> f64 {
        let d = self.dot(self);
        d.sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();
        let div = if length == 0.0 {
            f64::INFINITY
        } else {
            1.0 / length
        };
        *self * div
    }

    pub fn to_color(&self) -> [u8; 4] {
        let r = self.x.min(1.0).max(0.0) * 255.0;
        let g = self.y.min(1.0).max(0.0) * 255.0;
        let b = self.z.min(1.0).max(0.0) * 255.0;

        [r as u8, g as u8, b as u8, 255u8]
    }
}