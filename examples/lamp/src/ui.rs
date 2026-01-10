use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{ChatEntry, Model, Screen, Status};

const RED: Color = Color::Rgb(0xeb, 0x16, 0x00);
const RED_DIM: Color = Color::Rgb(0x80, 0x38, 0x30);
const DIM: Color = Color::Rgb(0x60, 0x60, 0x60);
const USER_BG: Color = Color::Rgb(0x2a, 0x2a, 0x2a);

const SPLASH: &[&str] = &[
    "░█░░█▀▀▄░█▀▄▀█░▄▀▀▄",
    "░█░░█▄▄█░█░▀░█░█▄▄█",
    "░▀▀░▀░░▀░▀░░░▀░█░░░",
];

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

fn draw_space_select(frame: &mut Frame, spaces: &[crate::app::Space], selected: usize) {
    let area = frame.area();
    let mut lines: Vec<Line> = Vec::new();

    // Splash
    for line in SPLASH {
        lines.push(splash_line(line));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Databricks Genie",
        Style::default().fg(DIM),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from("Select a space:"));
    lines.push(Line::from(""));

    for (i, space) in spaces.iter().enumerate() {
        let style = if i == selected {
            Style::default().fg(Color::White).bg(USER_BG)
        } else {
            Style::default().fg(Color::Gray)
        };
        let prefix = if i == selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, space.title),
            style,
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑↓ to move, Enter to select",
        Style::default().fg(DIM),
    )));

    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, area);
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

    if model.show_suggestions && !model.suggestions.is_empty() {
        draw_suggestions(frame, model, chunks[0]);
    }

    max_scroll
}

fn draw_chat(frame: &mut Frame, model: &Model, area: Rect) -> u16 {
    let mut lines: Vec<Line> = Vec::new();
    let width = area.width.saturating_sub(2) as usize;

    // Splash at top
    for line in SPLASH {
        lines.push(splash_line(line));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Databricks Genie",
        Style::default().fg(DIM),
    )));
    lines.push(Line::from(""));

    for entry in &model.chat {
        match entry {
            ChatEntry::User(text) => {
                lines.push(Line::from(""));
                let padded = format!("{} ", text);
                let display = if padded.len() < width {
                    format!("{:width$}", padded, width = width)
                } else {
                    padded
                };
                lines.push(Line::from(Span::styled(display, Style::default().bg(USER_BG))));
            }
            ChatEntry::Assistant(text) => {
                lines.push(Line::from(""));
                for line in text.lines() {
                    lines.push(Line::from(Span::raw(line.to_string())));
                }
            }
            ChatEntry::Sql(sql) => {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled("SQL:", Style::default().fg(DIM))));
                for line in sql.lines() {
                    lines.push(Line::from(Span::styled(
                        format!("  {line}"),
                        Style::default().fg(Color::White),
                    )));
                }
            }
            ChatEntry::Table { headers, rows } => {
                lines.push(Line::from(""));

                let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
                for row in rows {
                    for (i, cell) in row.iter().enumerate() {
                        if i < widths.len() {
                            widths[i] = widths[i].max(cell.len());
                        }
                    }
                }

                let header_line: String = headers
                    .iter()
                    .zip(&widths)
                    .map(|(h, w)| format!(" {h:w$} "))
                    .collect::<Vec<_>>()
                    .join("│");
                lines.push(Line::from(Span::styled(
                    format!("│{header_line}│"),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                )));

                let sep: String = widths
                    .iter()
                    .map(|w| "─".repeat(w + 2))
                    .collect::<Vec<_>>()
                    .join("┼");
                lines.push(Line::from(Span::styled(
                    format!("├{sep}┤"),
                    Style::default().fg(DIM),
                )));

                for row in rows {
                    let row_line: String = row
                        .iter()
                        .zip(&widths)
                        .map(|(cell, w)| format!(" {cell:w$} "))
                        .collect::<Vec<_>>()
                        .join("│");
                    lines.push(Line::from(Span::styled(
                        format!("│{row_line}│"),
                        Style::default().fg(Color::Gray),
                    )));
                }

                lines.push(Line::from(Span::styled(
                    format!("({} rows)", rows.len()),
                    Style::default().fg(DIM),
                )));
            }
            ChatEntry::Error(text) => {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("Error: {text}"),
                    Style::default().fg(RED),
                )));
            }
        }
    }

    match model.status {
        Status::Thinking => {
            lines.push(Line::from(""));
            lines.push(animated_status_line("Thinking...", model.animation_tick));
        }
        Status::Running => {
            lines.push(Line::from(""));
            lines.push(animated_status_line("Running...", model.animation_tick));
        }
        Status::Idle => {}
    }

    let content_height = lines.len() as u16;
    let visible_height = area.height;
    let max_scroll = content_height.saturating_sub(visible_height);

    let paragraph = Paragraph::new(Text::from(lines))
        .wrap(Wrap { trim: false })
        .scroll((model.scroll.min(max_scroll), 0));

    frame.render_widget(paragraph, area);

    max_scroll
}

fn draw_input(frame: &mut Frame, model: &Model, area: Rect) {
    // Split input area: input line + status line
    let chunks = Layout::vertical([
        Constraint::Length(2), // border + input
        Constraint::Length(1), // status line
    ])
    .split(area);

    let input_text = format!("> {}", model.input);

    let input = Paragraph::new(input_text.as_str())
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(DIM)),
        )
        .style(Style::default().fg(Color::White));

    frame.render_widget(input, chunks[0]);

    // Status line with controls and suggestion indicator
    let mut status_spans: Vec<Span> = Vec::new();

    // Suggestion indicator
    if !model.suggestions.is_empty() {
        let indicator = if model.show_suggestions {
            format!(" {} suggestions ", model.suggestions.len())
        } else {
            format!(" {} suggestions (↑) ", model.suggestions.len())
        };
        status_spans.push(Span::styled(indicator, Style::default().fg(RED)));
        status_spans.push(Span::styled(" ", Style::default()));
    }

    // Controls hint
    status_spans.push(Span::styled(
        "Enter send · Tab suggestions · Esc Esc quit · /help",
        Style::default().fg(DIM),
    ));

    let status_line = Line::from(status_spans);
    let status = Paragraph::new(status_line);
    frame.render_widget(status, chunks[1]);

    let cursor_x = chunks[0].x + 2 + model.cursor as u16;
    let cursor_y = chunks[0].y + 1;
    frame.set_cursor_position((cursor_x, cursor_y));
}

fn draw_suggestions(frame: &mut Frame, model: &Model, chat_area: Rect) {
    let h = (model.suggestions.len() + 2).min(8) as u16;
    let w = model
        .suggestions
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
        .suggestions
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if Some(i) == model.suggestion_idx {
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
