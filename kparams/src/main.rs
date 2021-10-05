use std::process;

fn main() {
    match kparams::run() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
