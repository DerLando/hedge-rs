//! Facades over a mesh and component index to enable fluent adjcency traversals.

use super::*;

use cgmath::prelude::*;


/// Function set for operations related to the Face struct
#[derive(Debug, Copy, Clone)]
pub struct FaceFn<'mesh> {
    mesh: &'mesh Mesh,
    pub element: &'mesh Face,
    pub index: FaceIndex
}

impl<'mesh> FaceFn<'mesh> {

    pub fn new(index: FaceIndex, mesh: &'mesh Mesh) -> FaceFn {
        FaceFn {
            mesh,
            element: &mesh.get(&index),
            index
        }
    }

    pub fn from_index_and_item(index: FaceIndex, element: &'mesh Face, mesh: &'mesh Mesh) -> FaceFn<'mesh> {
        FaceFn {
            mesh, element, index
        }
    }

    fn calc_area(&self, p0: &Point, p1: &Point, p2: &Point) -> f64 {
        let a = p1.position - p0.position;
        let b = p2.position - p0.position;
        a.cross(b).magnitude() / 2.0
    }

    pub fn area(&self) -> f64 {
        let mut area: f64 = 0.0;
        let v0 = self.edge().vertex();
        let mut v1 = v0.edge().next().vertex();
        let mut v2 = v1.edge().next().vertex();
        while v0.index != v2.index {
            let p0 = self.mesh.vertex(v0.index).point();
            let p1 = self.mesh.vertex(v1.index).point();
            let p2 = self.mesh.vertex(v2.index).point();
            area += self.calc_area(p0, p1, p2);
            v1 = v2;
            v2 = v2.edge().next().vertex();
        }
        return area;
    }

    /// Convert this `FaceFn` to an `EdgeFn`.
    pub fn edge(&self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.element.edge_index.get(), self.mesh)
    }

    pub fn edges(&self) -> FaceEdges {
        FaceEdges::new(self.mesh.next_tag(), self.edge())
    }

    pub fn vertices(&self) -> FaceVertices {
        FaceVertices::new(self.edges())
    }
}

impl<'mesh> IsValid for FaceFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.element.is_valid()
    }
}


/// Function set for operations related to the Edge struct
#[derive(Debug, Copy, Clone)]
pub struct EdgeFn<'mesh> {
    mesh: &'mesh Mesh,
    pub element: &'mesh Edge,
    pub index: EdgeIndex
}

impl<'mesh> EdgeFn<'mesh> {
    pub fn new(index: EdgeIndex, mesh: &'mesh Mesh) -> EdgeFn {
        EdgeFn {
            mesh,
            element: &mesh.get(&index),
            index
        }
    }

    pub fn from_index_and_item(index: EdgeIndex, element: &'mesh Edge, mesh: &'mesh Mesh) -> EdgeFn<'mesh> {
        EdgeFn {
            mesh, element, index
        }
    }

    pub fn is_boundary(&self) -> bool {
        !self.face().is_valid() || !self.twin().face().is_valid()
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's next edge
    pub fn next(&self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.element.next_index.get(), self.mesh)
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's prev edge
    pub fn prev(&self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.element.prev_index.get(), self.mesh)
    }

    /// Convert this `EdgeFn` to an `EdgeFn` of it's twin edge
    pub fn twin(&self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.element.twin_index.get(), self.mesh)
    }

    /// Convert this `EdgeFn` to an `FaceFn`
    pub fn face(&self) -> FaceFn<'mesh> {
        FaceFn::new(self.element.face_index.get(), self.mesh)
    }

    /// Convert this `EdgeFn` to an `VertexFn`
    pub fn vertex(&self) -> VertexFn<'mesh> {
        VertexFn::new(self.element.vertex_index.get(), self.mesh)
    }
}

impl<'mesh> IsValid for EdgeFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.element.is_valid()
    }
}


/// Function set for operations related to the Vertex struct
#[derive(Debug, Copy, Clone)]
pub struct VertexFn<'mesh> {
    mesh: &'mesh Mesh,
    pub element: &'mesh Vertex,
    pub index: VertexIndex
}

impl<'mesh> VertexFn<'mesh> {

    pub fn new(index: VertexIndex, mesh: &'mesh Mesh) -> VertexFn {
        VertexFn {
            mesh,
            element: &mesh.get(&index),
            index
        }
    }

    pub fn from_index_and_item(index: VertexIndex, element: &'mesh Vertex, mesh: &'mesh Mesh) -> VertexFn<'mesh> {
        VertexFn {
            mesh, element, index
        }
    }

    pub fn calc_normal(&self) -> Normal {
        let e = self.edge();

        let p0 = self.point();
        let p1 = e.next().vertex().point();
        let p2 = e.prev().vertex().point();

        let a = p0.position - p1.position;
        let b = p0.position - p2.position;

        a.cross(b).normalize()
    }

    /// Convert this `VertexFn` to an `EdgeFn`
    pub fn edge(&self) -> EdgeFn<'mesh> {
        EdgeFn::new(self.element.edge_index.get(), self.mesh)
    }

    pub fn point(&self) -> &'mesh Point {
        self.mesh.get(&self.element.point_index.get())
    }
}

impl<'mesh> IsValid for VertexFn<'mesh> {
    fn is_valid(&self) -> bool {
        self.element.is_valid()
    }
}
