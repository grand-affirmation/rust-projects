use std::env::args;
use std::process;
use pg::PG;
use colored::*;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let try_pg = PG::new(args);
    if let Err(e) = try_pg {
        eprintln!("{}", format!("{}: {}", "Error".red(), e));
        process::exit(1);
    }
    
    let mut pg = try_pg.unwrap();
    if let Err(e) = pg.generate_password() {
        eprintln!("{}", format!("{}: {}", "Error".red(), e));
        process::exit(1);
    }
}
