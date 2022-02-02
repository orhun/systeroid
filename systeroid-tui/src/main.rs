use std::env;
use std::io;
use std::process;
use systeroid_tui::args::Args;
use systeroid_tui::error::Result;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;

fn main() -> Result<()> {
    if let Some(args) = Args::parse(env::args().collect()) {
        let output = io::stderr();
        let output = output.into_raw_mode()?;
        let output = MouseTerminal::from(output);
        let output = AlternateScreen::from(output);
        let backend = TermionBackend::new(output);
        match systeroid_tui::run(args, backend) {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    }
    Ok(())
}
