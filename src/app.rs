use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::str::FromStr;

use atty::{self, Stream};

use clap::{App as ClapApp, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};

use console::Term;

#[cfg(windows)]
use ansi_term;

use errors::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PagingMode {
    Always,
    QuitIfOneScreen,
    Never,
}

// #[derive(Clone)]
// pub struct Config<'a> {
//     /// The character width of the terminal
//     pub term_width: usize,

//     /// Pager or STDOUT
//     pub paging_mode: PagingMode,

//     /// User name
//     pub username: String,

//     /// Password
//     pub password: String,
// }

fn is_truecolor_terminal() -> bool {
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}

pub struct App {
    pub matches: ArgMatches<'static>,
    interactive_output: bool,
}

impl App {
    pub fn new() -> Self {
        #[cfg(windows)]
        let _ = ansi_term::enable_ansi_support();

        let interactive_output = atty::is(Stream::Stdout);

        App {
            matches: Self::matches(interactive_output),
            interactive_output,
        }
    }

    fn matches(interactive_output: bool) -> ArgMatches<'static> {
        let clap_color_setting = if interactive_output {
            AppSettings::ColoredHelp
        } else {
            AppSettings::ColorNever
        };

        ClapApp::new(crate_name!())
            .version(crate_version!())
            .global_setting(clap_color_setting)
            .max_term_width(90)
            .about(
                "A CLI based carddav client.\n\n\
                 Use '--help' instead of '-h' to see a more detailed version of the help text.",
            ).long_about("A CLI based carddav client.")
            .arg(
                Arg::with_name("URL")
                    .required(true)
                    .takes_value(true)
                    .help("URL of the carddav server.")
                    .long_help("URL of the carrdav server")
                    .multiple(false)
                    .empty_values(false),
            )
            .arg(
                Arg::with_name("user")
                    .required(true)
                    .takes_value(true)
                    .help("username")
                    .long_help("username for authentication")
                    .multiple(false)
                    .empty_values(false),
            )
            .arg(
                Arg::with_name("pass")
                    .required(true)
                    .takes_value(true)
                    .help("pasword")
                    .long_help("pasword for authentication")
                    .multiple(false)
                    .empty_values(false),
            ).subcommand(
            ).help_message("Print this help message")
            .version_message("Show version information")
            .get_matches()
    }
}
