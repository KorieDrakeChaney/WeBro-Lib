pub mod vectors;
pub mod matrices;
pub mod traits;

pub mod prelude { 
    pub use crate::matrices::*;
    pub use crate::vectors::*;
    pub use crate::traits::*;
}