/// A marker trait to say what is an enum for SystemOrdering
pub trait SystemOrdering: Into<i32> + Copy {}

/// A default enum for SystemOrdering
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum DefaultOrdering {
    /// Runs first
    PreRun = 1,
    /// Runs second
    Run = 2,
    /// Runs last
    PostRun = 3
}

impl Into<i32> for DefaultOrdering {
    fn into(self) -> i32 {
        self as i32
    }
}
impl SystemOrdering for DefaultOrdering {}