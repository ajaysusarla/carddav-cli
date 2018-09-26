// Limit `error_chain!`s recursion depth
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;

extern crate ansi_term;
extern crate atty;
extern crate console;
extern crate syntect;

mod app;

use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::process;

use ansi_term::Colour::Green;
use ansi_term::Style;

use app::{App, Config};

mod errors {
    error_chain! {
        foreign_links {
            Clap(::clap::Error);
            Io(::std::io::Error);
            SyntectError(::syntect::LoadingError);
            ParseIntError(::std::num::ParseIntError);
        }
    }

    pub fn handle_error(error: &Error) {
        match error {
            &Error(ErrorKind::Io(ref io_error), _)
                if io_error.kind() == super::io::ErrorKind::BrokenPipe =>
            {
                super::process::exit(0);
            }
            _ => {
                use ansi_term::Colour::Red;
                eprintln!("{}: {}", Red.paint("[carddav-cli error]"), error);
            }
        };
    }
}

use errors::*;

fn run() -> Result<bool> {
}

fn main() {
    let result = run();

    match result {
        Err(error) => {
            handle_error(&error);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }

}
