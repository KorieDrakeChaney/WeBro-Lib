use std::ops::{Add, Sub, Mul, Div};
use num::*;
use crate::vectors::{Vector2, Vector4};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3<T : Num> {
    pub x : T, 
    pub y : T, 
    pub z : T, 
}

impl<T : Num> Vector3<T> {
    pub fn new(x : T, y : T, z : T) -> Self {
        Self {
            x, 
            y, 
            z, 
        }
    }   
}

impl Default for Vector3<f32> {
    fn default() -> Self {
        Self {
            x : 0.0,
            y : 0.0,
            z : 0.0,
        }
    }
}

impl Default for Vector3<i32> {
    fn default() -> Self {
        Self {
            x : 0,
            y : 0,
            z : 0,
        }
    }
}

 
impl<T: Num> Add<Self> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn add(self, other : Self) -> Self{
        Self {
            x : self.x + other.x, 
            y : self.y + other.y,
            z : self.z + other.z, 
        }
    }
}

impl<T: Num> Sub<Self> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn sub(self, other : Self) -> Self{
        Self {
            x : self.x - other.x, 
            y : self.y - other.y,
            z : self.z - other.z, 
        }
    }
}

impl<T: Num> Mul<Self> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn mul(self, other : Self) -> Self{
        Self {
            x : self.x * other.x, 
            y : self.y * other.y,
            z : self.z * other.z, 
        }
    }
}

impl<T: Num> Div<Self> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn div(self, other : Self) -> Self{
        Self {
            x : self.x / other.x, 
            y : self.y / other.y,
            z : self.z / other.z, 
        }
    }
}

impl<T: Num> Add<Vector4<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn add(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x + other.x, 
            y : self.y + other.y,
            z : self.z + other.z,
        }
    }
}

impl<T: Num> Sub<Vector4<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn sub(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x - other.x, 
            y : self.y - other.y,
            z : self.z - other.z,
        }
    }
}

impl<T: Num> Mul<Vector4<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn mul(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x * other.x, 
            y : self.y * other.y,
            z : self.z * other.z,
        }
    }
}

impl<T: Num> Div<Vector4<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn div(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x / other.x, 
            y : self.y / other.y,
            z : self.z / other.z,
        }
    }
}

impl<T: Num> Add<Vector2<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn add(self, other : Vector2<T>) -> Self{
        Self {
            x : self.x + other.x, 
            y : self.y + other.y,
            z : self.z,
        }
    }
}

impl<T: Num> Sub<Vector2<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn sub(self, other : Vector2<T>) -> Self{
        Self {
            x : self.x - other.x, 
            y : self.y - other.y,
            z : self.z,
        }
    }
}

impl<T: Num> Mul<Vector2<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn mul(self, other : Vector2<T>) -> Self{
        Self {
            x : self.x * other.x, 
            y : self.y * other.y,
            z : self.z,
        }
    }
}

impl<T: Num> Div<Vector2<T>> for Vector3<T> {
    type Output = Self;
    #[must_use]
    fn div(self, other : Vector2<T>) -> Self{
        Self {
            x : self.x / other.x, 
            y : self.y / other.y,
            z : self.z,
        }
    }
}