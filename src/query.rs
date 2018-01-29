//! Query interface and some provided implementations

use std::collections::HashSet;

use super::Mesh;
use super::core::*;
//use super::iterators::*;

pub enum SelectionSet {
  Vertices(HashSet<VertexIndex>),
  Edges(HashSet<EdgeIndex>),
  Faces(HashSet<FaceIndex>),
  Points(HashSet<PointIndex>),
  EdgeLoop(Vec<EdgeIndex>),
}

pub trait Query<Args> {
  fn exec(mesh: &Mesh, args: Args) -> SelectionSet;
}
