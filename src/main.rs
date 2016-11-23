extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::path::Path;

const USAGE: &'static str = "
zDocker.

Usage:
    zdocker [options] <env> [--] <args>...

Options:
    -h --help           Show this screen.
    --version           Show version.
    -c --config=<conf>  The config file to use [default: ./.zdocker.toml].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_env: Vec<String>,
    arg_args: Vec<String>,
    flag_config: String,
}

fn main() {
    // parse args
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    // print args for now
    // TODO: delete this
    println!("{:?}", args);
}
