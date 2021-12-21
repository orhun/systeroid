use std::env;
use std::io;
use std::process;
use systeroid::args::Args;

fn main() {
    if let Some(args) = Args::parse(env::args().collect()) {
        let mut stdout = io::stdout();
        match systeroid::run(args, &mut stdout) {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    }
}
