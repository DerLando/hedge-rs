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

pub trait ElementProxy<'mesh, I: ElementHandle + Default, D: ElementData + Default> {
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
pub struct FaceProxy<'mesh> {
    mesh: &'mesh Mesh,
    pub index: FaceHandle,
}

impl<'mesh> ElementProxy<'mesh, FaceHandle, FaceData> for FaceProxy<'mesh> {
    fn new(index: FaceHandle, mesh: &'mesh Mesh) -> Self {
        FaceProxy {
            mesh,
            index,
        }
    }

    fn element(&self) -> Option<&'mesh Face> {
        self.mesh.get_element(&self.index)
    }
}

impl<'mesh> FaceProxy<'mesh> {
    pub fn root_edge(&self) -> HalfEdgeProxy<'mesh> {
        let edge_index = self.data().map(|data| data.root_edge);
        HalfEdgeProxy::maybe(edge_index, self.mesh)
    }

    pub fn edges(&self) -> FaceEdges<'mesh> {
        FaceEdges::new(self.mesh.next_tag(), *self)
    }

    pub fn vertices(&self) -> FaceVertices<'mesh> {
        FaceVertices::new(self.mesh.next_tag(), *self)
    }
}

impl<'mesh> IsValid for FaceProxy<'mesh> {
    fn is_valid(&self) -> bool {
        self.element().is_some()
    }
}

/// Function set for operations related to the Edge struct
#[derive(Debug, Copy, Clone)]
pub struct HalfEdgeProxy<'mesh> {
    mesh: &'mesh Mesh,
    pub index: HalfEdgeHandle,
}

impl<'mesh> ElementProxy<'mesh, HalfEdgeHandle, HalfEdgeData> for HalfEdgeProxy<'mesh> {
    fn new(index: HalfEdgeHandle, mesh: &'mesh Mesh) -> Self {
        HalfEdgeProxy {
            mesh,
            index,
        }
    }

    fn element(&self) -> Option<&'mesh HalfEdge> {
        self.mesh.get_element(&self.index)
    }
}

impl<'mesh> HalfEdgeProxy<'mesh> {
    pub fn is_boundary(&self) -> bool {
        !self.face().is_valid() || !self.twin().face().is_valid()
    }

    pub fn next(&self) -> HalfEdgeProxy<'mesh> {
        let next_index = self.data().map(|data| data.next);
        HalfEdgeProxy::maybe(next_index, self.mesh)
    }

    pub fn prev(&self) -> HalfEdgeProxy<'mesh> {
        let prev_index = self.data().map(|data| data.prev);
        HalfEdgeProxy::maybe(prev_index, self.mesh)
    }

    pub fn twin(&self) -> HalfEdgeProxy<'mesh> {
        let twin_index = self.data().map(|data| data.adjacent);
        HalfEdgeProxy::maybe(twin_index, self.mesh)
    }

    pub fn face(&self) -> FaceProxy<'mesh> {
        let face_index = self.data().map(|data| data.face);
        FaceProxy::maybe(face_index, self.mesh)
    }

    pub fn vertex(&self) -> VertexProxy<'mesh> {
        let vertex_index = self.data().map(|data| data.vertex);
        VertexProxy::maybe(vertex_index, self.mesh)
    }
}

impl<'mesh> IsValid for HalfEdgeProxy<'mesh> {
    fn is_valid(&self) -> bool {
        self.element().is_some()
    }
}

/// Function set for operations related to the Vertex struct
#[derive(Debug, Copy, Clone)]
pub struct VertexProxy<'mesh> {
    mesh: &'mesh Mesh,
    pub index: VertexHandle,
}

impl<'mesh> ElementProxy<'mesh, VertexHandle, VertexData> for VertexProxy<'mesh> {
    fn new(index: VertexHandle, mesh: &'mesh Mesh) -> Self {
        VertexProxy {
            mesh,
            index,
        }
    }

    fn element(&self) -> Option<&'mesh Vertex> {
        self.mesh.get_element(&self.index)
    }
}

impl<'mesh> VertexProxy<'mesh> {
    pub fn edge(&self) -> HalfEdgeProxy<'mesh> {
        let edge_index = self.data().map(|data| data.edge);
        HalfEdgeProxy::maybe(edge_index, self.mesh)
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

impl<'mesh> IsValid for VertexProxy<'mesh> {
    fn is_valid(&self) -> bool {
        self.element().is_some()
    }
}
