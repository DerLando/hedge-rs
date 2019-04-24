//! Iterators for simple or common mesh traversal patterns.

use std::slice::Iter;
use std::iter::Enumerate;
use log::*;
use super::*;

pub struct ElementEnumerator<'mesh, E> {
    tag: Tag,
    iter: Enumerate<Iter<'mesh, E>>,
}

impl<'mesh, E: IsActive + Taggable + Indexable> ElementEnumerator<'mesh, E> {
    pub fn new(tag: Tag, iter: Enumerate<Iter<'mesh, E>>) -> ElementEnumerator<'mesh, E> {
        debug!("New element enumerator");
        ElementEnumerator { tag, iter }
    }

    pub fn next_element(&mut self) -> Option<(Index<E>, &'mesh E)> {
        for (offset, element) in self.iter.by_ref() {
            let is_next = element.is_active() && element.tag() != self.tag;
            if is_next {
                element.set_tag(self.tag);
                let index = Index::with_generation(offset, element.generation());
                return Some((index, element));
            }
        }
        debug!("Element enumeration completed.");
        return None;
    }
}

pub type VertexEnumerator<'mesh> = ElementEnumerator<'mesh, Vertex>;
pub type FaceEnumerator<'mesh> = ElementEnumerator<'mesh, Face>;
pub type EdgeEnumerator<'mesh> = ElementEnumerator<'mesh, Edge>;
pub type PointEnumerator<'mesh> = ElementEnumerator<'mesh, Point>;

