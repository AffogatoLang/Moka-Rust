use std::io;
use std::collections::HashMap;
use common::util::Loader;
use common::module::LexFile;
use common::module::LangFile;
use common::module::InterpFile;

struct ModuleOpts<'a> {
    name: String,
    version: String,
    author: String,
    license: String,
    core: Option<String>,
    options: HashMap<&'a str, &'a str>
}

impl<'a> ModuleOpts<'a> {
    fn optAsBool(&self, key: &'a str) -> Option<bool> {
        let optionalopt = self.options.get(key);
        let opt = match optionalopt {
            Some(val) => val,
            None => return None
        };
        match *opt {
            "true" => Some(true),
            "false" => Some(false),
            _ => None
        }
    }
}

struct Module<'b> {
    path: String,
    lex_files: Vec<LexFile>,
    lang_files: Vec<LangFile>,
    interp_files: Vec<InterpFile>,
    options: ModuleOpts<'b>
}

pub struct ModuleLoader {
    path: String
}

// impl Loader<Module> for ModuleLoader {
//     fn load(&self) -> Result<Module, io::Error> {
//
//     }
// }
