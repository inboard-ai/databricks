use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{ChatEntry, FocusTarget, Model, Screen, Status};
use crate::component::chat::Focus as ChatFocus;
use crate::table::Table;
use databricks::sql;

const RED: Color = Color::Rgb(0xeb, 0x16, 0x00);
const RED_DIM: Color = Color::Rgb(0x80, 0x38, 0x30);
const DIM: Color = Color::Rgb(0x60, 0x60, 0x60);
const USER_BG: Color = Color::Rgb(0x2a, 0x2a, 0x2a);
const GREEN: Color = Color::Rgb(0x00, 0xaa, 0x00);
const YELLOW: Color = Color::Rgb(0xcc, 0xaa, 0x00);

const SPLASH: &[&str] = &[
    "░█░░█▀▀▄░█▀▄▀█░▄▀▀▄",
    "░█░░█▄▄█░█░▀░█░█▄▄█",
    "░▀▀░▀░░▀░▀░░░▀░█░░░",
];

/// Dim a color for unfocused entries
fn dim_color(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(r / 2, g / 2, b / 2),
        Color::White => Color::Rgb(0x60, 0x60, 0x60),
        Color::Gray => Color::Rgb(0x40, 0x40, 0x40),
        _ => color,
    }
}

fn splash_line(s: &str) -> Line<'static> {
    let spans: Vec<Span> = s
        .chars()
        .map(|c| {
            if c == '░' || c == '▒' {
                Span::styled(c.to_string(), Style::default().fg(RED_DIM))
            } else {
                Span::styled(c.to_string(), Style::default().fg(RED))
            }
        })
        .collect();
    Line::from(spans)
}

/// Get foreground color, possibly dimmed
fn fg(color: Color, dim: bool) -> Color {
    if dim { dim_color(color) } else { color }
}


fn animated_status_line(text: &str, tick: u8) -> Line<'static> {
    // Subtle grayscale shimmer centered around DIM (0x60)
    const GRAYS: [u8; 6] = [0x55, 0x60, 0x70, 0x80, 0x70, 0x60];

    let spans: Vec<Span> = text
        .chars()
        .enumerate()
        .map(|(i, c)| {
            // Subtract tick to spin the other direction, divide to slow down
            let idx = (i.wrapping_sub(tick as usize / 2)) % GRAYS.len();
            let gray = GRAYS[idx];
            Span::styled(
                c.to_string(),
                Style::default()
                    .fg(Color::Rgb(gray, gray, gray))
                    .add_modifier(Modifier::ITALIC),
            )
        })
        .collect();
    Line::from(spans)
}

pub fn view(frame: &mut Frame, model: &Model) -> u16 {
    match &model.screen {
        Screen::SelectWarehouse { warehouses, selected } => {
            draw_warehouse_select(frame, warehouses, *selected);
            0
        }
        Screen::SelectSpace { spaces, selected } => {
            draw_space_select(frame, spaces, *selected);
            0
        }
        Screen::Chat => draw_chat_screen(frame, model),
        Screen::QuitConfirm => {
            let max_scroll = draw_chat_screen(frame, model);
            draw_quit_confirm(frame);
            max_scroll
        }
    }
}

fn draw_quit_confirm(frame: &mut Frame) {
    let area = frame.area();

    // Center a small dialog
    let w = 30u16;
    let h = 5u16;
    let x = area.x + (area.width.saturating_sub(w)) / 2;
    let y = area.y + (area.height.saturating_sub(h)) / 2;
    let dialog = Rect::new(x, y, w, h);

    frame.render_widget(Clear, dialog);

    let text = Text::from(vec![
        Line::from(""),
        Line::from("  Exit? (Y/n)"),
        Line::from(""),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(DIM))
        .title(" Confirm ");

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, dialog);
}

