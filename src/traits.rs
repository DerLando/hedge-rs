use crate::data::{Generation, Index};
use crate::handles::Handle;

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    fn is_valid(&self) -> bool;
}

pub trait AddElement<E> {
    fn add(&mut self, element: E) -> Handle;
}

pub trait RemoveElement {
    fn remove(&mut self, handle: impl Into<Handle>);
}

pub trait GetElement<E> {
    fn get(&self, handle: impl Into<Handle>) -> Option<&E>;
}

pub trait MakeFace<A> {
    fn make_face(&mut self, args: A) -> Handle;
}
