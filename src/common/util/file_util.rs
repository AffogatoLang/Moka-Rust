use std::path::PathBuf;
use std::env;

pub fn get_bin_dir () -> PathBuf {
    let bin_dir_buf = env::current_exe();
    match bin_dir_buf {
        Ok(val) => (*val.parent().unwrap()).to_path_buf(), // Not possible for parent to not exist if bin is found, so unwrap is fine
        Err(_) => panic!("Cannot get bin dir, check your system config")
    }
}
