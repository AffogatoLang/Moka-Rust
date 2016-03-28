use std::result::Result;
pub trait ProgramFragment {
    fn run(&self) -> Result<(), &'static str>;
}

pub trait Configurable {
    fn set_flag(&self) -> bool;
    fn set_arg(&self, key:&str, value:&str) -> &str;
}
