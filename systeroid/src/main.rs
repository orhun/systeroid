use std::env;
use std::process;
use systeroid::args::Args;

fn main() {
    if let Some(args) = Args::parse(env::args().collect()) {
        match systeroid::run(args) {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    }
}
