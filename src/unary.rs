use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Unary {
    pub name: &'static str,
    pub f: fn(f64) -> f64,
}

pub const COS: Unary = Unary {
    name: "cos",
    f: |x| x.cos()
};


// impl Debug for Unary {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "[{}]", self.name);
//     }
// }