//!
//! Types, Traits, and impl for various operations on `Mesh` objects.
//!

use super::iterators::*;
use super::{Vertex, VertexIndex, Edge, EdgeIndex,
            Face, FaceIndex, Mesh, IsValid, ElementIndex};

pub mod edge {
    //! Operation parameters for creating and modifying edges.
    //!
    //! - `mesh.add(edge::FromVerts(vert0, vert1));`
    //! - `mesh.add(edge::ExtendLoop(edge, vert));`
    //! - `mesh.add(edge::CloseLoop(prev, next));`
    use super::{VertexIndex, EdgeIndex};

    /// Create a new edge (two half-edges) between the vertices specified.
    pub struct FromVerts(pub VertexIndex, pub VertexIndex);
    /// Create a new edge extending from the specified edge to the specified vertex.
    pub struct ExtendLoop(pub EdgeIndex, pub VertexIndex);
    /// Create a new edge between the end and begining of the specified edges.
    pub struct CloseLoop(pub EdgeIndex, pub EdgeIndex);
}


pub mod face {
    //! Operation parameters for creating faces.
    //!
    //! - `mesh.add(face::FromEdgeLoop(e));`
    use super::EdgeIndex;

    /// Create a new face and associate it with the loop starting at the specified edge.
    pub struct FromEdgeLoop(pub EdgeIndex);
}

pub mod triangle {
    //! Operation parameters for creating triangles.
    //!
    //! - `mesh.add(triangle::FromVerts(vert0, vert1, vert2));`
    //! - `mesh.add(triangle::FromOneEdge(edge, vert));`
    //! - `mesh.add(triangle::FromTwoEdges(prev, next));`
    //! - `mesh.add(triangle::FromThreeEdges(edge0, edge1, edge2));`
    use super::{VertexIndex, EdgeIndex};

    /// Create a new triangle from the 3 vertices specified.
    pub struct FromVerts(pub VertexIndex, pub VertexIndex, pub VertexIndex);
    /// Create a new triangle using the specified edge as the root.
    pub struct FromOneEdge(pub EdgeIndex, pub VertexIndex);
    /// Create a new triangle adjacent to the two edges specified.
    pub struct FromTwoEdges(pub EdgeIndex, pub EdgeIndex, pub EdgeOrder);
    /// Create a new triangle by connecting each of the specified edges in succession.
    pub struct FromThreeEdges(pub EdgeIndex, pub EdgeIndex, pub EdgeIndex);

    /// When creating a triangle from only two edges this explains how to
    /// iterpret the provided edges.
    #[derive(Debug, Clone, Copy)]
    pub enum EdgeOrder {
        AB,
        AC,
        BC,
    }
}

// pub mod quad {
//     use super::{VertexIndex, EdgeIndex};

//     pub struct FromVerts(VertexIndex, VertexIndex, VertexIndex, VertexIndex);
//     pub struct FromAdjacent(EdgeIndex, VertexIndex, VertexIndex);
//     pub struct BridgeEdges(EdgeIndex, EdgeIndex);
//     pub struct StitchCorner(EdgeIndex, EdgeIndex, VertexIndex);
// }

////////////////////////////////////////////////////////////////////////////////

/// Interface for adding elements to a `Mesh`.
pub trait AddGeometry<Descriptor, I: ElementIndex> {
    fn add(&mut self, descriptor: Descriptor) -> I;
}

/// No questions asked, insert this `Vertex` into the `Mesh` storage.
impl AddGeometry<Vertex, VertexIndex> for Mesh {
    fn add(&mut self, vertex: Vertex) -> VertexIndex {
        let result = VertexIndex::new(self.vertex_list.len());
        self.vertex_list.push(vertex);
        return result;
    }
}

/// No questions asked, insert this `Edge` into the `Mesh` storage.
impl AddGeometry<Edge, EdgeIndex> for Mesh {
    fn add(&mut self, edge: Edge) -> EdgeIndex {
        let result = EdgeIndex::new(self.edge_list.len());
        self.edge_list.push(edge);
        return result;
    }
}

/// No questions asked, insert this `Face` into the `Mesh` storage.
impl AddGeometry<Face, FaceIndex> for Mesh {
    fn add(&mut self, face: Face) -> FaceIndex {
        let result = FaceIndex::new(self.face_list.len());
        self.face_list.push(face);
        return result;
    }
}

