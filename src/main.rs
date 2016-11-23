extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
zDocker.

Usage:
    zdocker <env> <args>...

Options:
    -h --help   Show this screen.
    --version   Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_env: Vec<String>,
    arg_args: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
