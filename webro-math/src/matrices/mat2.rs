use crate::vectors::Vector4;
use crate::traits::{Identity, IdentityTransorm};
use std::ops::{Add, Sub, Mul, Div};
use num::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat2<T : Num> {
    pub x : Vector4<T>,
    pub y : Vector4<T>,
} 

impl Default for Mat2<f32> {
    fn default() -> Self {
        Self {
            x : Vector4::default(),
            y : Vector4::default(),
            
        }
    }
}

impl Default for Mat2<i32> {
    fn default() -> Self {
        Self {
            x : Vector4::default(),
            y : Vector4::default(),
            
        }
    }
}

impl Identity for Mat2<f32> {
    fn identity() -> Self {
        Self {
            x : Vector4::new(1.0, 0.0, 0.0, 0.0), 
            y : Vector4::new(0.0, 1.0, 0.0, 0.0), 
            
        }
    }
}

impl Identity for Mat2<i32> {
    fn identity() -> Self {
        Self {
            x : Vector4::new(1, 0, 0, 0), 
            y : Vector4::new(0, 1, 0, 0), 
            
        }
    }
}

impl IdentityTransorm for Mat2<f32> {
    fn identity_transform(&mut self){
        self.x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        self.y = Vector4::new(0.0, 1.0, 0.0, 0.0); 
    }
}



impl IdentityTransorm for Mat2<i32> {
    fn identity_transform(&mut self){
        self.x = Vector4::new(1, 0, 0, 0);
        self.y = Vector4::new(0, 1, 0, 0); 
    }
}


impl<T : Num> Mat2<T> {
    pub fn new(x : Vector4<T>, y : Vector4<T>) -> Self{
        Self {
            x, 
            y, 
            
        }
    }

}

impl<T : Num> Add<Self> for Mat2<T> {
    type Output = Self;
    fn add(self, other : Self) -> Self {
        Self {
            x : self.x + other.x, 
            y : self.y + other.y, 
            
        }
    }
}

impl<T : Num> Sub<Self> for Mat2<T> {
    type Output = Self;
    fn sub(self, other : Self) -> Self {
        Self {
            x : self.x - other.x, 
            y : self.y - other.y, 
            
        }
    }
}

impl<T : Num> Mul<Self> for Mat2<T> {
    type Output = Self;
    fn mul(self, other : Self) -> Self {
        Self {
            x : self.x * other.x, 
            y : self.y * other.y, 
            
        }
    }
}

impl<T : Num> Div<Self> for Mat2<T> {
    type Output = Self;
    fn div(self, other : Self) -> Self {
        Self {
            x : self.x / other.x, 
            y : self.y / other.y, 
            
        }
    }
}