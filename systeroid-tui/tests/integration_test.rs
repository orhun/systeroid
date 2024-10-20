use insta::assert_snapshot;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use systeroid_core::config::Config;
use systeroid_core::sysctl::controller::Sysctl;
use systeroid_core::sysctl::parameter::Parameter;
use systeroid_core::sysctl::section::Section;
use systeroid_tui::app::App;
use systeroid_tui::command::Command;
use systeroid_tui::error::Result;
use systeroid_tui::options::{Direction, ScrollArea};
use systeroid_tui::style::Colors;
use systeroid_tui::ui::render;

#[test]
fn test_render_tui() -> Result<()> {
    let mut sysctl = Sysctl {
        parameters: vec![
            Parameter {
                name: String::from("user.name"),
                value: String::from("system"),
                description: None,
                section: Section::User,
                docs_path: PathBuf::new(),
                docs_title: String::new(),
            },
            Parameter {
                name: String::from("kernel.fictional.test_param"),
                value: String::from("0"),
                description: Some(String::from("This is a fictional parameter for testing")),
                section: Section::Kernel,
                docs_path: PathBuf::from("/etc/cosmos"),
                docs_title: String::from("Test Parameter"),
            },
            Parameter {
                name: String::from("vm.stat_interval"),
                value: String::from("1"),
                description: Some(String::from(
                    "The time interval between which vm statistics are updated",
                )),
                section: Section::Vm,
                docs_path: PathBuf::from("/usr/share/doc/linux/admin-guide/sysctl/vm.rst"),
                docs_title: String::from("stat_interval"),
            },
        ],
        config: Config::default(),
    };
    let mut app = App::new(&mut sysctl);
    let colors = Colors::default();
    let backend = TestBackend::new(40, 10);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("default", terminal.backend());

    app.run_command(Command::Help)?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Down, 1))?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Up, 1))?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("about", terminal.backend());
    app.run_command(Command::Select)?;

    app.run_command(Command::Select)?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("set", terminal.backend());
    assert!(app.is_input_mode());

    app.run_command(Command::ClearInput(false))?;
    app.run_command(Command::MoveCursor(Direction::Left))?;
    app.run_command(Command::ClearInput(true))?;
    app.run_command(Command::ClearInput(true))?;
    "kill"
        .chars()
        .try_for_each(|c| app.run_command(Command::UpdateInput(c)))?;

    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("input", terminal.backend());

    app.run_command(Command::ProcessInput)?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("process_input", terminal.backend());

    thread::sleep(Duration::from_millis(2000));
    app.tick();
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("go_back", terminal.backend());

    app.run_command(Command::Search)?;
    app.run_command(Command::Cancel)?;
    app.run_command(Command::Copy)?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("copy", terminal.backend());

    app.run_command(Command::Scroll(ScrollArea::List, Direction::Down, 1))?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Up, 1))?;
    app.run_command(Command::Select)?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("copy_not_enabled", terminal.backend());

    thread::sleep(Duration::from_millis(2000));
    app.tick();
    app.run_command(Command::Scroll(ScrollArea::Section, Direction::Left, 1))?;
    app.run_command(Command::Scroll(ScrollArea::Section, Direction::Left, 1))?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("list", terminal.backend());
    app.run_command(Command::Scroll(ScrollArea::Section, Direction::Right, 1))?;
    app.run_command(Command::Scroll(ScrollArea::Section, Direction::Right, 1))?;

    app.input = Some(String::new());
    app.run_command(Command::Search)?;
    app.run_command(Command::UpdateInput('_'))?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("search", terminal.backend());

    app.run_command(Command::ProcessInput)?;
    app.run_command(Command::Scroll(ScrollArea::Documentation, Direction::Up, 1))?;
    app.run_command(Command::Scroll(
        ScrollArea::Documentation,
        Direction::Down,
        5,
    ))?;
    app.run_command(Command::Scroll(ScrollArea::Documentation, Direction::Up, 1))?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("documentation", terminal.backend());

    app.run_command(Command::Scroll(ScrollArea::List, Direction::Bottom, 1))?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Up, 1))?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Up, 2))?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Top, 1))?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Down, 1))?;
    app.run_command(Command::Scroll(ScrollArea::List, Direction::Down, 2))?;
    app.run_command(Command::Refresh)?;
    terminal.draw(|frame| render(frame, &mut app, &colors))?;
    assert_snapshot!("refreshed", terminal.backend());

    app.run_command(Command::Nothing)?;
    app.run_command(Command::Exit)?;
    assert!(!app.running);

    Ok(())
}
