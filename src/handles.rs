use std::cmp;
use std::hash::{Hash, Hasher};

use crate::data::{Generation, ComponentID};
use crate::traits::IsValid;

#[derive(Debug, Copy, Clone, Eq, Default)]
pub struct Handle {
    id: ComponentID,
    generation: Option<Generation>,
}

impl Handle {
    pub fn for_id(id: ComponentID) -> Self {
        Handle {
            id,
            generation: None,
        }
    }

    pub fn new(id: ComponentID, generation: Generation) -> Self {
        Handle {
            id,
            generation: Some(generation),
        }
    }
}

impl From<ComponentID> for Handle {
    fn from(index: ComponentID) -> Self {
        Handle::for_id(index)
    }
}

impl Hash for Handle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for Handle {
    fn partial_cmp(&self, other: &Handle) -> Option<cmp::Ordering> {
        // Only the index should matter when it comes to ordering
        self.id.partial_cmp(&other.id)
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Handle) -> bool {
        if self.generation.is_none() {
            self.id.eq(&other.id)
        } else {
            self.id.eq(&other.id) && self.generation.eq(&other.generation)
        }
    }
}

impl IsValid for Handle {
    fn is_valid(&self) -> bool {
        self.id != ComponentID::Invalid
    }
}
