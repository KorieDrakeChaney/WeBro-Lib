use std::ops::{Add, Sub, Mul, Div};
use num::*;
use crate::vectors::{Vector3, Vector4};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2<T : Num> {
    pub x : T, 
    pub y : T, 
}


impl<T : Num> Vector2<T> {
    pub fn new(x : T, y : T) -> Self {
        Self {
            x, 
            y, 
        }
    }   
}
 
impl Default for Vector2<f32> {
    fn default() -> Self {
        Self {
            x : 0.0,
            y : 0.0,
        }
    }
}

impl Default for Vector2<i32> {
    fn default() -> Self {
        Self {
            x : 0,
            y : 0,
        }
    }
}

impl<T: Num> Add<Self> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn add(self, other : Self) -> Self{
        Self {
            x : self.x + other.x, 
            y : self.y + other.y,
        }
    }
}

impl<T: Num> Sub<Self> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn sub(self, other : Self) -> Self{
        Self {
            x : self.x - other.x, 
            y : self.y - other.y,
        }
    }
}

impl<T: Num> Mul<Self> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn mul(self, other : Self) -> Self{
        Self {
            x : self.x * other.x, 
            y : self.y * other.y,
        }
    }
}

impl<T: Num> Div<Self> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn div(self, other : Self) -> Self{
        Self {
            x : self.x / other.x, 
            y : self.y / other.y,
        }
    }
}



impl<T: Num> Add<Vector3<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn add(self, other : Vector3<T>) -> Self{
        Self {
            x : self.x + other.x, 
            y : self.y + other.y,
        }
    }
}

impl<T: Num> Sub<Vector3<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn sub(self, other : Vector3<T>) -> Self{
        Self {
            x : self.x - other.x, 
            y : self.y - other.y,
        }
    }
}

impl<T: Num> Mul<Vector3<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn mul(self, other : Vector3<T>) -> Self{
        Self {
            x : self.x * other.x, 
            y : self.y * other.y,
        }
    }
}

impl<T: Num> Div<Vector3<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn div(self, other : Vector3<T>) -> Self{
        Self {
            x : self.x / other.x, 
            y : self.y / other.y,
        }
    }
}

impl<T: Num> Add<Vector4<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn add(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x + other.x, 
            y : self.y + other.y,
        }
    }
}

impl<T: Num> Sub<Vector4<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn sub(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x - other.x, 
            y : self.y - other.y,
        }
    }
}

impl<T: Num> Mul<Vector4<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn mul(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x * other.x, 
            y : self.y * other.y,
        }
    }
}

impl<T: Num> Div<Vector4<T>> for Vector2<T> {
    type Output = Self;
    #[must_use]
    fn div(self, other : Vector4<T>) -> Self{
        Self {
            x : self.x / other.x, 
            y : self.y / other.y,
        }
    }
}