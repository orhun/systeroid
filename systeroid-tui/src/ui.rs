use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

/// Renders the user interface.
pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App) {
    let rect = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(rect);
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(rect.height - 3), Constraint::Min(3)].as_ref())
            .split(chunks[0]);
        render_variable_list(frame, chunks[0], app);
        render_input_prompt(frame, chunks[1], rect.height - 2, app);
    }
}

/// Renders the list that contains the sysctl variables.
fn render_variable_list<B: Backend>(frame: &mut Frame<'_, B>, rect: Rect, app: &mut App) {
    let max_width = app
        .variable_list
        .items
        .iter()
        .map(|p| p.name.len())
        .max_by(|x, y| x.cmp(y))
        .and_then(|v| u16::try_from(v).ok())
        .unwrap_or(1);
    let minimize_rows = rect.width < max_width + 10;
    let rows = app.variable_list.items.iter().map(|item| {
        Row::new(if minimize_rows {
            vec![Cell::from(format!("{} = {}", item.name, item.value))]
        } else {
            vec![
                Cell::from(item.name.clone()),
                Cell::from(item.value.clone()),
            ]
        })
        .height(1)
        .bottom_margin(1)
    });
    frame.render_stateful_widget(
        Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::all())
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black)),
            )
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
            .widths(&if minimize_rows {
                [Constraint::Percentage(100), Constraint::Min(0)]
            } else {
                [Constraint::Min(max_width), Constraint::Percentage(100)]
            }),
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
    let text = match &app.input {
        Some(input) => format!(
            "{}{}",
            if app.input_time.is_some() {
                "MSG: "
            } else {
                frame.set_cursor(input.width() as u16 + 2, cursor_y);
                ":"
            },
            input,
        ),
        None => String::new(),
    };
    frame.render_widget(
        Paragraph::new(text).block(
            Block::default()
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        ),
        rect,
    );
}
