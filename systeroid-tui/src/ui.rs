use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

/// Renders the user interface.
pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App) {
    let rect = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(rect.height - 3), Constraint::Min(3)].as_ref())
        .split(rect);
    render_variable_list(frame, chunks[0], app);
    render_input_prompt(frame, chunks[1], rect.height - 2, app);
}

/// Renders the list that contains the sysctl variables.
fn render_variable_list<B: Backend>(frame: &mut Frame<'_, B>, rect: Rect, app: &mut App) {
    frame.render_stateful_widget(
        List::new(
            app.variable_list
                .items
                .iter()
                .map(|variable| {
                    ListItem::new(Span::raw(format!("{} = {}", variable.name, variable.value)))
                })
                .collect::<Vec<ListItem<'_>>>(),
        )
        .block(
            Block::default()
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        )
        .highlight_symbol("> "),
        rect,
        &mut app.variable_list.state,
    );
}

/// Renders the input prompt for running commands.
fn render_input_prompt<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    cursor_y: u16,
    app: &mut App,
) {
    if let Some(input) = &app.input {
        frame.set_cursor(input.width() as u16 + 2, cursor_y);
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
        rect,
    );
}
