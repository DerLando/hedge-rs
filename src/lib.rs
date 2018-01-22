
//!
//! An index based half-edge mesh implementation.
//!

#[macro_use]
extern crate log;

extern crate cgmath;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use core::*;
pub use function_sets::*;
pub use iterators::*;

pub mod core;
pub mod function_sets;
pub mod iterators;


/// Storage interface for Mesh types
#[derive(Debug, Default)]
pub struct Kernel {
    edge_buffer: ElementBuffer<Edge>,
    face_buffer: ElementBuffer<Face>,
    vertex_buffer: ElementBuffer<Vertex>,
    point_buffer: ElementBuffer<Point>,
}

impl Kernel {

    /// Sorts contents of each buffer moving inactive elements to the back.
    #[allow(dead_code)]
    fn defrag(&mut self) {
        unimplemented!()
    }

    /// Drops all inactive elements and shrinks buffers.
    #[allow(dead_code)]
    fn collect(&mut self) {
        unimplemented!()
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////


pub struct Mesh {
    kernel: Kernel,
    tag: AtomicUsize,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Half-Edge Mesh {{ {} points, {} vertices, {} edges, {} faces }}",
               self.point_count(), self.vertex_count(),
               self.edge_count(), self.face_count())
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
        self.kernel.face_buffer.len() - 1
    }

    pub fn faces<'mesh>(&'mesh self) -> FaceFnIterator<'mesh> {
        FaceFnIterator::new(&self)
    }

    /// Returns an `EdgeFn` for the given index.
    pub fn edge(&self, index: EdgeIndex) -> EdgeFn {
        EdgeFn::new(index, &self)
    }

    pub fn edge_count(&self) -> usize {
        self.kernel.edge_buffer.len() - 1
    }

    pub fn edges<'mesh>(&'mesh self) -> EdgeFnIterator<'mesh> {
        EdgeFnIterator::new(&self)
    }

    /// Returns a `VertexFn` for the given index.
    pub fn vertex(&self, index: VertexIndex) -> VertexFn {
        VertexFn::new(index, &self)
    }

    pub fn vertex_count(&self) -> usize {
        self.kernel.vertex_buffer.len() - 1
    }

    pub fn vertices<'mesh>(&'mesh self) -> VertexFnIterator<'mesh> {
        VertexFnIterator::new(&self)
    }

    pub fn point(&self, index: PointIndex) -> &Point {
        self.kernel.point_buffer.get(&index)
    }

    pub fn point_count(&self) -> usize {
        self.kernel.point_buffer.len() - 1
    }


}

////////////////////////////////////////////////////////////////////////////////
// Adding elements

impl AddElement<Vertex> for Mesh {
    fn add(&mut self, vertex: Vertex) -> VertexIndex {
        self.kernel.vertex_buffer.add(vertex)
    }
}

impl AddElement<Edge> for Mesh {
    fn add(&mut self, edge: Edge) -> EdgeIndex {
        self.kernel.edge_buffer.add(edge)
    }
}

impl AddElement<Face> for Mesh {
    fn add(&mut self, face: Face) -> FaceIndex {
        self.kernel.face_buffer.add(face)
    }
}

impl AddElement<Point> for Mesh {
    fn add(&mut self, point: Point) -> PointIndex {
        self.kernel.point_buffer.add(point)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Removing elements

impl RemoveElement<Vertex> for Mesh {
    fn remove(&mut self, index: VertexIndex) {
        self.kernel.vertex_buffer.remove(index);
    }
}

impl RemoveElement<Edge> for Mesh {
    fn remove(&mut self, index: EdgeIndex) {
        self.kernel.edge_buffer.remove(index);
    }
}

impl RemoveElement<Face> for Mesh {
    fn remove(&mut self, index: FaceIndex) {
        self.kernel.face_buffer.remove(index);
    }
}

impl RemoveElement<Point> for Mesh {
    fn remove(&mut self, index: PointIndex) {
        self.kernel.point_buffer.remove(index);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Get immutable references

impl GetElement<Vertex> for Mesh {
    fn get(&self, index: &VertexIndex) -> &Vertex {
        self.kernel.vertex_buffer.get(index)
    }
}

impl GetElement<Edge> for Mesh {
    fn get(&self, index: &EdgeIndex) -> &Edge {
        self.kernel.edge_buffer.get(index)
    }
}

impl GetElement<Face> for Mesh {
    fn get(&self, index: &FaceIndex) -> &Face {
        self.kernel.face_buffer.get(index)
    }
}

impl GetElement<Point> for Mesh {
    fn get(&self, index: &PointIndex) -> &Point {
        self.kernel.point_buffer.get(index)
    }
}

#[cfg(test)]
mod tests;
