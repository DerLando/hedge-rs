use super::*;

/// Given two vertex indices, create an adjacent edge pair
pub fn build_full_edge(mesh: &mut Mesh, v0: VertexIndex, v1: VertexIndex) -> EdgeIndex {
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
  mesh.kernel.get(&v0).edge_index.set(e0);
  mesh.kernel.get(&v1).edge_index.set(e1);

  return e0;
}

/// Given an edge index, and a vertex index, creates a new edge connected to the specified edge
pub fn build_full_edge_from(mesh: &mut Mesh, prev: EdgeIndex, v1: VertexIndex) -> EdgeIndex {
  let e0 = {
    let v0 = mesh.edge(prev).twin().vertex().index;
    build_full_edge(mesh, v0, v1)
  };
  connect_edges(mesh, prev, e0);
  return e0;
}

pub fn close_edge_loop(mesh: &mut Mesh, prev: EdgeIndex, next: EdgeIndex) -> EdgeIndex {
  let v0 = mesh.edge(prev).next().element.vertex_index.get();
  let v1 = mesh.edge(next).element.vertex_index.get();

  let e0 = build_full_edge(mesh, v0, v1);

  connect_edges(mesh, prev, e0);
  connect_edges(mesh, e0, next);

  return e0;
}

/// Associates a previous and next edge
pub fn connect_edges(mesh: &mut Mesh, prev: EdgeIndex, next: EdgeIndex) {
  mesh.edge(prev).element.next_index.set(next);
  mesh.edge(next).element.prev_index.set(prev);
}