impl AddGeometry<face::FromEdgeLoop, FaceIndex> for Mesh {
    fn add(&mut self, params: face::FromEdgeLoop) -> FaceIndex {
        debug!("Creating a new face from an edge loop starting at {:?}", params.0);
        let result = self.add(Face::new(params.0));
        self.face_mut(result).edge_index = params.0;
        let edge_indices: Vec<EdgeIndex> = EdgeLoop::new(params.0, &self.edge_list).collect();
        for index in edge_indices {
            self.edge_mut(index).face_index = result;
        }
        return result;
    }
}

impl AddGeometry<triangle::FromVerts, FaceIndex> for Mesh {
    fn add(&mut self, verts: triangle::FromVerts) -> FaceIndex {
        debug!("Creating new face from 3 vertices {:?}, {:?}, {:?}", verts.0, verts.1, verts.2);
        debug_assert!(verts.0.is_valid());
        debug_assert!(verts.1.is_valid());
        debug_assert!(verts.2.is_valid());

        let e0 = self.add(edge::FromVerts(verts.0, verts.1));
        let e1 = self.add(edge::ExtendLoop(e0, verts.2));
        let e2 = self.add(edge::CloseLoop(e1, e0));

        let result = self.add(Face::new(e0));
        trace!("Created {:?} with {:?} as the root edge.", result, e0);

        self.edge_mut(e0).face_index = result;
        trace!("{:?} : {:?}", e0, result);
        self.edge_mut(e1).face_index = result;
        trace!("{:?} : {:?}", e1, result);
        self.edge_mut(e2).face_index = result;
        trace!("{:?} : {:?}", e2, result);

        return result;
    }
}

impl AddGeometry<triangle::FromOneEdge, FaceIndex> for Mesh {
    fn add(&mut self, params: triangle::FromOneEdge) -> FaceIndex {
        let triangle::FromOneEdge(e0, vert) = params;
        debug!("Creating new face extending from {:?}", e0);
        debug_assert!(vert.is_valid());
        debug_assert!(e0.is_valid());

        trace!("Creating new edge loop starting at {:?}", e0);
        let e1 = self.add(edge::ExtendLoop(e0, vert));
        trace!("Created {:?} by extending {:?} to {:?}", e1, e0, vert);
        let e2 = self.add(edge::CloseLoop(e1, e0));
        trace!("Created {:?} by bridging {:?} -> {:?}", e2, e1, e0);

        let result = self.add(Face::new(e0));
        trace!("Created {:?} with {:?} as the root edge.", result, e0);

        self.edge_mut(e0).face_index = result;
        trace!("{:?} : {:?}", e0, result);
        self.edge_mut(e1).face_index = result;
        trace!("{:?} : {:?}", e1, result);
        self.edge_mut(e2).face_index = result;
        trace!("{:?} : {:?}", e2, result);

        return result;
    }
}

impl AddGeometry<triangle::FromTwoEdges, FaceIndex> for Mesh {
    fn add(&mut self, params: triangle::FromTwoEdges) -> FaceIndex {
        use triangle::EdgeOrder;
        debug!("Creating new face from two edges {:?}, {:?} using EdgeOrder::{:?}", params.0, params.1, params.2);
        debug_assert!(params.0.is_valid());
        debug_assert!(params.1.is_valid());

        let (e0, e1, e2) = match params.2 {
            EdgeOrder::AC => {
                self.connect_edges(params.1, params.0);
                let e1 = self.add(edge::CloseLoop(params.0, params.1));
                trace!("Created {:?} by bridging {:?} to {:?}", e1, params.0, params.1);
                (params.0, e1, params.1)
            },
            EdgeOrder::AB => {
                self.connect_edges(params.0, params.1);
                let e2 = self.add(edge::CloseLoop(params.1, params.0));
                trace!("Created {:?} by bridging {:?} to {:?}", e2, params.1, params.0);
                (params.0, params.1, e2)
            },
            EdgeOrder::BC => {
                self.connect_edges(params.0, params.1);
                let e0 = self.add(edge::CloseLoop(params.1, params.0));
                trace!("Created {:?} by bridging {:?} to {:?}", e0, params.1, params.0);
                (e0, params.0, params.1)
            }
        };

        let result = self.add(Face::new(e0));
        trace!("Created {:?} with {:?} as the root edge.", result, e0);

        self.edge_mut(e0).face_index = result;
        trace!("{:?} : {:?}", e0, result);
        self.edge_mut(e1).face_index = result;
        trace!("{:?} : {:?}", e1, result);
        self.edge_mut(e2).face_index = result;
        trace!("{:?} : {:?}", e2, result);

        return result;
    }
}

