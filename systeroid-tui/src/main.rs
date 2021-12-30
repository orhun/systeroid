use std::io;
use std::process;

fn main() {
    let mut stderr = io::stderr();
    match systeroid_tui::run(&mut stderr) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}
