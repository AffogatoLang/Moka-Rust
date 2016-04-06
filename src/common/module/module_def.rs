use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::ops::Add;
use std::collections::HashMap;
use toml;

#[derive(Debug, RustcDecodable)]
pub struct ModuleOpts {
    pub meta: MetaOpts,
    pub options: OptionsOpts,
    pub dependencies: Option<HashMap<String, String>>
}

impl ModuleOpts {
    pub fn load(path: String) -> Result<ModuleOpts, String> {
        let mut mod_string = String::new();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                return Err(String::from("Could not load config file"))
            }
        };
        let res = file.read_to_string(&mut mod_string);
        match res {
            Err(e) => return Err(format!("Unrecoverable error while reading config: {}", e)),
            Ok(_) => ()
        }

        let mut t_parser = toml::Parser::new(&mod_string);
        let tml = t_parser.parse();

        if tml.is_none() {
            let mut err_msg = String::new();
            for err in &t_parser.errors {
                let (loline, locol) = t_parser.to_linecol(err.lo);
                let (hiline, hicol) = t_parser.to_linecol(err.hi);
                let e_msg = format!("{}:{}:{}-{}:{} error: {}",
                         path, loline, locol, hiline, hicol, err.desc);
                err_msg = err_msg.add(&e_msg);
                err_msg = err_msg.add("\n");
            }
            return Err(err_msg)
        }
        let module_opts = toml::Value::Table(tml.unwrap());

        // TODO : check dependencies exist, complain if not

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

    pub fn get_opts(&'b mut self) -> Result<&'b ModuleOpts, String> {
        match self.options {
            Some(ref opts) => Ok(opts),
            None => {
                self.options = match ModuleOpts::load(self.path.to_str().unwrap().to_string()) {
                    Ok(opts) => Some(opts),
                    Err(e) => return Err(e)
                };
                self.get_opts()
            }
        }
    }
}
