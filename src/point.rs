
use super::{Point, IsValid, IsActive};


impl IsValid for Point {
    fn is_valid(&self) -> bool {
        self.is_active()
    }
}
