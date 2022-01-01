use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

/// Renders the user interface.
pub fn render<B: Backend>(frame: &mut Frame<'_, B>) {
    let rect = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(rect.height - 3), Constraint::Min(3)].as_ref())
        .split(rect);
    frame.render_widget(
        Paragraph::new(env!("CARGO_PKG_NAME")).block(
            Block::default()
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        ),
        chunks[1],
    );
}
