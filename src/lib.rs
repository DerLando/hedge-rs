//!
//! An index based half-edge mesh implementation.
//!

extern crate cgmath;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;
extern crate ordermap;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use core::*;
pub use function_sets::*;
pub use iterators::*;
pub use query::*;
pub use operator::*;

pub mod core;
pub mod function_sets;
pub mod iterators;
pub mod utils;
pub mod query;
pub mod operator;

pub struct Mesh {
    kernel: Kernel,
    tag: AtomicUsize,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mesh {{ {} points, {} vertices, {} edges, {} faces }}",
            self.point_count(),
            self.vertex_count(),
            self.edge_count(),
            self.face_count()
        )
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            kernel: Kernel::default(),
            tag: AtomicUsize::new(1),
        }
    }

    fn next_tag(&self) -> usize {
        self.tag.fetch_add(1, Ordering::SeqCst)
    }

    /// Returns a `FaceFn` for the given index.
    pub fn face(&self, index: FaceIndex) -> FaceFn {
        FaceFn::new(index, &self)
    }

    pub fn face_count(&self) -> usize {
        self.kernel.face_count() - 1
    }

    pub fn faces<'mesh>(&'mesh self) -> FaceFnIterator<'mesh> {
        FaceFnIterator::new(&self)
    }

    /// Returns an `EdgeFn` for the given index.
    pub fn edge(&self, index: EdgeIndex) -> EdgeFn {
        EdgeFn::new(index, &self)
    }

    pub fn edge_count(&self) -> usize {
        self.kernel.edge_count() - 1
    }

    pub fn edges<'mesh>(&'mesh self) -> EdgeFnIterator<'mesh> {
        EdgeFnIterator::new(&self)
    }

    /// Returns a `VertexFn` for the given index.
    pub fn vertex(&self, index: VertexIndex) -> VertexFn {
        VertexFn::new(index, &self)
    }

    pub fn vertex_count(&self) -> usize {
        self.kernel.vertex_count() - 1
    }

    pub fn vertices<'mesh>(&'mesh self) -> VertexFnIterator<'mesh> {
        VertexFnIterator::new(&self)
    }

    pub fn point(&self, index: PointIndex) -> &Point {
        self.kernel.get(&index)
    }

    pub fn point_count(&self) -> usize {
        self.kernel.point_count() - 1
    }
}

#[cfg(test)]
mod tests;
