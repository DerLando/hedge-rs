
//!
//! An index based half-edge mesh implementation.
//!

#[macro_use]
extern crate log;

extern crate cgmath;

use std::fmt;

pub use core::*;
pub use function_sets::*;

pub mod core;
pub mod function_sets;


/// Storage interface for Mesh types
#[derive(Debug, Default)]
pub struct Kernel {
    edge_buffer: ElementBuffer<Edge>,
    face_buffer: ElementBuffer<Face>,
    vertex_buffer: ElementBuffer<Vertex>,
    point_buffer: ElementBuffer<Point>,
}

impl Kernel {
    pub fn edge_count(&self) -> usize {
        self.edge_buffer.len()
    }

    pub fn face_count(&self) -> usize {
        self.face_buffer.len()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_buffer.len()
    }

    pub fn point_count(&self) -> usize {
        self.point_buffer.len()
    }

    pub fn get_edge(&self, index: &EdgeIndex) -> &Edge {
        self.edge_buffer.get(index)
    }

    pub fn get_face(&self, index: &FaceIndex) -> &Face {
        self.face_buffer.get(index)
    }

    pub fn get_vertex(&self, index: &VertexIndex) -> &Vertex {
        self.vertex_buffer.get(index)
    }

    pub fn get_point(&self, index: &PointIndex) -> &Point {
        self.point_buffer.get(index)
    }

    pub fn get_edge_mut(&mut self, index: &EdgeIndex) -> Option<&mut Edge> {
        self.edge_buffer.get_mut(index)
    }

    pub fn get_face_mut(&mut self, index: &FaceIndex) -> Option<&mut Face> {
        self.face_buffer.get_mut(index)
    }

    pub fn get_vertex_mut(&mut self, index: &VertexIndex) -> Option<&mut Vertex> {
        self.vertex_buffer.get_mut(index)
    }

    pub fn get_point_mut(&mut self, index: &PointIndex) -> Option<&mut Point> {
        self.point_buffer.get_mut(index)
    }

    pub fn add_edge(&mut self, edge: Edge) -> EdgeIndex {
        self.edge_buffer.add(edge)
    }

    pub fn add_face(&mut self, face: Face) -> FaceIndex {
        self.face_buffer.add(face)
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> VertexIndex {
        self.vertex_buffer.add(vertex)
    }

    pub fn add_point(&mut self, point: Point) -> PointIndex {
        self.point_buffer.add(point)
    }

    pub fn remove_edge(&mut self, index: EdgeIndex) {
        self.edge_buffer.remove(index);
    }

    pub fn remove_face(&mut self, index: FaceIndex) {
        self.face_buffer.remove(index);
    }

    pub fn remove_vertex(&mut self, index: VertexIndex) {
        self.vertex_buffer.remove(index);
    }

    pub fn remove_point(&mut self, index: PointIndex) {
        self.point_buffer.remove(index);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////


pub struct Mesh {
    pub kernel: Kernel
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Half-Edge Mesh {{ {} points, {} vertices, {} edges, {} faces }}",
               self.kernel.point_count(), self.kernel.vertex_count(),
               self.kernel.edge_count(), self.kernel.face_count())
    }
}

impl Mesh {
    /// Creates a new Mesh with an initial component added to each Vec.
    ///
    /// The idea behind having a single invalid component at the front of each
    /// Vec comes from the blog http://ourmachinery.com/post/defaulting-to-zero/
    pub fn new() -> Mesh {
        Mesh {
            kernel: Kernel::default()
        }
    }

    /// Returns a `FaceFn` for the given index.
    pub fn face(&self, index: FaceIndex) -> FaceFn {
        FaceFn::new(index, &self)
    }

    /// Returns an `EdgeFn` for the given index.
    pub fn edge(&self, index: EdgeIndex) -> EdgeFn {
        EdgeFn::new(index, &self)
    }

    /// Returns a `VertexFn` for the given index.
    pub fn vertex(&self, index: VertexIndex) -> VertexFn {
        VertexFn::new(index, &self)
    }
}


#[cfg(test)]
mod tests;
