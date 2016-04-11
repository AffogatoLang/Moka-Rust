use common::util::{ConfigurableProgram, ProgramFragment};

use common::module::Module;

use common::lexer::Lexer;

use std::option::Option;

use std::path::Path;

pub struct ParseBuilder{
    is_verbose : bool,
    is_archive : bool,
    module_path : Option<String>,
    input_path : Option<String>,
    output_path : Option<String>
}

impl ParseBuilder {
    pub fn new() -> ParseBuilder {
        ParseBuilder {
            is_verbose : false,
            is_archive : false,
            module_path : Option::None,
            input_path : Option::None,
            output_path : Option::None
        }
    }
}

impl ConfigurableProgram<ParseRunner> for ParseBuilder {
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
            "input" => self.input_path = Option::Some(value),
            "output" => self.output_path = Option::Some(value),
            _ => ()
        };
        self
    }
    fn config(&self) -> ParseRunner {
        let module = match self.module_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured ParseRunner, missing 'module' def")
        };

        let input = match self.input_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured ParseRunner, missing 'input' def")
        };

        let output = match self.output_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured ParseRunner, missing 'output' def")
        };

        ParseRunner {
            is_verbose: self.is_verbose,
            is_archive: self.is_archive,
            module_path: module.clone(),
            input_path: input.clone(),
            output_path: output.clone()
        }
    }
}

#[allow(dead_code)]
pub struct ParseRunner {
    is_verbose : bool,
    is_archive : bool,
    module_path : String,
    input_path : String,
    output_path : String
}

impl ProgramFragment for ParseRunner {
    fn run<'a>(&self) -> Result<(), String> {
        let module_conf = try!(Module::new(Path::new(&self.module_path)));
        let module_opts = match module_conf.get_opts() {
            Ok(opts) => opts,
            Err(e) => return Err(e)
        };

        if self.is_verbose {
            println!(r#"Found configuration for module named [{0}]
    :: Written by {1} and licensed under {2}
    :: Module Version {3}"#, module_opts.meta.name,
            module_opts.meta.author, module_opts.meta.license, module_opts.meta.version);
        }

        let mut lex = Lexer::from_dir(module_conf.sub_dir("lex"), self.is_verbose);
        lex.set_ignore_whitespace(module_opts.options.strip_whitespace.unwrap_or(false));
        let s : String = "RIGHTLY data #\nVERILY \"hello\" + data #".into();

        let tokens = try!(lex.tokenise(&s));

        Result::Ok(())
    }
}
