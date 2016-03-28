extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const MOKA_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE : &'static str = "
Compile or run a Moka module

Usage:
    moka [-va] use <module> <input> <output>
    moka [-va] compile <module> <output>
    moka -h
    moka --version

Options:
    -a, --archive   The specified module is an archive instead of a folder
    -h, --help      Show this text
    -v, --verbose   Enable verbose output
    --version       Show the installed Moka version
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_archive: bool,
    flag_verbose: bool,
    flag_version: bool,
    flag_help: bool,
    cmd_use: bool,
    cmd_compile: bool,
    arg_module: Option<String>,
    arg_input: Option<String>,
    arg_output: Option<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|opts| opts.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("v{}", MOKA_VERSION);
        std::process::exit(0);
    }

    if args.flag_help {
        println!("{}", USAGE);
        std::process::exit(0);
    }
}
