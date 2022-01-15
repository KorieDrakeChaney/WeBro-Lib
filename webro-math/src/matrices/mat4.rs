use crate::vectors::Vector4;
use crate::traits::{Identity, IdentityTransorm};
use std::ops::{Add, Sub, Mul, Div};
use num::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4<T : Num> {
    pub x : Vector4<T>,
    pub y : Vector4<T>,
    pub z : Vector4<T>,
    pub w : Vector4<T>,
    
} 

impl Default for Mat4<f32> {
    fn default() -> Self {
        Self {
            x : Vector4::default(),
            y : Vector4::default(),
            z : Vector4::default(),
            w : Vector4::default(),
            
        }
    }
}

impl Default for Mat4<i32> {
    fn default() -> Self {
        Self {
            x : Vector4::default(),
            y : Vector4::default(),
            z : Vector4::default(),
            w : Vector4::default(),
            
        }
    }
}

impl Identity for Mat4<f32> {
    fn identity() -> Self {
        Self {
            x : Vector4::new(1.0, 0.0, 0.0, 0.0), 
            y : Vector4::new(0.0, 1.0, 0.0, 0.0), 
            z : Vector4::new(0.0, 0.0, 1.0, 0.0), 
            w : Vector4::new(0.0, 0.0, 0.0, 1.0), 
            
        }
    }
}

impl Identity for Mat4<i32> {
    fn identity() -> Self {
        Self {
            x : Vector4::new(1, 0, 0, 0), 
            y : Vector4::new(0, 1, 0, 0), 
            z : Vector4::new(0, 0, 1, 0), 
            w : Vector4::new(0, 0, 0, 1), 
            
        }
    }
}

impl IdentityTransorm for Mat4<f32> {
    fn identity_transform(&mut self){
        self.x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        self.y = Vector4::new(0.0, 1.0, 0.0, 0.0); 
        self.z = Vector4::new(0.0, 0.0, 1.0, 0.0); 
        self.w = Vector4::new(0.0, 0.0, 0.0, 1.0); 
    }
}



impl IdentityTransorm for Mat4<i32> {
    fn identity_transform(&mut self){
        self.x = Vector4::new(1, 0, 0, 0);
        self.y = Vector4::new(0, 1, 0, 0); 
        self.z = Vector4::new(0, 0, 1, 0); 
        self.w = Vector4::new(0, 0, 0, 1); 
    }
}

impl<T : Num> Mat4<T> {
    pub fn new(x : Vector4<T>, y : Vector4<T>, z : Vector4<T>, w : Vector4<T>) -> Self{
        Self {
            x, 
            y, 
            z, 
            w, 
            
        }
    }
}

impl<T : Num> Add<Self> for Mat4<T> {
    type Output = Self;
    fn add(self, other : Self) -> Self {
        Self {
            x : self.x + other.x, 
            y : self.y + other.y, 
            z : self.z + other.z, 
            w : self.w + other.w, 
            
        }
    }
}

impl<T : Num> Sub<Self> for Mat4<T> {
    type Output = Self;
    fn sub(self, other : Self) -> Self {
        Self {
            x : self.x - other.x, 
            y : self.y - other.y, 
            z : self.z - other.z, 
            w : self.w - other.w, 
            
        }
    }
}

impl<T : Num> Mul<Self> for Mat4<T> {
    type Output = Self;
    fn mul(self, other : Self) -> Self {
        Self {
            x : self.x * other.x, 
            y : self.y * other.y, 
            z : self.z * other.z, 
            w : self.w * other.w, 
            
        }
    }
}

impl<T : Num> Div<Self> for Mat4<T> {
    type Output = Self;
    fn div(self, other : Self) -> Self {
        Self {
            x : self.x / other.x, 
            y : self.y / other.y, 
            z  : self.z / other.z, 
            w : self.w / other.w, 
            
        }
    }
}