use std::result::Result;
pub trait ProgramFragment {
    fn run(&self) -> Result<(), String>;
}

pub trait ConfigurableProgram<T:ProgramFragment> {
    fn set_flag(&mut self, key:&str, value:bool) -> &mut Self;
    fn set_arg(&mut self, key:&str, value:String) -> &mut Self;
    fn config(&self) -> T; // Should panic on incorrectly defined Programs, User cannot recover
}
