use common::util::Loader;

use common::module::LexFile;
use common::module::LangFile;
use common::module::InterpFile;

struct Module {
    path: String,
    is_valid: bool,
    lex_files: Vec<LexFile>,
    lang_files: Vec<LangFile>,
    interp_files: Vec<InterpFile>
}


pub struct ModuleLoader {
    path: String
}

impl Loader<Module> for ModuleLoader {
    fn load(&self, path:String) -> Result<Module, String> {
        Err("Not Yet Implemented".to_string())
    }
}