fn draw_warehouse_select(frame: &mut Frame, warehouses: &[crate::app::Warehouse], selected: usize) {
    let area = frame.area();
    let mut lines: Vec<Line> = Vec::new();

    // Splash
    for line in SPLASH {
        lines.push(splash_line(line));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Select a warehouse:"));
    lines.push(Line::from(""));

    for (i, warehouse) in warehouses.iter().enumerate() {
        let is_selected = i == selected;
        let (dot_color, state_label) = match warehouse.state {
            sql::State::Running => (GREEN, "Running"),
            sql::State::Stopped => (DIM, "Stopped"),
            sql::State::Starting => (YELLOW, "Starting"),
            sql::State::Stopping => (YELLOW, "Stopping"),
            sql::State::Deleting => (DIM, "Deleting"),
            sql::State::Deleted => (DIM, "Deleted"),
        };

        let bg = if is_selected { USER_BG } else { Color::Reset };
        let fg = if is_selected { Color::White } else { Color::Gray };
        let prefix = if is_selected { "> " } else { "  " };

        lines.push(Line::from(vec![
            Span::styled(prefix, Style::default().fg(fg).bg(bg)),
            Span::styled(&warehouse.name, Style::default().fg(fg).bg(bg)),
            Span::styled(" ", Style::default().bg(bg)),
            Span::styled("●", Style::default().fg(dot_color).bg(bg)),
            Span::styled(format!(" {}", state_label), Style::default().fg(DIM).bg(bg)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑↓ to move, Enter to select",
        Style::default().fg(DIM),
    )));

    // Center the content
    let content_height = lines.len() as u16;
    let content_width = lines.iter().map(|l| l.width()).max().unwrap_or(40) as u16;
    let x = area.x + (area.width.saturating_sub(content_width)) / 2;
    let y = area.y + (area.height.saturating_sub(content_height)) / 2;
    let centered = Rect::new(x, y, content_width.min(area.width), content_height.min(area.height));

    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, centered);
}

fn draw_space_select(frame: &mut Frame, spaces: &[crate::app::Space], selected: usize) {
    let area = frame.area();
    let mut lines: Vec<Line> = Vec::new();

    // Splash
    for line in SPLASH {
        lines.push(splash_line(line));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Select a space:"));
    lines.push(Line::from(""));

    for (i, space) in spaces.iter().enumerate() {
        let is_selected = i == selected;
        let bg = if is_selected { USER_BG } else { Color::Reset };
        let fg = if is_selected { Color::White } else { Color::Gray };
        let prefix = if is_selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, space.title),
            Style::default().fg(fg).bg(bg),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑↓ to move, Enter to select",
        Style::default().fg(DIM),
    )));

    // Center the content
    let content_height = lines.len() as u16;
    let content_width = lines.iter().map(|l| l.width()).max().unwrap_or(40) as u16;
    let x = area.x + (area.width.saturating_sub(content_width)) / 2;
    let y = area.y + (area.height.saturating_sub(content_height)) / 2;
    let centered = Rect::new(x, y, content_width.min(area.width), content_height.min(area.height));

    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, centered);
}

fn draw_chat_screen(frame: &mut Frame, model: &Model) -> u16 {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Min(5),
        Constraint::Length(3),
    ])
    .split(area);

    let max_scroll = draw_chat(frame, model, chunks[0]);
    draw_input(frame, model, chunks[1]);

    // Suggestions: expanded popup or collapsed indicator
    if model.suggestion.is_expanded() {
        draw_suggestions(frame, model, chunks[0]);
    } else if model.suggestion.has_suggestions() {
        draw_suggestion_indicator(frame, model, chunks[0]);
    }

    max_scroll
}

