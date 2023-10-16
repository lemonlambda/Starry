pub trait SystemOrdering: Into<i32> + Copy {}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum DefaultOrdering {
    PreRun = 1,
    Run = 2,
    PostRun = 3
}

impl Into<i32> for DefaultOrdering {
    fn into(self) -> i32 {
        self as i32
    }
}
impl SystemOrdering for DefaultOrdering {}