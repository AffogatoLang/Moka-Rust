use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use toml;

#[derive(Debug, RustcDecodable)]
pub struct ModuleOpts {
    pub meta: MetaOpts,
    pub options: OptionsOpts
}

impl ModuleOpts {
    pub fn load(path: String) -> Result<ModuleOpts, &'static str> {
        let mut mod_string = String::new();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                return Err("Could not load config file")
            }
        };
        file.read_to_string(&mut mod_string)
        .unwrap_or_else(|e| panic!("Unrecoverable error while reading config: {}", e));

        let mut t_parser = toml::Parser::new(&mod_string);
        let tml = t_parser.parse();

        if tml.is_none() {
            for err in &t_parser.errors {
                let (loline, locol) = t_parser.to_linecol(err.lo);
                let (hiline, hicol) = t_parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         path, loline, locol, hiline, hicol, err.desc);
            }
            panic!("Unrecoverable error while parsing module config");


        }
        let module_opts = toml::Value::Table(tml.unwrap());

        Ok(toml::decode(module_opts).unwrap())
    }
}

#[derive(Debug, RustcDecodable)]
pub struct MetaOpts {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String
}

#[derive(Debug, RustcDecodable)]
pub struct OptionsOpts {
    pub strip_whitespace: Option<bool>,
    pub core: Option<String>
}

pub struct Module<'a> {
    path: &'a Path, // Sub paths are determined relative to this one
    options: Option<ModuleOpts>
}

impl<'b> Module<'b> {
    pub fn new(path:&'b Path) -> Module {
        Module {
            path: path,
            options: None
        }
    }

    pub fn get_opts(&'b mut self) -> &'b ModuleOpts {
        match self.options {
            Some(ref opts) => opts,
            None => {
                self.options = Some(ModuleOpts::load(self.path.to_str().unwrap().to_string()).unwrap());
                self.get_opts()
            }
        }
    }
}
