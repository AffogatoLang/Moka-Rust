use std::path::{Path, PathBuf};
use std::env;
use std::io::prelude::*;
use std::io;
use std::fs;
use common::util;
use walkdir::{WalkDir, DirEntry};

pub fn get_bin_dir () -> PathBuf {
    let bin_dir_buf = env::current_exe();
    match bin_dir_buf {
        Ok(val) => (*val.parent().unwrap()).to_path_buf(), // Not possible for parent to not exist if bin is found, so unwrap is fine
        Err(_) => panic!("Cannot get bin dir, check your system config")
    }
}

pub fn load_as_dir_or_file(path: &Path) -> io::Result<Vec<(String, String)>> {
    let meta : fs::Metadata = fs::metadata(path)?;
    if meta.is_dir() {
        read_as_dir(path)
    } else {
        read_as_file(path)
    }
}

fn read_as_dir(path: &Path) -> io::Result<Vec<(String, String)>> {
    let mut file_contents : Vec<(String, String)> = Vec::new();
    for entry in WalkDir::new(path) {
        let entry : DirEntry = entry.unwrap();
        let ent_path : &Path = entry.path();
        let ent_meta = entry.metadata()?;
        println!("{}", ent_path.display());
        if ent_meta.is_file() {
            let mut ent_file : fs::File = fs::File::open(ent_path)?;

            let buffer = &mut String::new();
            ent_file.read_to_string(buffer)?;

            file_contents.push((util::osstr_to_string(ent_path.file_stem().unwrap()).unwrap(), buffer.clone()));
        }
    }

    Ok(file_contents)
}

fn read_as_file(path: &Path) -> io::Result<Vec<(String, String)>> {
    let mut file : fs::File = fs::File::open(path)?;
    let buffer = &mut String::new();
    file.read_to_string(buffer)?;
    Ok(vec![(util::osstr_to_string(path.file_stem().unwrap()).unwrap(), buffer.clone())])
}
