use common::util::ConfigurableProgram;
use common::util::ProgramFragment;

use std::option::Option;

pub struct CompileBuilder{
    is_verbose : bool,
    is_archive : bool,
    module_path : Option<String>,
    output_path : Option<String>
}

impl CompileBuilder {
    pub fn new() -> CompileBuilder {
        CompileBuilder {
            is_verbose : false,
            is_archive : false,
            module_path : Option::None,
            output_path : Option::None
        }
    }
}

impl ConfigurableProgram<CompileRunner> for CompileBuilder {
    fn set_flag(&mut self, name:String, value:bool) -> &mut Self {
        let n:&str = &name;
        match n {
            "verbose" => self.is_verbose = value,
            "archive" => self.is_archive = value,
            _ => ()
        };
        self
    }
    fn set_arg(&mut self, name:String, value:String) -> &mut Self {
        let n:&str = &name;
        match n {
            "module" => self.module_path = Option::Some(value),
            "output" => self.output_path = Option::Some(value),
            _ => ()
        };
        self
    }
    fn config(&self) -> CompileRunner {
        CompileRunner
    }
}

struct CompileRunner;
impl ProgramFragment for CompileRunner {
    fn run(&self) -> Result<(), String> {
        Result::Ok(())
    }
}
