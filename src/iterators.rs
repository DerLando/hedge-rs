//! Iterators for simple or common mesh traversal patterns.

use super::*;

use std::slice::Iter;
use std::iter::Enumerate;


pub struct FaceFnIterator<'mesh> {
    tag: Tag,
    iter: Enumerate<Iter<'mesh, Face>>,
    mesh: &'mesh Mesh,
}

impl<'mesh> FaceFnIterator<'mesh> {
    pub fn new(tag: Tag, iter: Enumerate<Iter<'mesh, Face>>, mesh: &'mesh Mesh) -> FaceFnIterator<'mesh> {
        FaceFnIterator {
            tag,
            iter,
            mesh,
        }
    }
}

impl<'mesh> Iterator for FaceFnIterator<'mesh> {
    type Item = FaceFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        for (offset, face) in self.iter.by_ref() {
            let is_next = face.props().status.get() == ElementStatus::ACTIVE &&
                face.props().tag.get() != self.tag;
            if is_next {
                debug!("{:?} is being tagged and returned.", face);
                face.props().tag.set(self.tag);
                let index = FaceIndex::with_generation(offset, face.props().generation.get());
                return Some(FaceFn::from_index_and_item(index, face, self.mesh));
            } else {
                debug!("{:?} is not an active element", face);
            }
        }
        return None;
    }
}

///// An iterator that returns the `FaceIndex` of every Face in the mesh.
/////
///// Currently this does not iterate using connectivity information but will
///// perhaps do this in the future.
//pub struct Faces {
//    face_count: usize,
//    previous_index: FaceIndex
//}

//impl Faces {
//    pub fn new(face_count: usize) -> Faces {
//        Faces {
//            face_count,
//            previous_index: FaceIndex::default()
//        }
//    }
//}

// TODO: iterate over faces based on connectivity?
//impl Iterator for Faces {
//    type Item = FaceIndex;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        self.previous_index = FaceIndex::new(self.previous_index.offset + 1);
//        if self.previous_index.offset >= self.face_count {
//            None
//        } else {
//            Some(self.previous_index)
//        }
//    }
//}

//pub struct EdgesAroundVertex<'mesh> {
//    mesh: &'mesh Mesh,
//    last_index: EdgeIndex,
//    next_index: EdgeIndex,
//}
//
//impl<'mesh> EdgesAroundVertex<'mesh> {
//    pub fn new(edge_index: EdgeIndex, mesh: &'mesh Mesh) -> EdgesAroundVertex<'mesh> {
//        EdgesAroundVertex {
//            mesh,
//            last_index: EdgeIndex::default(),
//            next_index: edge_index,
//        }
//    }
//}
//
//impl<'mesh> Iterator for EdgesAroundVertex<'mesh> {
//    type Item = EdgeIndex;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        self.last_index = self.next_index;
//        if self.last_index.is_valid() {
//            self.next_index = self.mesh.edge_fn(self.last_index)
//                .prev().twin().index;
//            Some(self.last_index)
//        } else {
//            None
//        }
//    }
//}

///// An iterator that walks an edge loop around a face returning each `VertexIndex` in the loop.
// yeah yeah yeah, I know this is copypasta...
//pub struct EdgeLoopVertices<'mesh> {
//    edge_list: &'mesh EdgeList,
//    initial_index: EdgeIndex,
//    current_index: EdgeIndex
//}

//impl<'mesh> EdgeLoopVertices<'mesh> {
//    pub fn new(index: EdgeIndex, edge_list: &'mesh EdgeList) -> EdgeLoopVertices {
//        EdgeLoopVertices {
//            edge_list,
//            initial_index: index,
//            current_index: EdgeIndex::default()
//        }
//    }
//}

//impl<'mesh> Iterator for EdgeLoopVertices<'mesh> {
//    type Item = VertexIndex;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        if self.current_index.is_valid() {
//            self.edge_list.get(self.current_index.offset)
//                .and_then(|last_edge| {
//                    self.current_index = last_edge.next_index;
//                    if self.current_index == self.initial_index {
//                        None
//                    } else {
//                        self.edge_list.get(self.current_index.offset)
//                            .map(|e| e.vertex_index)
//                    }
//                })
//        } else {
//            if self.initial_index.is_valid() {
//                self.current_index = self.initial_index;
//                self.edge_list.get(self.current_index.offset).map(|e| e.vertex_index)
//            } else {
//                None
//            }
//        }
//    }
//}

///// An iterator that walks an edge loop around a face returning each `EdgeIndex` in the loop.
//pub struct EdgeLoop<'mesh> {
//    edge_list: &'mesh EdgeList,
//    initial_index: EdgeIndex,
//    current_index: EdgeIndex
//}

//impl<'mesh> EdgeLoop<'mesh> {
//    pub fn new(index: EdgeIndex, edge_list: &'mesh EdgeList) -> EdgeLoop {
//        EdgeLoop {
//            edge_list,
//            initial_index: index,
//            current_index: EdgeIndex::default()
//        }
//    }
//}

//impl<'mesh> Iterator for EdgeLoop<'mesh> {
//    type Item = EdgeIndex;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        if self.current_index.is_valid() {
//            self.edge_list.get(self.current_index.offset).and_then(|current_edge| {
//                self.current_index = current_edge.next_index;
//                if self.current_index == self.initial_index {
//                    None
//                } else {
//                    Some(self.current_index)
//                }
//            })
//        } else {
//            if self.initial_index.is_valid() {
//                self.current_index = self.initial_index;
//                Some(self.current_index)
//            } else {
//                None
//            }
//        }
//    }
//}
