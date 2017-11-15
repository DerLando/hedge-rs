//! Facades over a mesh and component index to enable fluent adjcency traversals.

use super::{
    Edge, Face, Vertex, Mesh,
    EdgeIndex, FaceIndex, VertexIndex,
    IsValid
};


// pub trait Walk<Index, Step> {
//     fn walk_from(&self, index: Index) -> Step;
// }


/// Function set for operations related to the Face struct
#[derive(Debug, Copy, Clone)]
pub struct FaceFn<'mesh> {
    mesh: &'mesh Mesh,
    face: &'mesh Face,
    pub index: FaceIndex
}

impl<'mesh> FaceFn<'mesh> {

    pub fn new(index: FaceIndex, mesh: &'mesh Mesh) -> FaceFn {
        FaceFn {
            mesh,
            face: &mesh.face(index),
            index
        }
    }

    /// Convert this `FaceFn` to an `EdgeFn`.
    pub fn edge(self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.face.edge_index, self.mesh)
    }
}

impl<'mesh> IsValid for FaceFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.face.is_valid()
    }
}

// impl<'mesh> Walk<FaceIndex, FaceFn<'mesh>> for Mesh {
//     fn walk_from(&self, index: FaceIndex) -> FaceFn<'mesh> {
//         FaceFn<'mesh>::new(index, &self)
//     }
// }


/// Function set for operations related to the Edge struct
#[derive(Debug, Copy, Clone)]
pub struct EdgeFn<'mesh> {
    mesh: &'mesh Mesh,
    edge: &'mesh Edge,
    pub index: EdgeIndex
}

impl<'mesh> EdgeFn<'mesh> {
    pub fn new(index: EdgeIndex, mesh: &'mesh Mesh) -> EdgeFn {
        EdgeFn {
            mesh,
            edge: &mesh.edge(index),
            index
        }
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's next edge
    pub fn next(self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.edge.next_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's prev edge
    pub fn prev(self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.edge.prev_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's twin edge
    pub fn twin(self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.edge.twin_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `FaceFn`
    pub fn face(self) -> FaceFn<'mesh> {
        FaceFn::new(self.edge.face_index, self.mesh)
    }

    /// Convert this `EdgeFn` to an `VertexFn`
    pub fn vertex(self) -> VertexFn<'mesh> {
        VertexFn::new(self.edge.vertex_index, self.mesh)
    }
}

impl<'mesh> IsValid for EdgeFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.edge.is_valid()
    }
}


/// Function set for operations related to the Vertex struct
#[derive(Debug, Copy, Clone)]
pub struct VertexFn<'mesh> {
    mesh: &'mesh Mesh,
    vertex: &'mesh Vertex,
    pub index: VertexIndex
}

impl<'mesh> VertexFn<'mesh> {

    pub fn new(index: VertexIndex, mesh: &'mesh Mesh) -> VertexFn {
        VertexFn {
            mesh,
            vertex: &mesh.vertex(index),
            index
        }
    }

    /// Convert this `VertexFn` to an `EdgeFn`
    pub fn edge(self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.vertex.edge_index, self.mesh)
    }
}

impl<'mesh> IsValid for VertexFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.vertex.is_valid()
    }
}
