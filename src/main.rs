#[macro_use] extern crate log;
extern crate docopt;
extern crate env_logger;
extern crate rustc_serialize;
extern crate toml;

use docopt::Docopt;
use std::fs;
use std::io::prelude::*;
use std::path;
use std::collections;

// TODO: add a detailed description of how environments work.
/// The usage string
const USAGE: &'static str = "
zDocker.

Execute docker-compose comands within a configured environment.

Usage:
    zdocker [options] [--env ENV...] [--] <args>...

Options:
    -h --help           Show this screen.
    --version           Show version.
    -e --env=<env>      Configured environment.
    -c --config=<conf>  The config file to use [default: ./.zdocker.toml].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_args: Vec<String>,
    flag_config: String,
    flag_env: Vec<String>
}

/// Execute docker-compose with configured environment flags
fn main() {
    env_logger::init().unwrap();
    // parse args
    let args: Args = get_args();

    // print args for now
    trace!("received arguments: {:?}", args);

    // load config file (if it exists)
    let config_path = path::Path::new(&args.flag_config);
    debug!("loading zdocker config ({})...", config_path.display());
    let config = read_config(config_path);

    // parse the config string
    let config = parse_config(&config);

    // assemble environment flags

    // execute docker-compose with environment flags

    // TODO: delete this
    debug!("config: {:?}", config);
}

/// Parse command line arguments
fn get_args() -> Args {
    Docopt::new(USAGE)
            .and_then(|d| d.decode())
            .unwrap_or_else(|e| e.exit())
}

/// Read the config file into a string
fn read_config(config_path: &path::Path) -> String {
    // open the config file
    fs::File::open(config_path)
    // read the contents into a string
    .and_then(|mut f| {
        trace!("found zdocker config ({})", config_path.display());
        let mut s = String::new();
        f.read_to_string(&mut s)
         .map(|_| {
             trace!("loaded zdocker config ({})", config_path.display());
             s
         })
    })
    // otherwise, log the error and return an empty string
    .unwrap_or_else(|e| {
        debug!("could not load zdocker config: {}", e);
        String::new()
    })
}

/// Parse the config file from a string to a TOML table
fn parse_config(config: &str) -> toml::Table {
    let mut config_parser = toml::Parser::new(config);
    config_parser.parse()
    .unwrap_or_else(|| {
        // log parsing errors
        debug!("could not parse zdocker config:");
        debug!("BEGIN PARSER_ERRORS");
        config_parser.errors.clone()
            .into_iter()
            .map(|e| debug!("{}", e))
            .last();

        debug!("END PARSER_ERRORS");
        // return empty table
        collections::BTreeMap::new()
    })
}
