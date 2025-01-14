use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Widget},
};

/// A widget to display a task progress.
///
/// Note: Gauge requires minimum height of 2, Gauge2 has been modified to work with 
/// a height of 1
///
/// # Examples:
///
/// ```
/// # use tui::widgets::{Widget, Gauge2, Block, Borders};
/// # use tui::style::{Style, Color, Modifier};
/// Gauge2::default()
///     .block(Block::default().borders(Borders::ALL).title("Progress"))
///     .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::ITALIC))
///     .percent(20);
/// ```
#[derive(Debug, Clone)]
pub struct Gauge2<'a> {
    block: Option<Block<'a>>,
    ratio: f64,
    label: Option<Span<'a>>,
    style: Style,
    gauge_style: Style,
}

impl<'a> Default for Gauge2<'a> {
    fn default() -> Gauge2<'a> {
        Gauge2 {
            block: None,
            ratio: 0.0,
            label: None,
            style: Style::default(),
            gauge_style: Style::default(),
        }
    }
}

impl<'a> Gauge2<'a> {
    pub fn block(mut self, block: Block<'a>) -> Gauge2<'a> {
        self.block = Some(block);
        self
    }

    pub fn percent(mut self, percent: u16) -> Gauge2<'a> {
        assert!(
            percent <= 100,
            "Percentage should be between 0 and 100 inclusively."
        );
        self.ratio = f64::from(percent) / 100.0;
        self
    }

    /// Sets ratio ([0.0, 1.0]) directly.
    pub fn ratio(mut self, ratio: f64) -> Gauge2<'a> {
        assert!(
            ratio <= 1.0 && ratio >= 0.0,
            "{}", format!("Ratio ({}) should be between 0 and 1 inclusively.", ratio).to_string()
        );
        self.ratio = ratio;
        self
    }

    pub fn label<T>(mut self, label: T) -> Gauge2<'a>
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: Style) -> Gauge2<'a> {
        self.style = style;
        self
    }

    pub fn gauge_style(mut self, style: Style) -> Gauge2<'a> {
        self.gauge_style = style;
        self
    }
}

impl<'a> Widget for Gauge2<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let gauge_area = match self.block.take() {
            Some(b) => {
							let mut inner_area = b.inner(area);
							if area.height == 1 {
								inner_area = area;
							}
							b.render(area, buf);
							inner_area
            }
            None => area,
        };
        buf.set_style(gauge_area, self.gauge_style);
        if gauge_area.height < 1 {
					return;
			}

				let mut center = gauge_area.height / 2 + gauge_area.top();
				if gauge_area.height < 1 {
					center = gauge_area.height + gauge_area.top();
				};

        let width = (f64::from(gauge_area.width) * self.ratio).round() as u16;
        let end = gauge_area.left() + width;
        // Label
        let ratio = self.ratio;
        let label = self
            .label
            .unwrap_or_else(|| Span::from(format!("{}%", (ratio * 100.0).round())));

				for y in gauge_area.top()..gauge_area.bottom() {
						// Gauge2
            for x in gauge_area.left()..end {
                buf.get_mut(x, y).set_symbol(" ");
            }

            if y == center {
                let label_width = label.width() as u16;
                let middle = (gauge_area.width - label_width) / 2 + gauge_area.left();
                buf.set_span(middle, y, &label, gauge_area.right() - middle);
            }

            // Fix colors
            for x in gauge_area.left()..end {
                buf.get_mut(x, y)
                    .set_fg(self.gauge_style.bg.unwrap_or(Color::Reset))
                    .set_bg(self.gauge_style.fg.unwrap_or(Color::Reset));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn gauge_invalid_percentage() {
        Gauge2::default().percent(110);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_upper_bound() {
        Gauge2::default().ratio(1.1);
    }

    #[test]
    #[should_panic]
    fn gauge_invalid_ratio_lower_bound() {
        Gauge2::default().ratio(-0.5);
    }
}
