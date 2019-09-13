
use std::cell::{Ref, RefMut};
use crate::handles::{
    HalfEdgeHandle, FaceHandle,
};
use crate::data::{Tag, Generation, Offset, ElementStatus};

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    fn is_valid(&self) -> bool;
}

pub trait Element: Default + Clone + Storable {
    type Data: ElementData;
    type Handle: ElementHandle;

    fn with_data(data: Self::Data) -> Self;

    fn data(&self) -> Ref<Self::Data>;
    fn data_mut(&self) -> RefMut<Self::Data>;
}

pub trait ElementData: Default {}

/// Marker trait for handle types
pub trait ElementHandle: Default + Copy + IsValid {
    type Element: Element;

    fn new(offset: Offset) -> Self;
    fn with_generation(offset: Offset, generation: Generation) -> Self;
    fn offset(&self) -> Offset;
    fn generation(&self) -> Generation;
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

    fn is_active(&self) -> bool {
        self.status() == ElementStatus::ACTIVE
    }
}

/// Interface for adding elements to a `Mesh`.
pub trait AddElement<E> where E: Element {
    fn add(&mut self, element: E) -> E::Handle;
}

/// Interface for removing elements to a `Mesh`.
pub trait RemoveElement<H> where H: ElementHandle {
    fn remove(&mut self, handle: H);
}

/// Interface for getting an element reference.
pub trait GetElement<H> where H: ElementHandle {
    fn get(&self, handle: H) -> Option<&<H as ElementHandle>::Element>;
}

pub trait MakeEdgePair<A> {
    fn make_edge_pair(&mut self, args: A) -> HalfEdgeHandle;
}

pub trait AddFace<A> {
    fn add_face(&mut self, args: A) -> FaceHandle;
}
