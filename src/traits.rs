use crate::data::{ElementStatus, Generation, Index, Tag};
use crate::handles::{FaceHandle, HalfEdgeHandle};
use std::cell::{Ref, RefMut};

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

    fn new(index: Index) -> Self;
    fn with_generation(index: Index, generation: Generation) -> Self;
    fn index(&self) -> Index;
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
pub trait AddElement<E>
where
    E: Element,
{
    fn add(&mut self, element: E) -> E::Handle;
}

/// Interface for removing elements to a `Mesh`.
pub trait RemoveElement<H>
where
    H: ElementHandle,
{
    fn remove(&mut self, handle: H);
}

/// Interface for getting an element reference.
pub trait GetElement<H>
where
    H: ElementHandle,
{
    fn get(&self, handle: H) -> Option<&<H as ElementHandle>::Element>;
}

pub trait MakeEdge<A> {
    fn make_edge(&mut self, args: A) -> (HalfEdgeHandle, HalfEdgeHandle);
}

pub trait AddFace<A> {
    fn add_face(&mut self, args: A) -> FaceHandle;
}

// pub trait Bridge<A> {
//     fn bridge(&mut self, args: A) -> FaceHandle;
// }

// pub trait Extrude<A> {
//     fn extrude(&mut self, args: A) -> FaceHandle;
// }
