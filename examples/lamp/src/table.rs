use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub struct Table {
    widths: Vec<usize>,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: &[String], rows: &[Vec<String>]) -> Self {
        let widths = Self::compute_widths(headers, rows);
        Self {
            widths,
            headers: headers.to_vec(),
            rows: rows.to_vec(),
        }
    }

    fn compute_widths(headers: &[String], rows: &[Vec<String>]) -> Vec<usize> {
        headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                rows.iter()
                    .filter_map(|r| r.get(i))
                    .map(|c| c.len())
                    .max()
                    .unwrap_or(0)
                    .max(h.len())
            })
            .collect()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Render table as styled lines
    pub fn render(&self, border_fg: Color, header_fg: Color, row_fg: Color) -> Vec<Line<'static>> {
        let mut lines = Vec::with_capacity(self.rows.len() + 4);
        lines.push(self.top_border(border_fg));
        lines.push(self.header_line(border_fg, header_fg));
        lines.push(self.separator(border_fg));
        lines.extend(self.rows.iter().map(|r| self.data_row(r, border_fg, row_fg)));
        lines.push(self.bottom_border(border_fg));
        lines
    }

    fn border_line(&self, left: &str, mid: &str, right: &str, fg: Color) -> Line<'static> {
        let inner = self
            .widths
            .iter()
            .map(|w| "─".repeat(w + 2))
            .collect::<Vec<_>>()
            .join(mid);
        Line::styled(format!("{left}{inner}{right}"), Style::default().fg(fg))
    }

    fn top_border(&self, fg: Color) -> Line<'static> {
        self.border_line("┌", "┬", "┐", fg)
    }

    fn separator(&self, fg: Color) -> Line<'static> {
        self.border_line("├", "┼", "┤", fg)
    }

    fn bottom_border(&self, fg: Color) -> Line<'static> {
        self.border_line("└", "┴", "┘", fg)
    }

    fn header_line(&self, border_fg: Color, header_fg: Color) -> Line<'static> {
        self.styled_row(&self.headers, border_fg, header_fg, true)
    }

    fn data_row(&self, cells: &[String], border_fg: Color, cell_fg: Color) -> Line<'static> {
        self.styled_row(cells, border_fg, cell_fg, false)
    }

    fn styled_row(
        &self,
        cells: &[String],
        border_fg: Color,
        cell_fg: Color,
        bold: bool,
    ) -> Line<'static> {
        // Build row as single string for consistent color rendering
        let inner = cells
            .iter()
            .zip(&self.widths)
            .map(|(cell, w)| format!(" {cell:w$} "))
            .collect::<Vec<_>>()
            .join("│");
        let row_text = format!("│{inner}│");

        // If bold or different cell color, we need spans; otherwise single styled line
        if bold || cell_fg != border_fg {
            let border_style = Style::default().fg(border_fg);
            let cell_style = if bold {
                Style::default().fg(cell_fg).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(cell_fg)
            };

            let mut spans = Vec::new();
            spans.push(Span::styled("│", border_style));
            for (i, (cell, w)) in cells.iter().zip(&self.widths).enumerate() {
                if i > 0 {
                    spans.push(Span::styled("│", border_style));
                }
                spans.push(Span::styled(format!(" {cell:w$} "), cell_style));
            }
            spans.push(Span::styled("│", border_style));
            Line::from(spans)
        } else {
            Line::styled(row_text, Style::default().fg(border_fg))
        }
    }
}
