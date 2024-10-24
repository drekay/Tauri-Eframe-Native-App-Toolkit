#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}