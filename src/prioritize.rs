use std::cmp::Ordering;
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Priority {
    pub op_priority: u16,
    pub spaces: u16,
}
impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.spaces.partial_cmp(&self.spaces) {
            ord @ (Some(Ordering::Less) | Some(Ordering::Greater)) => ord,
            _ => self.op_priority.partial_cmp(&other.op_priority),
        }
    }
}
impl std::fmt::Debug for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "-{}+{}", 9 - self.spaces, self.op_priority)
    }
}
impl Priority {
    pub const fn new(op_priority: u16) -> Self {
        Self {
            op_priority,
            spaces: 0,
        }
    }

    pub const fn min() -> Self {
        Self {
            op_priority: 0,
            spaces: 0xffff,
        }
    }
    pub fn space(&mut self) -> Self {
        self.spaces += 1;
        *self
    }
}
