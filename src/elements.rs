
use std::cell::{Cell, RefCell, Ref, RefMut};
use super::{Tag, Generation};

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE,
}

/// Marker trait for structs holding element specific data
pub trait ElementData {}

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

/// Trait for accessing Mesh element properties.
#[derive(Debug, Clone)]
pub struct MeshElement<D: ElementData + Default> {
    tag: Cell<Tag>,
    generation: Cell<Generation>,
    status: Cell<ElementStatus>,
    data: RefCell<D>,
}

impl<D: ElementData + Default> Default for MeshElement<D> {
    fn default() -> Self {
        MeshElement {
            tag: Cell::new(0),
            generation: Cell::new(1),
            status: Cell::new(ElementStatus::INACTIVE),
            data: RefCell::default()
        }
    }
}

impl<D: ElementData + Default> MeshElement<D> {
    pub fn with_data(data: D) -> Self {
        MeshElement {
            data: RefCell::new(data),
            ..MeshElement::default()
        }
    }

    pub fn data(&self) -> Ref<D> {
        self.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<D> {
        self.data.borrow_mut()
    }
}

impl<D: ElementData + Default> Storable for MeshElement<D> {
    fn generation(&self) -> Generation {
        self.generation.get()
    }

    fn set_generation(&self, generation: Generation) {
        self.generation.set(generation);
    }

    fn status(&self) -> ElementStatus {
        self.status.get()
    }

    fn set_status(&self, status: ElementStatus) {
        self.status.set(status);
    }
}

impl<D: ElementData + Default> Taggable for MeshElement<D> {
    fn tag(&self) -> Tag {
        self.tag.get()
    }

    fn set_tag(&self, tag: Tag) {
        self.tag.set(tag);
    }
}

impl<D: ElementData + Default> IsActive for MeshElement<D> {
    fn is_active(&self) -> bool {
        self.status.get() == ElementStatus::ACTIVE
    }
}
