//! Iterators for simple or common mesh traversal patterns.

use super::*;

use std::slice::Iter;
use std::iter::Enumerate;


pub struct ElementEnumerator<'mesh, E: 'mesh + MeshElement> {
    tag: Tag,
    iter: Enumerate<Iter<'mesh, E>>,
}

impl<'mesh, E> ElementEnumerator<'mesh, E>
    where E: 'mesh + MeshElement
{
    pub fn new(tag: Tag, iter: Enumerate<Iter<'mesh, E>>) -> ElementEnumerator<'mesh, E> {
        debug!("New element enumerator");
        ElementEnumerator {
            tag,
            iter,
        }
    }

    fn next_element(&mut self) -> Option<(Index<E>, &'mesh E)> {
        for (offset, element) in self.iter.by_ref() {
            let props = element.props();
            let is_next = props.status.get() == ElementStatus::ACTIVE &&
                props.tag.get() != self.tag;
            if is_next {
                props.tag.set(self.tag);
                let index = Index::with_generation(offset, props.generation.get());
                return Some((index, element));
            }
        }
        debug!("Element enumeration completed.");
        return None;
    }
}

type VertexEnumerator<'mesh> = ElementEnumerator<'mesh, Vertex>;
type FaceEnumerator<'mesh> = ElementEnumerator<'mesh, Face>;
type EdgeEnumerator<'mesh> = ElementEnumerator<'mesh, Edge>;
type PointEnumerator<'mesh> = ElementEnumerator<'mesh, Point>;

////////////////////////////////////////////////////////////////////////////////

pub struct VertexFnIterator<'mesh> {
    enumerator: VertexEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> VertexFnIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> VertexFnIterator<'mesh> {
        VertexFnIterator {
            enumerator: VertexEnumerator::new(mesh.next_tag(), mesh.kernel.vertex_buffer.enumerate()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for VertexFnIterator<'mesh> {
    type Item = VertexFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, vert)) = self.enumerator.next_element() {
            debug!("Found vertex {:?} - {:?}", index, vert);
            return Some(VertexFn::from_index_and_item(index, vert, self.mesh));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct FaceFnIterator<'mesh> {
    enumerator: FaceEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> FaceFnIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> FaceFnIterator<'mesh> {
        FaceFnIterator {
            enumerator: FaceEnumerator::new(mesh.next_tag(), mesh.kernel.face_buffer.enumerate()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for FaceFnIterator<'mesh> {
    type Item = FaceFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, face)) = self.enumerator.next_element() {
            debug!("Found face {:?} - {:?}", index, face);
            return Some(FaceFn::from_index_and_item(index, face, self.mesh));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct EdgeFnIterator<'mesh> {
    enumerator: EdgeEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> EdgeFnIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> EdgeFnIterator<'mesh> {
        EdgeFnIterator {
            enumerator: EdgeEnumerator::new(mesh.next_tag(), mesh.kernel.edge_buffer.enumerate()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for EdgeFnIterator<'mesh> {
    type Item = EdgeFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, edge)) = self.enumerator.next_element() {
            debug!("Found edge {:?} - {:?}", index, edge);
            return Some(EdgeFn::from_index_and_item(index, edge, self.mesh));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PointIterator<'mesh> {
    enumerator: PointEnumerator<'mesh>,
    mesh: &'mesh Mesh,
}

impl<'mesh> PointIterator<'mesh> {
    pub fn new(mesh: &'mesh Mesh) -> PointIterator<'mesh> {
        PointIterator {
            enumerator: PointEnumerator::new(mesh.next_tag(), mesh.kernel.point_buffer.enumerate()),
            mesh,
        }
    }
}

impl<'mesh> Iterator for PointIterator<'mesh> {
    type Item = &'mesh Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, edge)) = self.enumerator.next_element() {
            debug!("Found edge {:?} - {:?}", index, edge);
            return Some(self.mesh.point(index));
        }
        return None;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct FaceEdges<'mesh> {
    tag: Tag,
    edge: EdgeFn<'mesh>,
}

impl<'mesh> FaceEdges<'mesh> {
    pub fn new(tag: Tag, edge: EdgeFn<'mesh>) -> FaceEdges<'mesh> {
        FaceEdges {
            tag,
            edge,
        }
    }
}
