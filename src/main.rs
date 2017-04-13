use std::io;
use std::fs::File;
use std::collections::HashSet;

extern crate syn;
extern crate clap;

use clap::{Arg, App};

mod config;
mod rust_lib;
mod bindgen;

use config::Config;

fn main() {
    let matches = App::new("cbindgen")
                    .version("0.1.0")
                    .about("Generate C bindings for a Rust library")
                    .arg(Arg::with_name("config")
                         .short("c")
                         .long("config")
                         .value_name("CONFIG")
                         .help("the config to use. currently either `wr`, or `default`"))
                    .arg(Arg::with_name("INPUT")
                         .help("the crate or source file to generate bindings for")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("OUTPUT")
                         .help("the path to output the directories to")
                         .required(false)
                         .index(2))
                    .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let config = match matches.value_of("config") {
        Some(c) => Config::load(c).expect("unknown config"),
        None => Config::default(),
    };

    let lib = bindgen::Library::load(input,
                                     &config,
                                     vec![],
                                     HashSet::new());

    let built = lib.build(&config).unwrap();

    match matches.value_of("OUTPUT") {
        Some(file) => {
            built.write(&config, &mut File::create(file).unwrap());
        }
        _ => {
            built.write(&config, &mut io::stdout());
        }
    }
}
