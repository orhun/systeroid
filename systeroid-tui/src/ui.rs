use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

/// Renders the user interface.
pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App) {
    let documentation = app
        .parameter_list
        .selected()
        .and_then(|parameter| parameter.get_documentation());
    let rect = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if documentation.is_some() {
            [Constraint::Percentage(50), Constraint::Percentage(50)]
        } else {
            [Constraint::Percentage(100), Constraint::Min(0)]
        })
        .split(rect);
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(if app.input.is_some() {
                [Constraint::Min(rect.height - 3), Constraint::Min(3)]
            } else {
                [Constraint::Percentage(100), Constraint::Min(0)]
            })
            .split(chunks[0]);
        render_parameter_list(frame, chunks[0], app);
        if app.input.is_some() {
            render_input_prompt(frame, chunks[1], rect.height - 2, app);
        }
    }
    if let Some(documentation) = documentation {
        render_parameter_documentation(frame, chunks[1], documentation);
    }
}

/// Renders the list that contains the sysctl parameters.
fn render_parameter_list<B: Backend>(frame: &mut Frame<'_, B>, rect: Rect, app: &mut App) {
    let max_width = app
        .parameter_list
        .items
        .iter()
        .map(|p| p.name.len())
        .max_by(|x, y| x.cmp(y))
        .and_then(|v| u16::try_from(v).ok())
        .unwrap_or(1);
    let minimize_rows = rect.width < max_width + 10;
    let rows = app.parameter_list.items.iter().map(|item| {
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
                    .title(Span::styled(
                        "Parameters",
                        Style::default().fg(Color::White),
                    ))
                    .title_alignment(Alignment::Left)
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
        &mut app.parameter_list.state,
    );
    render_selection_text(
        frame,
        rect,
        format!(
            "{}/{}",
            app.parameter_list
                .state
                .selected()
                .map(|v| v + 1)
                .unwrap_or(0),
            app.parameter_list.items.len()
        ),
    );
}

/// Renders the text for displaying the selected index.
fn render_selection_text<B: Backend>(frame: &mut Frame<'_, B>, rect: Rect, selection_text: String) {
    let selection_text_width = u16::try_from(selection_text.width()).unwrap_or_default();
    if let Some(horizontal_area_width) = rect.width.checked_sub(selection_text_width + 2) {
        let vertical_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(rect.height.checked_sub(2).unwrap_or(rect.height)),
                    Constraint::Min(1),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(rect);
        let horizontal_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Min(horizontal_area_width),
                    Constraint::Min(selection_text_width),
                    Constraint::Min(1),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(vertical_area[1]);
        frame.render_widget(Clear, horizontal_area[1]);
        frame.render_widget(
            Paragraph::new(selection_text).block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::Black)),
            ),
            horizontal_area[1],
        );
        frame.render_widget(Clear, horizontal_area[2]);
        frame.render_widget(
            Paragraph::new(Text::default()).block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::Black)),
            ),
            horizontal_area[2],
        );
    }
}

/// Renders the documentation of the selected sysctl parameter.
fn render_parameter_documentation<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    documentation: String,
) {
    frame.render_widget(
        Paragraph::new(documentation).block(
            Block::default()
                .title(Span::styled(
                    "Documentation",
                    Style::default().fg(Color::White),
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        ),
        rect,
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
