// Limit `error_chain!`s recursion depth
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;

extern crate reqwest;

use std::process;

mod utils;

fn run() -> Result<(), Box<::std::error::Error>> {
    let args = clap_app!(carddavcli =>
                         (version: crate_version!())
                         (@arg username: -u --user +required +takes_value "Username")
                         (@arg password: -p --pass +required +takes_value "Password")
                         (@arg verbose: -v --verbose "be verbose")
                         (@arg URL: +required +takes_value "CardDAV server URL")
    ).get_matches();

    let url = utils::parse_url(args.value_of("URL").unwrap())?;
    let uname = args.value_of("username");
    let pass = args.value_of("password");
    let verbose = args.is_present("verbose");

    match url.scheme() {
        "http" | "https" =>  Ok(()),
        _ => utils::gen_error(format!("unsupported url scheme '{}'", url.scheme())),
    }
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}
