extern crate walkdir;

use walkdir::WalkDir;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::Read;
use std::path::Path;

fn main() {
    let target = env::var("PROFILE").unwrap();
    let out_dir = Path::new(".").join("target").join(&target);
    let res_dir = Path::new(".").join("resources");
    let mut write_opts = fs::OpenOptions::new();
    write_opts.write(true).create(true);
    println!("{:?}", out_dir);
    for entry in WalkDir::new(res_dir) {
        let e = match entry {
            Ok(ref ent) => ent,
            _ => continue
        };
        let target_dir = out_dir.join(e.path());
        if e.file_type().is_file() {
            println!("Target output: {:?}", target_dir);
            fs::create_dir_all(&target_dir.parent().unwrap());
            let mut outfile = write_opts.open(&target_dir).unwrap_or_else(|e| panic!(e));
            let contents = match cat(e.path()) {
                Ok(val) => val,
                _ => continue
            };
            match outfile.write(contents.as_bytes()) {
                Err(e) => println!("File Err {:?}", e),
                _ => continue
            }
        } else {
            match fs::create_dir_all(e.path()) {
                Err(e) => println!("Dir Err {:?}", e),
                _ => continue
            }
        }
        println!("{:?}", target_dir);
    }
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
