use log::*;
use super::*;

/// Given two vertex indices, create an adjacent edge pair
pub fn build_full_edge(
    mesh: &mut Mesh,
    v0: VertexHandle,
    v1: VertexHandle
) -> EdgeHandle {
    let e0 = mesh.add_element(
        Edge::with_data(EdgeData {
            vertex: v0,
            ..EdgeData::default()
        }));

    let e1 = mesh.add_element(
        Edge::with_data(EdgeData {
            adjacent: e0,
            vertex: v1,
            ..EdgeData::default()
        }));

    mesh.get_element(&e0).map(|e| e.data_mut().adjacent = e1);
    mesh.get_element(&v0).map(|e| e.data_mut().edge = e0);
    mesh.get_element(&v1).map(|e| e.data_mut().edge = e1);

    return e0;
}

pub fn build_half_edge(
    mesh: &mut Mesh,
    adjacent: EdgeHandle,
    vertex: VertexHandle,
) -> EdgeHandle {
    let e0 = mesh.add_element(
        Edge::with_data(EdgeData {
            vertex,
            adjacent,
            ..EdgeData::default()
        })
    );

    mesh.get_element(&adjacent).map(|e| e.data_mut().adjacent = e0);
    mesh.get_element(&vertex).map(|v| v.data_mut().edge = e0);

    return e0;
}

pub fn assoc_vert_edge(
    mesh: &Mesh,
    vert: VertexHandle,
    edge: EdgeHandle
) {
    mesh.get_element(&vert).map(|v| v.data_mut().edge = edge);
    mesh.get_element(&edge).map(|e| e.data_mut().vertex = vert);
}

/// Given an edge index, and a vertex index, creates a new edge connected to the specified edge
pub fn build_full_edge_from(
    mesh: &mut Mesh,
    prev: EdgeHandle,
    v1: VertexHandle
) -> EdgeHandle {
    let e0 = {
        let v0 = mesh.edge(prev).twin().vertex().index;
        build_full_edge(mesh, v0, v1)
    };
    connect_edges(mesh, prev, e0);
    return e0;
}

pub fn close_edge_loop(
    mesh: &mut Mesh,
    prev: EdgeHandle,
    next: EdgeHandle
) -> EdgeHandle {
    let v0 = mesh.edge(prev).twin().element().map(|e| e.data().vertex);
    let v1 = mesh.edge(next).element().map(|e| e.data().vertex);

    if let (Some(v0), Some(v1)) = (v0, v1) {
        let e0 = build_full_edge(mesh, v0, v1);
        connect_edges(mesh, prev, e0);
        connect_edges(mesh, e0, next);
        e0
    } else {
        error!("Failed to properly discover associated vertices.");
        EdgeHandle::default()
    }
}

/// Associates a previous and next edge
pub fn connect_edges(
    mesh: &mut Mesh,
    prev: EdgeHandle,
    next: EdgeHandle
) {
    mesh.get_element(&prev).map(|e| e.data_mut().next = next);
    mesh.get_element(&next).map(|e| e.data_mut().prev = prev);
}

pub fn assign_face_to_loop(
    mesh: &Mesh,
    root_edge_index: EdgeHandle,
    face_index: FaceHandle
) {
    let face = mesh.face(face_index);
    if let Some(mut data) = face.data_mut() {
        data.edge = root_edge_index;
    } else {
        error!("Invalid face index specified: {:?}", face_index);
        return;
    }
    let mut edge = face.edge();
    loop {
        if let Some(mut data) = edge.data_mut() {
            if data.face == face.index {
                break;
            }
            data.face = face.index;
            if data.next == root_edge_index {
                break;
            }
        } else {
            error!("Invalid edge index! {:?}", edge.index);
            break;
        }
        edge = edge.next();
    }
}
