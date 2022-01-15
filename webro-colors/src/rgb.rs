use std::ops::{Add, Sub, Mul, Div};
use crate::color::Color;
use crate::rgba::RGBA;

pub struct RGB { 
    pub r : f32, 
    pub g : f32, 
    pub b : f32,
}

impl Default for RGB { 
    fn default() -> Self{
        Self { 
            r : 0.0,
            g : 0.0,
            b : 0.0,
        }
    }
}

impl RGB {
    pub fn new(r : f32, g : f32, b : f32) -> Self {
        Self { 
            r, 
            g, 
            b
        }
    }
}


impl Add<Self> for RGB {
    type Output = Self;
    
    fn add(self, object : Self) ->  Self {
        Self { 
            r : self.r + object.r,
            g : self.g + object.g,
            b : self.b + object.b
        }
    }
}

impl Add<RGBA> for RGB {
    type Output = Self;
    
    fn add(self, object : RGBA) ->  Self {
        Self { 
            r : self.r + object.r,
            g : self.g + object.g,
            b : self.b + object.b,
        }
    }
}

impl Sub<Self> for RGB {
    type Output = Self;
    
    fn sub(self, object : Self) ->  Self {
        Self { 
            r : self.r - object.r,
            g : self.g - object.g,
            b : self.b - object.b,
        }
    }
}

impl Sub<RGBA> for RGB {
    type Output = Self;
    
    fn sub(self, object : RGBA) ->  Self {
        Self { 
            r : self.r - object.r,
            g : self.g - object.g,
            b : self.b - object.b,
        }
    }
}

impl Mul<Self> for RGB {
    type Output = Self;
    
    fn mul(self, object : Self) ->  Self {
        Self { 
            r : self.r * object.r,
            g : self.g * object.g,
            b : self.b * object.b,
        }
    }
}

impl Mul<RGBA> for RGB {
    type Output = Self;
    
    fn mul(self, object : RGBA) ->  Self {
        Self { 
            r : self.r * object.r,
            g : self.g * object.g,
            b : self.b * object.b,
        }
    }
}

impl Div<Self> for RGB {
    type Output = Self;
    
    fn div(self, object : Self) ->  Self {
        Self { 
            r : self.r / object.r,
            g : self.g / object.g,
            b : self.b / object.b,
        }
    }
}

impl Div<RGBA> for RGB {
    type Output = Self;
    
    fn div(self, object : RGBA) ->  Self {
        Self { 
            r : self.r / object.r,
            g : self.g / object.g,
            b : self.b / object.b,
        }
    }
}

impl Color for RGB {
    fn color(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b]
    }
}   