fn draw_chat(frame: &mut Frame, model: &Model, area: Rect) -> u16 {
    // Two-column layout: indicator (2 chars) + content
    let chunks = Layout::horizontal([
        Constraint::Length(2),
        Constraint::Min(0),
    ])
    .split(area);

    let indicator_area = chunks[0];
    let content_area = chunks[1];
    let width = content_area.width.saturating_sub(1) as usize;
    let dim = model.suggestion.is_expanded();

    let focused_idx = match model.chat.focus {
        ChatFocus::Entry(idx) => Some(idx),
        ChatFocus::None => None,
    };

    // Build content lines and track entry start positions
    let mut lines: Vec<Line> = Vec::new();
    let mut entry_starts: Vec<usize> = Vec::new(); // line index where each entry starts

    // Header: Logo + space name
    draw_header_lines(&mut lines, model);

    for (i, entry) in model.chat.entries.iter().enumerate() {
        lines.push(Line::raw("")); // blank line before entry
        entry_starts.push(lines.len()); // mark where this entry's content starts

        match entry {
            ChatEntry::User(text) => {
                let bg = fg(USER_BG, dim);
                let text_fg = fg(Color::White, dim);
                let padded = if text.len() < width {
                    format!("{:w$}", text, w = width)
                } else {
                    text.clone()
                };
                lines.push(Line::styled(padded, Style::default().fg(text_fg).bg(bg)));
            }
            ChatEntry::Assistant(text) => {
                let text_fg = fg(Color::White, dim);
                for line in text.lines() {
                    lines.push(Line::styled(line.to_string(), Style::default().fg(text_fg)));
                }
            }
            ChatEntry::Table { sql, headers, rows, expanded } => {
                let border_fg = fg(DIM, dim);
                let header_fg = fg(Color::White, dim);
                let row_fg = fg(Color::Gray, dim);
                let hint_fg = fg(DIM, dim);

                // SQL expansion
                if *expanded {
                    if let Some(sql_text) = sql {
                        lines.push(Line::styled("SQL:", Style::default().fg(fg(DIM, dim))));
                        let sql_fg = fg(Color::White, dim);
                        for sql_line in sql_text.lines() {
                            lines.push(Line::styled(
                                format!("  {sql_line}"),
                                Style::default().fg(sql_fg),
                            ));
                        }
                        lines.push(Line::raw(""));
                    }
                }

                // Render table
                let table = Table::new(headers, rows);
                lines.extend(table.render(border_fg, header_fg, row_fg));

                // Hint
                let is_focused = focused_idx == Some(i);
                let hint = match (is_focused, *expanded, sql.is_some()) {
                    (true, true, _) => format!("{} rows · Space to hide SQL", table.row_count()),
                    (true, false, true) => format!("{} rows · Space to show SQL", table.row_count()),
                    _ => format!("{} rows", table.row_count()),
                };
                lines.push(Line::styled(hint, Style::default().fg(hint_fg)));
            }
            ChatEntry::Error(text) => {
                lines.push(Line::styled(
                    format!("Error: {text}"),
                    Style::default().fg(fg(RED, dim)),
                ));
            }
        }
    }

    match model.status {
        Status::Thinking => {
            lines.push(Line::raw(""));
            lines.push(animated_status_line("Thinking...", model.animation_tick));
        }
        Status::Running => {
            lines.push(Line::raw(""));
            lines.push(animated_status_line("Running...", model.animation_tick));
        }
        Status::Idle => {}
    }

    let content_height = lines.len() as u16;
    let visible_height = content_area.height;
    let max_scroll = content_height.saturating_sub(visible_height);
    let scroll = model.chat.scroll.min(max_scroll);

    // Render content
    let paragraph = Paragraph::new(Text::from(lines))
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, content_area);

    // Render indicator column for focused chat entry
    if let Some(idx) = focused_idx {
        if let Some(&start_line) = entry_starts.get(idx) {
            let indicator_y = start_line as u16;
            if indicator_y >= scroll && indicator_y < scroll + visible_height {
                let y = indicator_area.y + indicator_y - scroll;
                let indicator = Paragraph::new(Span::styled("⏺", Style::default().fg(RED)));
                frame.render_widget(
                    indicator,
                    Rect::new(indicator_area.x + 1, y, 1, 1),
                );
            }
        }
    }

    max_scroll
}

fn draw_header_lines(lines: &mut Vec<Line>, model: &Model) {
    let space_name = model.header.space_name.as_deref().unwrap_or("");
    let has_info = !space_name.is_empty();

    // Build logo with info on the right (3 logo lines + 1 spacer = 4 lines total)
    for (i, logo_line) in SPLASH.iter().enumerate() {
        let logo = splash_line(logo_line);
        let mut spans = logo.spans;

        if has_info {
            spans.push(Span::raw("  "));
            spans.push(Span::styled("│", Style::default().fg(DIM)));
            spans.push(Span::raw("  "));

            match i {
                0 => {
                    // Line 1: empty (aligns space name with middle of logo)
                }
                1 => {
                    // Line 2: space name
                    spans.push(Span::styled(space_name.to_string(), Style::default().fg(Color::White)));
                }
                2 => {
                    // Line 3: warehouse status (only if known)
                    if let Some(state) = &model.header.warehouse_state {
                        let (dot_color, label) = match state {
                            sql::State::Running => (GREEN, "Running"),
                            sql::State::Stopped => (DIM, "Stopped"),
                            sql::State::Starting => (YELLOW, "Starting"),
                            sql::State::Stopping => (YELLOW, "Stopping"),
                            sql::State::Deleting => (DIM, "Deleting"),
                            sql::State::Deleted => (DIM, "Deleted"),
                        };
                        spans.push(Span::styled("●", Style::default().fg(dot_color)));
                        spans.push(Span::styled(format!(" {}", label), Style::default().fg(DIM)));
                    }
                }
                _ => {}
            }
        }

        lines.push(Line::from(spans));
    }
    // Line 4: empty spacer
    lines.push(Line::raw(""));
}

