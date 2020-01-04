use crate::data::{Index, Generation, Handle};

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    fn is_valid(&self) -> bool;
}

pub trait Storable {
    fn make_handle(index: Index, generation: Generation) -> Handle;
}

pub trait MakeFace<A> {
    fn make_face(&mut self, args: A) -> Handle;
}
