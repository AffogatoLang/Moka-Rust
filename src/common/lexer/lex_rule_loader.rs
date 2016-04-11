use common::lexer::{Ruleset, LexRule};
use walkdir::{WalkDir, DirEntry};
use std::io::prelude::*;
use std::io;
use std::fs;

pub fn ruleset_from_dir (dir_path: String) -> io::Result<Ruleset> {
    ruleset_from_dir_v(dir_path, false)
}

pub fn ruleset_from_dir_v (dir_path: String, verbose: bool) -> io::Result<Ruleset> {
    let mut lex_list:Vec<DirEntry> = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        let ent_path = &entry.path();
        match ent_path.extension() {
            Some(ext) => {
                if ext == "lex" {
                    lex_list.push(entry.clone());
                }
            },
            None => ()
        }
    }

    let mut rules = Ruleset::new();

    for entry in lex_list {
        if verbose {
            println!("Loading rules from file [{}]", &entry.path().display());
        }
        let mut file = try!(fs::File::open(entry.path()));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        for line in contents.lines() {
            if ! ((line.trim() == "")  || (line.starts_with("#"))) {
                rules.add(LexRule::from_string(String::from(line)).unwrap());
            }
        }
    }
    Ok(rules.clone())
}
