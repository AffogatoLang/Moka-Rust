use std::process::Command;
use std::process;
use std::io;

pub fn run_file(path:&str, args:Vec<&str>) -> io::Result<process::Output> {
    let mut cmd = Command::new("python");
    cmd.arg(path);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.output()
}
