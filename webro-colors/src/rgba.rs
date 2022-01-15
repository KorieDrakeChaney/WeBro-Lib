use std::ops::{Add, Sub, Mul, Div};
use crate::color::Color;
use crate::rgb::RGB;

pub struct RGBA { 
    pub r : f32, 
    pub g : f32, 
    pub b : f32,
    pub a : f32,
}

impl Default for RGBA { 
    fn default() -> Self{
        Self { 
            r : 0.0,
            g : 0.0,
            b : 0.0,
            a : 0.0,
        }
    }
}




impl RGBA {
    pub fn new(r : f32, g : f32, b : f32, a : f32) -> Self {
        Self { 
            r, 
            g, 
            b, 
            a
        }
    }
}


impl Add<Self> for RGBA {
    type Output = Self;
    
    fn add(self, object : Self) ->  Self {
        Self { 
            r : self.r + object.r,
            g : self.g + object.g,
            b : self.b + object.b,
            a : self.a + object.a,
        }
    }
}

impl Add<RGB> for RGBA {
    type Output = Self;
    
    fn add(self, object : RGB) ->  Self {
        Self { 
            r : self.r + object.r,
            g : self.g + object.g,
            b : self.b + object.b,
            a : self.a
        }
    }
}

impl Sub<Self> for RGBA {
    type Output = Self;
    
    fn sub(self, object : Self) ->  Self {
        Self { 
            r : self.r - object.r,
            g : self.g - object.g,
            b : self.b - object.b,
            a : self.a - object.a,
        }
    }
}

impl Sub<RGB> for RGBA {
    type Output = Self;
    
    fn sub(self, object : RGB) ->  Self {
        Self { 
            r : self.r - object.r,
            g : self.g - object.g,
            b : self.b - object.b,
            a : self.a
        }
    }
}

impl Mul<Self> for RGBA {
    type Output = Self;
    
    fn mul(self, object : Self) ->  Self {
        Self { 
            r : self.r * object.r,
            g : self.g * object.g,
            b : self.b * object.b,
            a : self.a * object.a,
        }
    }
}

impl Mul<RGB> for RGBA {
    type Output = Self;
    
    fn mul(self, object : RGB) ->  Self {
        Self { 
            r : self.r * object.r,
            g : self.g * object.g,
            b : self.b * object.b,
            a : self.a
        }
    }
}

impl Div<Self> for RGBA {
    type Output = Self;
    
    fn div(self, object : Self) ->  Self {
        Self { 
            r : self.r / object.r,
            g : self.g / object.g,
            b : self.b / object.b,
            a : self.a / object.a,
        }
    }
}

impl Div<RGB> for RGBA {
    type Output = Self;
    
    fn div(self, object : RGB) ->  Self {
        Self { 
            r : self.r / object.r,
            g : self.g / object.g,
            b : self.b / object.b,
            a : self.a
        }
    }
}

impl Color for RGBA {
    fn color(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}   

