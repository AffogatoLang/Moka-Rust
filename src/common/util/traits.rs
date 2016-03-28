pub trait ProgramFragment {
    fn run(&self) -> i32;
}

pub trait Configurable {
    fn set_flag(&self) -> bool;
    fn set_arg(&self, key:&str, value:&str) -> &str;
}
