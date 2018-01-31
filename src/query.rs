//! Query interface and some provided implementations

use ordermap::OrderSet;
use super::Mesh;
use super::core::*;
//use super::iterators::*;

pub enum Selection {
  Vertices(OrderSet<VertexIndex>),
  Edges(OrderSet<EdgeIndex>),
  Faces(OrderSet<FaceIndex>),
  Points(OrderSet<PointIndex>),
  EdgeLoop(Vec<EdgeIndex>),
  Empty,
}

pub trait Query<Args> {
  fn exec(mesh: &Mesh, args: Args) -> Selection;
}
