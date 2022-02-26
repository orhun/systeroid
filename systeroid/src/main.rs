use std::env;
use std::io;
use std::process::{self, Command};
use systeroid::args::Args;

fn main() {
    if let Some(args) = Args::parse(env::args().collect()) {
        if args.show_tui {
            let bin = format!("{}-tui", env!("CARGO_PKG_NAME"));
            match Command::new(&bin).spawn().map(|mut child| child.wait()) {
                Ok(_) => process::exit(0),
                Err(e) => {
                    eprintln!("Cannot run `{}` ({})", bin, e);
                    process::exit(1)
                }
            }
        } else {
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
}