impl AddGeometry<triangle::FromThreeEdges, FaceIndex> for Mesh {
    fn add(&mut self, params: triangle::FromThreeEdges) -> FaceIndex {
        let triangle::FromThreeEdges(e0, e1, e2) = params;
        trace!("Creating new face from {:?} -> {:?} -> {:?}", e0, e1, e2);
        debug_assert!(e0.is_valid());
        debug_assert!(e1.is_valid());
        debug_assert!(e2.is_valid());

        self.connect_edges(e0, e1);
        self.connect_edges(e1, e2);
        self.connect_edges(e2, e0);

        let result = self.add(Face::new(e0));
        trace!("Created {:?} with {:?} as the root edge.", result, e0);

        self.edge_mut(e0).face_index = result;
        trace!("{:?} : {:?}", e0, result);
        self.edge_mut(e1).face_index = result;
        trace!("{:?} : {:?}", e1, result);
        self.edge_mut(e2).face_index = result;
        trace!("{:?} : {:?}", e2, result);

        return result;
    }
}

impl AddGeometry<edge::FromVerts, EdgeIndex> for Mesh {
    fn add(&mut self, verts: edge::FromVerts) -> EdgeIndex {
        debug_assert!(verts.0.is_valid());
        debug_assert!(verts.1.is_valid());

        let eindex_a = EdgeIndex::new(self.edge_list.len());
        let eindex_b = EdgeIndex::new(eindex_a.offset + 1);

        let edge_a = Edge {
            twin_index: eindex_b,
            vertex_index: verts.0,
            ..Edge::default()
        };
        self.vertex_mut(verts.0).edge_index = eindex_a;

        let edge_b = Edge {
            twin_index: eindex_a,
            vertex_index: verts.1,
            ..Edge::default()
        };
        self.vertex_mut(verts.1).edge_index = eindex_b;

        let eindex_a = self.add(edge_a);
        debug_assert!(eindex_a == eindex_a);
        trace!("Created {:?} between {:?} and {:?}", eindex_a, verts.0, verts.1);
        let eindex_b = self.add(edge_b);
        debug_assert!(eindex_b == eindex_b);
        trace!("Created {:?} between {:?} and {:?}", eindex_b, verts.1, verts.0);

        return eindex_a;
    }
}

impl AddGeometry<edge::ExtendLoop, EdgeIndex> for Mesh {
    fn add(&mut self, params: edge::ExtendLoop) -> EdgeIndex {
        let edge::ExtendLoop(prev, vert) = params;
        debug!("Extending edge loop from {:?} to {:?}", prev, vert);
        debug_assert!(prev.is_valid()); //
        debug_assert!(vert.is_valid());
        let result = {
            debug_assert!(self.edge(prev).twin_index.is_valid());
            let prev_vert = self.edge_fn(prev).twin().vertex().index;
            self.add(edge::FromVerts(prev_vert, vert))
        };
        self.connect_edges(prev, result);
        return result;
    }
}

impl AddGeometry<edge::CloseLoop, EdgeIndex> for Mesh {
    fn add(&mut self, params: edge::CloseLoop) -> EdgeIndex {
        let edge::CloseLoop(prev, next) = params;
        debug!("Closing loop between {:?} and {:?}", prev, next);
        debug_assert!(prev.is_valid());
        debug_assert!(next.is_valid());
        let vindex_a = self.edge_fn(prev).twin().vertex().index;
        let vindex_b = self.edge_fn(next).vertex().index;
        let result = self.add(edge::FromVerts(vindex_a, vindex_b));
        self.connect_edges(prev, result);
        self.connect_edges(result, next);
        return result;
    }
}
