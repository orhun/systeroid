use crate::app::{App, KeyBinding, HELP_TEXT};
use crate::style::Colors;
use crate::widgets::SelectableList;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::{Span, Text};
use tui::widgets::{Block, BorderType, Borders, Cell, Clear, Paragraph, Row, Table, Wrap};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

/// Renders the user interface.
pub fn render<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App, colors: &Colors) {
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
        render_parameter_list(frame, chunks[0], app, colors);
        if app.input.is_some() {
            render_input_prompt(frame, chunks[1], rect.height - 2, app, colors);
        }
    }
    if let Some(documentation) = documentation {
        render_parameter_documentation(
            frame,
            chunks[1],
            documentation,
            &mut app.docs_scroll_amount,
            colors,
        );
    }
    if app.show_help {
        render_help_text(frame, rect, &mut app.key_bindings, colors);
    }
}

/// Renders the list that contains the sysctl parameters.
fn render_parameter_list<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    app: &mut App,
    colors: &Colors,
) {
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
        let value = item.value.replace('\t', " ");
        Row::new(if minimize_rows {
            vec![Cell::from(Span::styled(
                format!("{} = {}", item.name, value),
                colors.get_fg_style(),
            ))]
        } else {
            vec![
                Cell::from(Span::styled(item.name.clone(), colors.get_fg_style())),
                Cell::from(Span::styled(value, colors.get_fg_style())),
            ]
        })
        .height(1)
        .bottom_margin(0)
    });
    frame.render_stateful_widget(
        Table::new(rows)
            .block(
                Block::default()
                    .title(Span::styled("Parameters", colors.get_fg_style()))
                    .title_alignment(Alignment::Left)
                    .borders(Borders::all())
                    .border_style(colors.get_fg_style())
                    .border_type(BorderType::Rounded)
                    .style(colors.get_bg_style()),
            )
            .highlight_style(colors.get_style())
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
        colors,
    );
    if let Some(section) = app.section_list.selected() {
        render_section_text(frame, rect, section, colors);
    }
    if let Some(options) = app.options.as_mut() {
        render_options_menu(frame, rect, options, colors);
    }
}

/// Renders the text for displaying the selected index.
fn render_selection_text<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    selection_text: String,
    colors: &Colors,
) {
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
                    .style(colors.get_style()),
            ),
            horizontal_area[1],
        );
        frame.render_widget(Clear, horizontal_area[2]);
        frame.render_widget(
            Paragraph::new(Text::default()).block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(colors.get_bg_style()),
            ),
            horizontal_area[2],
        );
    }
}

/// Renders the text for displaying the parameter section.
fn render_section_text<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    section: &str,
    colors: &Colors,
) {
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
                        .checked_sub(text_width + 3)
                        .unwrap_or(rect.height),
                ),
                Constraint::Length(text_width),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(vertical_area[0]);
    frame.render_widget(Clear, area[1]);
    frame.render_widget(
        Paragraph::new(Span::styled(section, colors.get_fg_style())).block(
            Block::default()
                .borders(Borders::NONE)
                .style(colors.get_bg_style()),
        ),
        area[1],
    );
}

/// Renders the text for displaying help.
fn render_help_text<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    key_bindings: &mut SelectableList<&KeyBinding>,
    colors: &Colors,
) {
    let (percent_x, percent_y) = (50, 50);
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(rect);
    let rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(
                    (HELP_TEXT.lines().count() + 2)
                        .try_into()
                        .unwrap_or_default(),
                ),
                Constraint::Min(
                    (key_bindings.items.len() + 2)
                        .try_into()
                        .unwrap_or_default(),
                ),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(rect);
    frame.render_widget(Clear, area[0]);
    frame.render_widget(
        Paragraph::new(Text::styled(HELP_TEXT, colors.get_fg_style()))
            .block(
                Block::default()
                    .title(Span::styled("About", colors.get_fg_style()))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all())
                    .border_style(colors.get_fg_style())
                    .border_type(BorderType::Rounded)
                    .style(colors.get_bg_style()),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false }),
        area[0],
    );
    frame.render_widget(Clear, area[1]);
    frame.render_stateful_widget(
        Table::new(key_bindings.items.iter().map(|item| {
            Row::new(vec![
                Cell::from(Span::styled(item.key, colors.get_fg_style())),
                Cell::from(Span::styled(item.action, colors.get_fg_style())),
            ])
            .height(1)
            .bottom_margin(0)
        }))
        .block(
            Block::default()
                .title(Span::styled("Key Bindings", colors.get_fg_style()))
                .title_alignment(Alignment::Center)
                .borders(Borders::all())
                .border_style(colors.get_fg_style())
                .border_type(BorderType::Rounded)
                .style(colors.get_bg_style()),
        )
        .highlight_style(colors.get_style())
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]),
        area[1],
        &mut key_bindings.state,
    );
}

/// Renders a list as a popup for showing options.
fn render_options_menu<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    options: &mut SelectableList<&str>,
    colors: &Colors,
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
            Row::new(vec![Cell::from(Span::styled(
                item.to_string(),
                colors.get_fg_style(),
            ))])
            .height(1)
            .bottom_margin(0)
        }))
        .block(
            Block::default()
                .title(Span::styled("Copy to clipboard", colors.get_fg_style()))
                .title_alignment(Alignment::Center)
                .borders(Borders::all())
                .border_style(colors.get_fg_style())
                .border_type(BorderType::Rounded)
                .style(colors.get_bg_style()),
        )
        .highlight_style(colors.get_style())
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
    colors: &Colors,
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
        Paragraph::new(Text::styled(documentation, colors.get_fg_style()))
            .block(
                Block::default()
                    .title(Span::styled("Documentation", colors.get_fg_style()))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all())
                    .border_style(colors.get_fg_style())
                    .border_type(BorderType::Rounded)
                    .style(colors.get_bg_style()),
            )
            .scroll((*scroll_amount, 0))
            .wrap(Wrap { trim: false }),
        rect,
    );
}

/// Renders the input prompt for running commands.
fn render_input_prompt<B: Backend>(
    frame: &mut Frame<'_, B>,
    rect: Rect,
    cursor_y: u16,
    app: &App,
    colors: &Colors,
) {
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
        Paragraph::new(Span::styled(text, colors.get_fg_style())).block(
            Block::default()
                .borders(Borders::all())
                .border_style(colors.get_fg_style())
                .border_type(BorderType::Rounded)
                .style(colors.get_bg_style()),
        ),
        rect,
    );
}
