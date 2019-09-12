//! Facades over a mesh and component index to enable fluent adjcency traversals.

use crate::traits::*;
use crate::elements::*;
use crate::mesh::Mesh;
use crate::iterators::{
    FaceEdges,
    FaceVertices,
    VertexCirculator
};
use std::cell::{Ref, RefMut};

pub trait FunctionSet<'mesh, I: ElementHandle + Default, D: ElementData + Default> {
    fn new(index: I, mesh: &'mesh Mesh) -> Self;
    fn element(&self) -> Option<&'mesh MeshElement<D>>;

    fn maybe(index: Option<I>, mesh: &'mesh Mesh) -> Self
        where Self: std::marker::Sized
    {
        if let Some(index) = index {
            Self::new(index, mesh)
        } else {
            Self::new(Default::default(), mesh)
        }
    }

    fn data(&'mesh self) -> Option<Ref<D>> {
        self.element().map(|e| e.data())
    }

    fn data_mut(&'mesh self) -> Option<RefMut<D>> {
        self.element().map(|e| e.data_mut())
    }
}

/// Function set for operations related to the Face struct
#[derive(Debug, Copy, Clone)]
pub struct FaceFn<'mesh> {
    mesh: &'mesh Mesh,
    pub index: FaceHandle,
}

impl<'mesh> FunctionSet<'mesh, FaceHandle, FaceData> for FaceFn<'mesh> {
    fn new(index: FaceHandle, mesh: &'mesh Mesh) -> Self {
        FaceFn {
            mesh,
            index,
        }
    }

    fn element(&self) -> Option<&'mesh Face> {
        self.mesh.get_element(&self.index)
    }
}

impl<'mesh> FaceFn<'mesh> {
    /// Convert this `FaceFn` to an `EdgeFn`.
    pub fn edge(&self) -> EdgeFn<'mesh> {
        let edge_index = self.data().map(|data| data.edge);
        EdgeFn::maybe(edge_index, self.mesh)
    }

    pub fn edges(&self) -> FaceEdges<'mesh> {
        FaceEdges::new(self.mesh.next_tag(), *self)
    }

    pub fn vertices(&self) -> FaceVertices<'mesh> {
        FaceVertices::new(self.mesh.next_tag(), *self)
    }
}

impl<'mesh> IsValid for FaceFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.element().is_some()
    }
}

/// Function set for operations related to the Edge struct
#[derive(Debug, Copy, Clone)]
pub struct EdgeFn<'mesh> {
    mesh: &'mesh Mesh,
    pub index: HalfEdgeHandle,
}

impl<'mesh> FunctionSet<'mesh, HalfEdgeHandle, HalfEdgeData> for EdgeFn<'mesh> {
    fn new(index: HalfEdgeHandle, mesh: &'mesh Mesh) -> Self {
        EdgeFn {
            mesh,
            index,
        }
    }

    fn element(&self) -> Option<&'mesh HalfEdge> {
        self.mesh.get_element(&self.index)
    }
}

impl<'mesh> EdgeFn<'mesh> {
    pub fn is_boundary(&self) -> bool {
        !self.face().is_valid() || !self.twin().face().is_valid()
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's next edge
    pub fn next(&self) -> EdgeFn<'mesh> {
        let next_index = self.data().map(|data| data.next);
        EdgeFn::maybe(next_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's prev edge
    pub fn prev(&self) -> EdgeFn<'mesh> {
        let prev_index = self.data().map(|data| data.prev);
        EdgeFn::maybe(prev_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's twin edge
    pub fn twin(&self) -> EdgeFn<'mesh> {
        let twin_index = self.data().map(|data| data.adjacent);
        EdgeFn::maybe(twin_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `FaceFn`
    pub fn face(&self) -> FaceFn<'mesh> {
        let face_index = self.data().map(|data| data.face);
        FaceFn::maybe(face_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `VertexFn`
    pub fn vertex(&self) -> VertexFn<'mesh> {
        let vertex_index = self.data().map(|data| data.vertex);
        VertexFn::maybe(vertex_index, self.mesh)
    }
}

impl<'mesh> IsValid for EdgeFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.element().is_some()
    }
}

/// Function set for operations related to the Vertex struct
#[derive(Debug, Copy, Clone)]
pub struct VertexFn<'mesh> {
    mesh: &'mesh Mesh,
    pub index: VertexHandle,
}

impl<'mesh> FunctionSet<'mesh, VertexHandle, VertexData> for VertexFn<'mesh> {
    fn new(index: VertexHandle, mesh: &'mesh Mesh) -> Self {
        VertexFn {
            mesh,
            index,
        }
    }

    fn element(&self) -> Option<&'mesh Vertex> {
        self.mesh.get_element(&self.index)
    }
}

impl<'mesh> VertexFn<'mesh> {
    /// Convert this `VertexFn` to an `EdgeFn`
    pub fn edge(&self) -> EdgeFn<'mesh> {
        let edge_index = self.data().map(|data| data.edge);
        EdgeFn::maybe(edge_index, self.mesh)
    }

    pub fn edges(&self) -> VertexCirculator {
        VertexCirculator::new(self.mesh.next_tag(), *self)
    }

    pub fn point(&self) -> Option<&'mesh Point> {
        self.data().and_then(|data| {
            self.mesh.get_element(&data.point)
        })
    }
}

impl<'mesh> IsValid for VertexFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.element().is_some()
    }
}