fn draw_input(frame: &mut Frame, model: &Model, area: Rect) {
    // Split input area: input line + status line
    let chunks = Layout::vertical([
        Constraint::Length(2), // border + input
        Constraint::Length(1), // status line
    ])
    .split(area);

    let input_focused = model.focus == FocusTarget::Input;

    // Input line with focus indicator
    let (prefix, text_style) = if input_focused {
        (
            Span::styled("⏺ ", Style::default().fg(RED)),
            Style::default().fg(Color::White),
        )
    } else {
        (
            Span::styled("  ", Style::default()),
            Style::default().fg(DIM),
        )
    };

    let input_line = Line::from(vec![
        prefix,
        Span::styled(model.input.text.clone(), text_style),
    ]);

    let input = Paragraph::new(input_line)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(DIM)),
        );

    frame.render_widget(input, chunks[0]);

    // Status line with controls and suggestion indicator
    let mut status_spans: Vec<Span> = Vec::new();

    // Suggestion indicator (collapsed)
    if model.suggestion.has_suggestions() && !model.suggestion.is_expanded() {
        let indicator = format!(" {} suggestions (Tab) ", model.suggestion.items.len());
        status_spans.push(Span::styled(indicator, Style::default().fg(DIM)));
        status_spans.push(Span::styled(" ", Style::default()));
    }

    // Controls hint based on focus (keys highlighted, descriptions dim)
    if model.suggestion.is_expanded() {
        status_spans.push(Span::styled("↑↓", Style::default().fg(Color::White)));
        status_spans.push(Span::styled(" select · ", Style::default().fg(DIM)));
        status_spans.push(Span::styled("Enter", Style::default().fg(Color::White)));
        status_spans.push(Span::styled(" accept · ", Style::default().fg(DIM)));
        status_spans.push(Span::styled("Esc", Style::default().fg(Color::White)));
        status_spans.push(Span::styled(" close", Style::default().fg(DIM)));
    } else {
        match model.focus {
            FocusTarget::Input => {
                status_spans.push(Span::styled("↑", Style::default().fg(Color::White)));
                status_spans.push(Span::styled(" navigate · ", Style::default().fg(DIM)));
                status_spans.push(Span::styled("Tab", Style::default().fg(Color::White)));
                status_spans.push(Span::styled(" suggestions · ", Style::default().fg(DIM)));
                status_spans.push(Span::styled("/help", Style::default().fg(Color::White)));
            }
            FocusTarget::Chat => {
                status_spans.push(Span::styled("↑↓", Style::default().fg(Color::White)));
                status_spans.push(Span::styled(" navigate · ", Style::default().fg(DIM)));
                status_spans.push(Span::styled("Space", Style::default().fg(Color::White)));
                status_spans.push(Span::styled(" expand · ", Style::default().fg(DIM)));
                status_spans.push(Span::styled("Esc", Style::default().fg(Color::White)));
                status_spans.push(Span::styled(" return", Style::default().fg(DIM)));
            }
        }
    }

    let status_line = Line::from(status_spans);
    let status = Paragraph::new(status_line);
    frame.render_widget(status, chunks[1]);

    // Only show cursor when input is focused
    if input_focused {
        let cursor_x = chunks[0].x + 2 + model.input.cursor as u16;
        let cursor_y = chunks[0].y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}

fn draw_suggestion_indicator(frame: &mut Frame, model: &Model, chat_area: Rect) {
    let count = model.suggestion.items.len();
    let text = format!(" {} suggestions (Tab) ", count);
    let w = text.len() as u16 + 2;
    let h = 1;
    let x = chat_area.x + chat_area.width.saturating_sub(w + 1);
    let y = chat_area.y + chat_area.height.saturating_sub(2);

    let indicator = Paragraph::new(text)
        .style(Style::default().fg(RED).add_modifier(Modifier::DIM))
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(indicator, Rect::new(x, y, w, h));
}

fn draw_suggestions(frame: &mut Frame, model: &Model, chat_area: Rect) {
    let h = (model.suggestion.items.len() + 2).min(8) as u16;
    let w = model
        .suggestion
        .items
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(20)
        .min(60) as u16
        + 4;

    let x = chat_area.x + chat_area.width.saturating_sub(w + 1);
    let y = chat_area.y + chat_area.height.saturating_sub(h);
    let area = Rect::new(x, y, w, h);

    frame.render_widget(Clear, area);

    let items: Vec<ListItem> = model
        .suggestion
        .items
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if Some(i) == model.suggestion.selected {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(s.as_str()).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(DIM))
            .title(" ↑↓ Enter "),
    );

    frame.render_widget(list, area);
}
