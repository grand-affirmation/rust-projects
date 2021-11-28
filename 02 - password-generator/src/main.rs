use std::env::args;
use std::process;
use pg::PG;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let pg = PG::new(args);
    if let Err(e) = pg {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
