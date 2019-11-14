//! Facades over a mesh and element handle to enable easy topology traversals.

use crate::elements::{Face, HalfEdge, Point, Vertex};
use crate::handles::{FaceHandle, HalfEdgeHandle, VertexHandle};
use crate::iterators::{FaceEdges, FaceVertices, VertexCirculator};
use crate::mesh::Mesh;
use crate::traits::*;
use std::cell::{Ref, RefMut};

pub trait ElementProxy<'mesh, E: Element + 'mesh> {
    fn new(handle: E::Handle, mesh: &'mesh Mesh) -> Self;
    fn element(&self) -> Option<&'mesh E>;

    fn maybe(handle: Option<E::Handle>, mesh: &'mesh Mesh) -> Self
    where
        Self: std::marker::Sized,
    {
        if let Some(handle) = handle {
            Self::new(handle, mesh)
        } else {
            Self::new(Default::default(), mesh)
        }
    }

    fn data(&'mesh self) -> Option<Ref<E::Data>> {
        self.element().map(|e| e.data())
    }

    fn data_mut(&'mesh self) -> Option<RefMut<E::Data>> {
        self.element().map(|e| e.data_mut())
    }
}

/// Function set for operations related to the Face struct
#[derive(Debug, Copy, Clone)]
pub struct FaceProxy<'mesh> {
    mesh: &'mesh Mesh,
    pub handle: FaceHandle,
}

impl<'mesh> ElementProxy<'mesh, Face> for FaceProxy<'mesh> {
    fn new(handle: FaceHandle, mesh: &'mesh Mesh) -> Self {
        FaceProxy { mesh, handle }
    }

    fn element(&self) -> Option<&'mesh Face> {
        self.mesh.get(self.handle)
    }
}

impl<'mesh> FaceProxy<'mesh> {
    pub fn root_edge(&self) -> HalfEdgeProxy<'mesh> {
        let edge_handle = self.data().map(|data| data.root_edge);
        HalfEdgeProxy::maybe(edge_handle, self.mesh)
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
    pub handle: HalfEdgeHandle,
}

impl<'mesh> ElementProxy<'mesh, HalfEdge> for HalfEdgeProxy<'mesh> {
    fn new(handle: HalfEdgeHandle, mesh: &'mesh Mesh) -> Self {
        HalfEdgeProxy { mesh, handle }
    }

    fn element(&self) -> Option<&'mesh HalfEdge> {
        self.mesh.get(self.handle)
    }
}

impl<'mesh> HalfEdgeProxy<'mesh> {
    pub fn is_boundary(&self) -> bool {
        !self.face().is_valid() || !self.adjacent().face().is_valid()
    }

    pub fn next(&self) -> HalfEdgeProxy<'mesh> {
        let next_handle = self.data().map(|data| data.next);
        HalfEdgeProxy::maybe(next_handle, self.mesh)
    }

    pub fn prev(&self) -> HalfEdgeProxy<'mesh> {
        let prev_handle = self.data().map(|data| data.prev);
        HalfEdgeProxy::maybe(prev_handle, self.mesh)
    }

    pub fn adjacent(&self) -> HalfEdgeProxy<'mesh> {
        let adjacent_handle = self.data().map(|data| data.adjacent);
        HalfEdgeProxy::maybe(adjacent_handle, self.mesh)
    }

    pub fn face(&self) -> FaceProxy<'mesh> {
        let face_handle = self.data().map(|data| data.face);
        FaceProxy::maybe(face_handle, self.mesh)
    }

    pub fn vertex(&self) -> VertexProxy<'mesh> {
        let vertex_handle = self.data().map(|data| data.vertex);
        VertexProxy::maybe(vertex_handle, self.mesh)
    }

    pub fn connect_to(&self, next: &HalfEdgeProxy) {
        log::trace!(
            "--- Connecting Edges {} -> {}",
            self.handle.index(), next.handle.index()
        );
        match (self.element(), next.element()) {
            (Some(p), Some(n)) => {
                p.data_mut().next = next.handle;
                n.data_mut().prev = self.handle;
            }
            (None, Some(_)) => {
                log::error!("Unable to connect edges: source proxy was invalid.");
            }
            (Some(_), None) => {
                log::error!("Unable to connect edges: target proxy was invalid.");
            }
            (None, None) => {
                log::error!("Unable to connect edges: both proxies were invalid.");
            }
        }
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
    pub handle: VertexHandle,
}

impl<'mesh> ElementProxy<'mesh, Vertex> for VertexProxy<'mesh> {
    fn new(handle: VertexHandle, mesh: &'mesh Mesh) -> Self {
        VertexProxy { mesh, handle }
    }

    fn element(&self) -> Option<&'mesh Vertex> {
        self.mesh.get(self.handle)
    }
}

impl<'mesh> VertexProxy<'mesh> {
    pub fn edge(&self) -> HalfEdgeProxy<'mesh> {
        let edge_handle = self.data().map(|data| data.edge);
        HalfEdgeProxy::maybe(edge_handle, self.mesh)
    }

    pub fn edges(&self) -> VertexCirculator {
        VertexCirculator::new(self.mesh.next_tag(), *self)
    }

    pub fn point(&self) -> Option<&'mesh Point> {
        self.data().and_then(|data| self.mesh.get(data.point))
    }
}

impl<'mesh> IsValid for VertexProxy<'mesh> {
    fn is_valid(&self) -> bool {
        self.element().is_some()
    }
}
