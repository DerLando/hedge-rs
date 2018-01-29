//! Operator interface and a set of initial implementations.

use failure::Error;

use super::Mesh;
use super::core::*;

#[derive(Debug, Fail)]
pub enum OperatorError {
    #[fail(display = "invalid arguments provided to operator.")]
    InvalidArguments,
}

///////////////////////////////////////////////////////////////////////////////

pub type ModifierResult = Result<(), Error>;

pub trait Modifier<Args> {
    fn cook(mesh: &mut Mesh, args: Args) -> ModifierResult;
}

///////////////////////////////////////////////////////////////////////////////

pub type GeneratorResult = Result<Mesh, Error>;

pub trait Generator<Args> {
    fn cook(args: Args) -> GeneratorResult;
}

///////////////////////////////////////////////////////////////////////////////

pub mod utils {
    use super::*;

    /// Given two vertex indices, create an adjacent edge pair
    pub fn add_edge_from_verts(mesh: &mut Mesh, v0: VertexIndex, v1: VertexIndex) -> EdgePair {
        let e0 = mesh.kernel.add(Edge {
            vertex_index: v0.into_cell(),
            ..Edge::default()
        });

        let e1 = mesh.kernel.add(Edge {
            twin_index: e0.into_cell(),
            vertex_index: v1.into_cell(),
            ..Edge::default()
        });

        mesh.kernel.get(&e0).twin_index.set(e1);

        EdgePair(e0, e1)
    }

    /// Given two point indices, create two vertices and an adjacent edge pair
    pub fn add_edge_from_points(mesh: &mut Mesh, p0: PointIndex, p1: PointIndex) -> EdgePair {
        let v0 = mesh.kernel.add(Vertex::from_point(p0));
        let v1 = mesh.kernel.add(Vertex::from_point(p1));

        add_edge_from_verts(mesh, v0, v1)
    }
}

///////////////////////////////////////////////////////////////////////////////
