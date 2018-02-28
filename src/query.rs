//! Query interface and some provided implementations

use ordermap::OrderSet;
use super::Mesh;
use super::core::{EdgeIndex, FaceIndex, PointIndex, VertexIndex};

#[derive(Debug, Hash, PartialEq, Clone, Copy)]
pub enum Selection {
    Point(PointIndex),
    Vertex(VertexIndex),
    Edge(EdgeIndex),
    Face(FaceIndex),
}

impl Eq for Selection {}

impl Into<Selection> for PointIndex {
    fn into(self) -> Selection {
        Selection::Point(self)
    }
}

impl Into<Selection> for FaceIndex {
    fn into(self) -> Selection {
        Selection::Face(self)
    }
}

impl Into<Selection> for VertexIndex {
    fn into(self) -> Selection {
        Selection::Vertex(self)
    }
}

impl Into<Selection> for EdgeIndex {
    fn into(self) -> Selection {
        Selection::Edge(self)
    }
}

pub type SelectionSet = OrderSet<Selection>;

pub trait Query {
    fn exec(mesh: &Mesh, selection: &mut SelectionSet);
}
