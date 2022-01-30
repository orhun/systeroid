use crate::app::App;
use crate::widgets::SelectableList;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Wrap};
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
        render_parameter_documentation(
            frame,
            chunks[1],
            documentation,
            &mut app.docs_scroll_amount,
        );
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
        .bottom_margin(0)
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
    if let Some(section) = app.section_list.selected() {
        render_section_text(frame, rect, section);
    }
    if let Some(options) = app.options.as_mut() {
        render_options_menu(frame, rect, options);
    }
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
                    .style(Style::default().bg(Color::White).fg(Color::Black)),
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

/// Renders the text for displaying the parameter section.
fn render_section_text<B: Backend>(frame: &mut Frame<'_, B>, rect: Rect, section: &str) {
    let section = format!("|{}|", section);
    let text_width: u16 = section.width().try_into().unwrap_or(1);
    let vertical_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Min(rect.height.checked_sub(1).unwrap_or(rect.height)),
            ]
            .as_ref(),
        )
        .split(rect);
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Min(
                    rect.width
                        .checked_sub(text_width + 2)
                        .unwrap_or(rect.height),
                ),
                Constraint::Min(text_width),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(vertical_area[0]);
    frame.render_widget(Clear, area[1]);
    frame.render_widget(
        Paragraph::new(Span::styled(section, Style::default().fg(Color::White))).block(
            Block::default()
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::Black)),
        ),
        area[1],
    );
}

/// Renders a list as a popup for showing options.
fn render_options_menu<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    options: &mut SelectableList<&str>,
) {
    let (length_x, length_y) = (
        25,
        u16::try_from(options.items.len()).unwrap_or_default() + 2,
    );
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length((rect.height.checked_sub(length_y)).unwrap_or_default() / 2),
                Constraint::Min(length_y),
                Constraint::Length((rect.height.checked_sub(length_y)).unwrap_or_default() / 2),
            ]
            .as_ref(),
        )
        .split(rect);
    let rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(
                    (popup_layout[1].width.checked_sub(length_x)).unwrap_or_default() / 2,
                ),
                Constraint::Min(length_x),
                Constraint::Length(
                    (popup_layout[1].width.checked_sub(length_x)).unwrap_or_default() / 2,
                ),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];
    frame.render_widget(Clear, rect);
    frame.render_stateful_widget(
        Table::new(options.items.iter().map(|item| {
            Row::new(vec![Cell::from(item.to_string())])
                .height(1)
                .bottom_margin(0)
        }))
        .block(
            Block::default()
                .title(Span::styled(
                    "Copy to clipboard",
                    Style::default().fg(Color::White),
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        )
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .widths(&[Constraint::Percentage(100)]),
        rect,
        &mut options.state,
    );
}

/// Renders the documentation of the selected sysctl parameter.
fn render_parameter_documentation<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    documentation: String,
    scroll_amount: &mut u16,
) {
    match (documentation.lines().count() * 2).checked_sub(rect.height.into()) {
        Some(scroll_overflow) => {
            if scroll_overflow < (*scroll_amount).into() {
                *scroll_amount = scroll_overflow as u16;
            }
        }
        None => {
            *scroll_amount = scroll_amount.checked_sub(1).unwrap_or_default();
        }
    }
    frame.render_widget(
        Paragraph::new(documentation)
            .block(
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
            )
            .scroll((*scroll_amount, 0))
            .wrap(Wrap { trim: false }),
        rect,
    );
}

/// Renders the input prompt for running commands.
fn render_input_prompt<B: Backend>(frame: &mut Frame<'_, B>, rect: Rect, cursor_y: u16, app: &App) {
    let text = match app.input.clone() {
        Some(mut input) => {
            if app.input_time.is_some() {
                format!("MSG: {}", input)
            } else {
                let mut skip_chars = 0;
                if let Some(width_overflow) = (input.width() as u16 + 4)
                    .checked_sub(app.input_cursor)
                    .and_then(|v| v.checked_sub(rect.width))
                {
                    skip_chars = width_overflow;
                    input.replace_range(skip_chars as usize..(skip_chars + 1) as usize, "\u{2026}");
                    if let Some(cursor_x_end) = rect.width.checked_sub(2) {
                        frame.set_cursor(cursor_x_end, cursor_y);
                    }
                } else {
                    let area_width = (rect.width - 4) as usize;
                    if input.width() > area_width {
                        input.replace_range(area_width..(area_width + 1), "\u{2026}");
                    }
                    let cursor_x = input.width() as u16 - app.input_cursor + 2;
                    frame.set_cursor(cursor_x, cursor_y);
                }
                format!(
                    "{}{}",
                    if app.search_mode { "/" } else { ":" },
                    input.chars().skip(skip_chars.into()).collect::<String>(),
                )
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use systeroid_core::config::Config;
    use systeroid_core::sysctl::controller::Sysctl;
    use tui::backend::TestBackend;
    use tui::buffer::Buffer;
    use tui::Terminal;

    fn assert_buffer(mut buffer: Buffer, backend: &TestBackend) {
        assert_eq!(buffer.area, backend.size().unwrap());
        for x in 0..buffer.area().width {
            for y in 0..buffer.area().height {
                buffer
                    .get_mut(x, y)
                    .set_style(backend.buffer().get(x, y).style());
            }
        }
        backend.assert_buffer(&buffer);
    }

    #[test]
    fn test_render_ui() {
        let mut sysctl = Sysctl {
            parameters: Vec::new(),
            config: Config::default(),
        };
        let mut app = App::new(&mut sysctl);
        app.section_list.state.select(None);

        let backend = TestBackend::new(40, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| render(frame, &mut app)).unwrap();

        assert_buffer(
            Buffer::with_lines(vec![
                "╭Parameters────────────────────────────╮",
                "│                                      │",
                "│                                      │",
                "│                                      │",
                "│                                      │",
                "│                                      │",
                "│                                      │",
                "│                                      │",
                "│                                  1/0 │",
                "╰──────────────────────────────────────╯",
            ]),
            terminal.backend(),
        );
    }
}
