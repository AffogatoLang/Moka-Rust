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
    fn set_flag(&mut self, name:&str, value:bool) -> &mut Self {
        match name {
            "verbose" => self.is_verbose = value,
            "archive" => self.is_archive = value,
            _ => ()
        };
        self
    }
    fn set_arg(&mut self, name:&str, value:String) -> &mut Self {
        match name {
            "module" => self.module_path = Option::Some(value),
            "output" => self.output_path = Option::Some(value),
            _ => ()
        };
        self
    }
    fn config(&self) -> CompileRunner {
        let module = match self.module_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured CompileRunner, missing 'module' def")
        };

        let output = match self.output_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured CompileRunner, missing 'output' def")
        };

        CompileRunner {
            is_verbose: self.is_verbose,
            is_archive: self.is_archive,
            module_path: module.clone(),
            output_path: output.clone()
        }
    }
}

#[allow(dead_code)]
struct CompileRunner {
    is_verbose : bool,
    is_archive : bool,
    module_path : String,
    output_path : String
}

impl ProgramFragment for CompileRunner {
    fn run(&self) -> Result<(), String> {
        Result::Ok(())
    }
}
