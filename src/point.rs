
use super::{Point, IsValid, ElementStatus};


impl IsValid for Point {
    fn is_valid(&self) -> bool {
        let props = self.props.borrow();
        props.status == ElementStatus::ACTIVE
    }
}
