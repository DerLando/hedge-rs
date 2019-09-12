
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use std::cmp;

use super::IsValid;

/// Marker trait for handle types
pub trait ElementHandle {}

pub type Offset = u32;
pub type Generation = u32;

/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_OFFSET: Offset = std::u32::MAX;

/// Type-safe index into kernel storage.
#[derive(Debug, Clone, Eq)]
pub struct Handle<T> {
    pub offset: Offset,
    pub generation: Generation,
    _marker: PhantomData<T>,
}

impl<T: Clone> Copy for Handle<T> {}

impl<T> Default for Handle<T> {
    fn default() -> Self {
        Handle {
            offset: INVALID_COMPONENT_OFFSET,
            generation: 0,
            _marker: Default::default(),
        }
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
        self.generation.hash(state);
    }
}

impl<T> Handle<T> {
    pub fn new(offset: Offset) -> Handle<T> {
        Handle {
            offset,
            generation: 0,
            _marker: PhantomData::default(),
        }
    }

    pub fn with_generation(offset: Offset, generation: Generation) -> Handle<T> {
        Handle {
            offset,
            generation,
            _marker: PhantomData::default(),
        }
    }
}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Handle<T>) -> Option<cmp::Ordering> {
        // Only the offset should matter when it comes to ordering
        self.offset.partial_cmp(&other.offset)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Handle<T>) -> bool {
        self.offset.eq(&other.offset) && self.generation.eq(&other.generation)
    }
}

impl<T> IsValid for Handle<T> {
    fn is_valid(&self) -> bool {
        self.offset != INVALID_COMPONENT_OFFSET
    }
}
