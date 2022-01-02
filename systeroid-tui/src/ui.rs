use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

/// Renders the user interface.
pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App) {
    let rect = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(rect.height - 3), Constraint::Min(3)].as_ref())
        .split(rect);

    if let Some(input) = &app.input {
        frame.set_cursor(input.width() as u16 + 2, rect.height - 2);
    }
    frame.render_widget(
        Paragraph::new(match &app.input {
            Some(input) => format!(":{}", input),
            None => String::new(),
        })
        .block(
            Block::default()
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        ),
        chunks[1],
    );
}
