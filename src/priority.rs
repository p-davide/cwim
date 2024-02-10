#[derive(Debug, PartialEq, Eq)]
struct Priority {
    op_priority: u16,
    spaces: u16,
    parens: u16
}
impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.parens.partial_cmp(other.parens) {
            ord @ (Some(Less) | Some(Greater)) => Some(Less),
        }
    }
};
