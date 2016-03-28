use std::result::Result;
pub trait ProgramFragment {
    fn run(&self) -> Result<(), &'static str>;
}

pub trait Configurable<T> {
    fn set_flag(&self, key:&str, value:bool) -> &Self;
    fn set_arg(&self, key:&str, value:&str) -> &Self;
    fn config(&self) -> T;
}
