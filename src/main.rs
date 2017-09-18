extern crate clap;
extern crate nqueens;

use clap::{App, Arg};

use std::error::Error;
use std::str::FromStr;
use std::process::exit;
use nqueens::board::NQueens;

const N_KEY: &'static str = "n";

fn main() {
    match inner_main() {
        Ok(_) => exit(0),
        Err(e) => {
            println!("Something went horribly wrong: {}", e);
            exit(1)
        }
    }
}

fn inner_main() -> Result<(), Box<Error>> {
    let version = version();
    let app = App::new("nqueens")
        .version(version.as_str())
        .author("Lloyd (github.com/lloydmeta)")
        .about(
            "Solves the nqueens problem",
        )
        .arg(
            Arg::with_name(N_KEY)
                .short("N")
                .long(N_KEY)
                .takes_value(true)
                .number_of_values(1)
                .required(true)
                .validator(|s| usize::from_str(&s[..]).map(|_| ()).map_err(|e| format!("{}", e)))
                .help(
                    "The number of queens and side length of the board yo uwant to solve for",
                ),
        );

    // in case we need to print help
    let mut app_clone = app.clone();
    let matches = app.get_matches();
    match matches.value_of(N_KEY) {
        Some(n) => {
            let n_queens = NQueens::new(usize::from_str(&n)?);
            let solutions = n_queens.solve();
            let rendered = solutions.render();
            for render in rendered {
                println!("{}", render);
            }
            Ok(())
        }
        _ => Ok(app_clone.print_help()?),
    }
}

/// Return the current crate version
fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}