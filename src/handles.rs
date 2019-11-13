use std::cmp;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::data::{Generation, Index};
use crate::elements::{Face, HalfEdge, Point, Vertex};
use crate::traits::{Element, ElementHandle, IsValid};

/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_INDEX: Index = std::u32::MAX;

/// Handles with this generation value will only have their index considered.
pub const IGNORED_GENERATION: Generation = 0;

/// Type-safe index into kernel storage.
#[derive(Debug, Clone, Eq)]
pub struct Handle<T> {
    index: Index,
    generation: Generation,
    _marker: PhantomData<T>,
}

impl<T: Clone> Copy for Handle<T> {}

impl<T> Default for Handle<T> {
    fn default() -> Self {
        Handle {
            index: INVALID_COMPONENT_INDEX,
            generation: IGNORED_GENERATION,
            _marker: Default::default(),
        }
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        //self.generation.hash(state);
    }
}

impl<T: Element> ElementHandle for Handle<T> {
    type Element = T;

    fn new(index: Index) -> Self {
        Handle {
            index,
            generation: 0,
            _marker: PhantomData::default(),
        }
    }

    fn with_generation(index: Index, generation: Generation) -> Self {
        Handle {
            index,
            generation,
            _marker: PhantomData::default(),
        }
    }

    fn index(&self) -> u32 {
        self.index
    }

    fn generation(&self) -> u32 {
        self.generation
    }
}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Handle<T>) -> Option<cmp::Ordering> {
        // Only the index should matter when it comes to ordering
        self.index.partial_cmp(&other.index)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Handle<T>) -> bool {
        if self.generation == IGNORED_GENERATION {
            self.index.eq(&other.index)
        } else {
            self.index.eq(&other.index) && self.generation.eq(&other.generation)
        }
    }
}

impl<T> IsValid for Handle<T> {
    fn is_valid(&self) -> bool {
        self.index != INVALID_COMPONENT_INDEX
    }
}

pub type HalfEdgeHandle = Handle<HalfEdge>;
pub type FaceHandle = Handle<Face>;
pub type VertexHandle = Handle<Vertex>;
pub type PointHandle = Handle<Point>;
