//! Facades over a mesh and component index to enable fluent adjcency traversals.

use super::{
    Edge, Face, Vertex, Mesh,
    EdgeIndex, FaceIndex, VertexIndex,
    IsValid
};


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

    pub fn area(self) -> f32 {
        let area: f32 = 0.0;
//        let v0 = self.edge().vertex();
//        let mut v1 = v0.edge().next().vertex();
//        let mut v2 = v1.edge().next().vertex();
//        while v0.index != v2.index {
//            let p0 = self.mesh.vertex(v0.index).point;
//            let p1 = self.mesh.vertex(v1.index).point;
//            let p2 = self.mesh.vertex(v2.index).point;
//            // TODO: area += calc_area(p0, p1, p2);
//            v1 = v2;
//            v2 = v2.edge().next().vertex();
//        }
        return area;
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
