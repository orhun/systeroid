//! A terminal user interface for managing kernel parameters.

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;
/// Command-line argument parser.
pub mod args;
/// Application commands.
pub mod command;
/// Error implementation.
pub mod error;
/// Event handling.
pub mod event;
/// Application options.
pub mod options;
/// Style helper.
pub mod style;
/// User interface renderer.
pub mod ui;
/// Custom widgets.
pub mod widgets;

use crate::app::App;
use crate::args::Args;
use crate::command::Command;
use crate::error::Result;
use crate::event::{Event, EventHandler};
use crate::style::Colors;
use command::LoggerCommand;
use log::LevelFilter;
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::env;
use std::str::FromStr;
use systeroid_core::cache::Cache;
use systeroid_core::config::Config;
use systeroid_core::sysctl::controller::Sysctl;

/// Runs `systeroid-tui`.
pub fn run<B: Backend>(args: Args, backend: B) -> Result<()> {
    let mut config = Config {
        display_deprecated: args.display_deprecated,
        kernel_docs: args.kernel_docs,
        ..Default::default()
    };
    config.tui.tick_rate = args.tick_rate;
    config.tui.save_path = args.save_path;
    config.tui.log_file = args.log_file;
    config.tui.no_docs = args.no_docs;
    config.tui.color.fg_color = args.fg_color;
    config.tui.color.bg_color = args.bg_color;
    config.parse(args.config)?;
    let colors = Colors::new(&config.tui.color.bg_color, &config.tui.color.fg_color)?;
    tui_logger::init_logger(if let Ok(log_level) = env::var("RUST_LOG") {
        LevelFilter::from_str(&log_level)?
    } else {
        LevelFilter::Trace
    })?;
    tui_logger::set_default_level(LevelFilter::Trace);
    if let Some(ref log_file) = config.tui.log_file {
        tui_logger::set_log_file(log_file)?;
    }
    log::trace!(target: "config", "{:?}", config);
    let mut sysctl = Sysctl::init(config)?;
    if !sysctl.config.tui.no_docs {
        sysctl.update_docs_from_cache(&Cache::init()?)?;
    }
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    let event_handler = EventHandler::new(sysctl.config.tui.tick_rate);
    let mut app = App::new(&mut sysctl);
    if let Some(section) = args.section {
        app.section_list.state.select(Some(
            app.section_list
                .items
                .iter()
                .position(|r| r == &section.to_string())
                .unwrap_or(0),
        ));
        app.search();
    }
    if args.search_query.is_some() {
        app.input = args.search_query;
        app.search();
        app.input = None;
    }
    while app.running {
        terminal.draw(|frame| ui::render(frame, &mut app, &colors))?;
        match event_handler.next()? {
            Event::KeyPress(key) => {
                let mut command = Command::parse(key, app.is_input_mode());
                if app.show_logs {
                    command = LoggerCommand::parse(key)
                        .map(Command::LoggerEvent)
                        .unwrap_or(command);
                }
                app.run_command(command)?;
            }
            #[cfg(not(test))]
            Event::Tick => {
                app.tick();
            }
            #[cfg(test)]
            Event::Tick => {
                app.running = false;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn test_systeroid_tui() -> Result<()> {
        let args = Args {
            tick_rate: 1000,
            fg_color: String::from("white"),
            bg_color: String::from("black"),
            ..Args::default()
        };
        let backend = TestBackend::new(40, 10);
        run(args, backend)?;
        Ok(())
    }
}
