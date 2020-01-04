use std::cmp;
use std::hash::{Hash, Hasher};

use crate::data::{Generation, Index, IGNORED_GENERATION};
use crate::traits::IsValid;

#[derive(Debug, Copy, Clone, Eq, Default)]
pub struct Handle {
    index: Index,
    generation: Generation,
}

impl Handle {
    pub fn new(index: Index, generation: Generation) -> Self {
        Handle {
            index,
            generation,
        }
    }
}

impl From<Index> for Handle {
    fn from(index: Index) -> Self {
        Handle::new(index, IGNORED_GENERATION)
    }
}

impl Hash for Handle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl PartialOrd for Handle {
    fn partial_cmp(&self, other: &Handle) -> Option<cmp::Ordering> {
        // Only the index should matter when it comes to ordering
        self.index.partial_cmp(&other.index)
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Handle) -> bool {
        if self.generation == IGNORED_GENERATION {
            self.index.eq(&other.index)
        } else {
            self.index.eq(&other.index) && self.generation.eq(&other.generation)
        }
    }
}

impl IsValid for Handle {
    fn is_valid(&self) -> bool {
        self.index != Index::Invalid
    }
}
