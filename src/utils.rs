use log::*;
use crate::mesh::Mesh;
use crate::handles::{
    HalfEdgeHandle, FaceHandle,
    VertexHandle,
};
use crate::data::{
    HalfEdgeData,
};
use crate::elements::*;
use crate::proxy::*;
use crate::traits::Element;

/// Given two vertex indices, create an adjacent edge pair
pub fn build_full_edge(
    mesh: &mut Mesh,
    v0: VertexHandle,
    v1: VertexHandle
) -> HalfEdgeHandle {
    let e0: HalfEdgeHandle = mesh.add(
        HalfEdge::with_data(HalfEdgeData {
            vertex: v0,
            ..HalfEdgeData::default()
        }));

    let e1 = mesh.add(
        HalfEdge::with_data(HalfEdgeData {
            adjacent: e0,
            vertex: v1,
            ..HalfEdgeData::default()
        }));

    if let Some(edge) = mesh.get(e0) {
        edge.data_mut().adjacent = e1;
    }
    mesh.get(v0).map(|e| e.data_mut().edge = e0);
    mesh.get(v1).map(|e| e.data_mut().edge = e1);

    return e0;
}

pub fn build_half_edge(
    mesh: &mut Mesh,
    adjacent: HalfEdgeHandle,
    vertex: VertexHandle,
) -> HalfEdgeHandle {
    let e0 = mesh.add(
        HalfEdge::with_data(HalfEdgeData {
            vertex,
            adjacent,
            ..HalfEdgeData::default()
        })
    );

    mesh.get(adjacent).map(|e| e.data_mut().adjacent = e0);
    mesh.get(vertex).map(|v| v.data_mut().edge = e0);

    return e0;
}

pub fn assoc_vert_edge(
    mesh: &Mesh,
    vert: VertexHandle,
    edge: HalfEdgeHandle
) {
    mesh.get(vert).map(|v| v.data_mut().edge = edge);
    mesh.get(edge).map(|e| e.data_mut().vertex = vert);
}

/// Given an edge index, and a vertex index, creates a new edge connected to the specified edge
pub fn build_full_edge_from(
    mesh: &mut Mesh,
    prev: HalfEdgeHandle,
    v1: VertexHandle
) -> HalfEdgeHandle {
    let e0 = {
        let v0 = mesh.edge(prev).adjacent().vertex().handle;
        build_full_edge(mesh, v0, v1)
    };
    connect_edges(mesh, prev, e0);
    return e0;
}

pub fn close_edge_loop(
    mesh: &mut Mesh,
    prev: HalfEdgeHandle,
    next: HalfEdgeHandle
) -> HalfEdgeHandle {
    let v0 = mesh.edge(prev).adjacent().element().map(|e| e.data().vertex);
    let v1 = mesh.edge(next).element().map(|e| e.data().vertex);

    if let (Some(v0), Some(v1)) = (v0, v1) {
        let e0 = build_full_edge(mesh, v0, v1);
        connect_edges(mesh, prev, e0);
        connect_edges(mesh, e0, next);
        e0
    } else {
        error!("Failed to properly discover associated vertices.");
        HalfEdgeHandle::default()
    }
}

/// Associates a previous and next edge
pub fn connect_edges(
    mesh: &mut Mesh,
    prev: HalfEdgeHandle,
    next: HalfEdgeHandle
) {
    mesh.get(prev).map(|e| e.data_mut().next = next);
    mesh.get(next).map(|e| e.data_mut().prev = prev);
}

pub fn assign_face_to_loop(
    mesh: &Mesh,
    root_edge_index: HalfEdgeHandle,
    face_index: FaceHandle
) {
    let face = mesh.face(face_index);
    if let Some(mut data) = face.data_mut() {
        data.root_edge = root_edge_index;
    } else {
        error!("Invalid face index specified: {:?}", face_index);
        return;
    }
    let mut edge = face.root_edge();
    loop {
        if let Some(mut data) = edge.data_mut() {
            if data.face == face.handle {
                break;
            }
            data.face = face.handle;
            if data.next == root_edge_index {
                break;
            }
        } else {
            error!("Invalid edge index! {:?}", edge.handle);
            break;
        }
        edge = edge.next();
    }
}
