use std::env;
use std::io;
use std::process;
use systeroid_tui::args::Args;

fn main() {
    if let Some(args) = Args::parse(env::args().collect()) {
        let mut stderr = io::stderr();
        match systeroid_tui::run(args, &mut stderr) {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    }
}
