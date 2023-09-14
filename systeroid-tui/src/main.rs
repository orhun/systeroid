use ratatui::backend::TermionBackend;
use std::env;
use std::io::{self, Write};
use std::panic;
use std::process;
use systeroid_tui::args::Args;
use systeroid_tui::error::Result;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

fn main() -> Result<()> {
    if let Some(args) = Args::parse(env::args().collect()) {
        let output = io::stderr();
        let output = output.into_raw_mode()?;
        let output = MouseTerminal::from(output);
        let output = output.into_alternate_screen()?;
        let backend = TermionBackend::new(output);
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            let panic_cleanup = || -> Result<()> {
                let mut output = io::stderr();
                write!(
                    output,
                    "{}{}{}",
                    termion::clear::All,
                    termion::screen::ToMainScreen,
                    termion::cursor::Show
                )?;
                output.into_raw_mode()?.suspend_raw_mode()?;
                io::stderr().flush()?;
                Ok(())
            };
            panic_cleanup().expect("failed to clean up for panic");
            panic_hook(panic);
        }));
        match systeroid_tui::run(args, backend) {
            Ok(_) => process::exit(0),
            Err(e) => {
                eprintln!("{e}");
                process::exit(1)
            }
        }
    }
    Ok(())
}
