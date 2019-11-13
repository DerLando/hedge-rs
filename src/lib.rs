//!
//! An index based half-edge mesh implementation.
//!

pub use crate::data::*;
pub use crate::elements::*;
pub use crate::handles::*;
pub use crate::iterators::*;
pub use crate::mesh::*;
pub use crate::proxy::*;
pub use crate::traits::*;

pub mod data;
pub mod elements;
pub mod handles;
pub mod kernel;
pub mod mesh;
pub mod proxy;
pub mod traits;
//pub mod utils;
pub mod iterators;
