
use super::data::{Tag, Generation, ElementStatus};
use super::handles::Handle;

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    fn is_valid(&self) -> bool;
}

/// Marker trait for structs holding element specific data
pub trait ElementData {}

/// Marker trait for handle types
pub trait ElementHandle {}

pub trait IsActive {
    fn is_active(&self) -> bool;
}

pub trait Taggable {
    fn tag(&self) -> Tag;
    fn set_tag(&self, tag: Tag);
}

pub trait Storable {
    fn generation(&self) -> Generation;
    fn set_generation(&self, generation: Generation);
    fn status(&self) -> ElementStatus;
    fn set_status(&self, status: ElementStatus);
}

/// Interface for adding elements to a `Mesh`.
pub trait AddElement<E> {
    fn add_element(&mut self, element: E) -> Handle<E>;
}

/// Interface for removing elements to a `Mesh`.
pub trait RemoveElement<E> {
    fn remove_element(&mut self, index: Handle<E>);
}

/// Interface for getting an element reference.
pub trait GetElement<E> {
    fn get_element(&self, index: &Handle<E>) -> Option<&E>;
}
