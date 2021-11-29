// main.rs

use std::env::args;
use std::process;
use pg::PG;
use colored::*;

fn main() {
    // gets the arguments of the command
    let args: Vec<String> = args().skip(1).collect(); // skip 1 because the first arg is always the program name
    let mut pg = match PG::new(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", format!("{}: {}", "Error".red(), e));
            process::exit(1);
        }
    };

    if let Err(e) = pg.generate_password() { // generate password and see IF it's returns an error
        eprintln!("{}", format!("{}: {}", "Error".red(), e));
        process::exit(1);
    }
}
