use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::ops::Add;
use std::collections::HashMap;
use toml;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct MetaOpts {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String
}

#[derive(Debug, Deserialize)]
pub struct OptionsOpts {
    pub strip_whitespace: Option<bool>,
    pub core: Option<String>
}

pub struct Module<'a> {
    path: &'a Path, // Sub paths are determined relative to this one
    options: Option<ModuleOpts>
}

fn sub_dir_of<'c>(path: &'c Path, name: &'c str) -> String {
        let mut path_buf : PathBuf = path.to_path_buf();
        path_buf.push(name);
        path_buf.into_os_string().into_string().unwrap()
}

impl<'b> Module<'b> {
    pub fn new(path:&'b Path) -> Result<Module, String> {
        let opts = ModuleOpts::load(sub_dir_of(path, "module.toml"))?;
        Ok(Module {
                path,
                options: Some(opts)
        })
    }

    pub fn sub_dir(&self, name: &'b str) -> String {
        let mut path_buf : PathBuf = self.path.to_path_buf();
        path_buf.push(name);
        path_buf.into_os_string().into_string().unwrap()
    }

    pub fn get_opts(&self) -> Result<&ModuleOpts, String> {
        match self.options {
            Some(ref opts) => Ok(opts),
            None => Err(String::from("Options has not been initialised yet"))
        }
    }
}
