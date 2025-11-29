use ratatui::{
    crossterm::event::KeyCode,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use super::PopUp;

#[derive(Debug)]
pub struct InfoPopUp {
    title: String,
    info: String,
    offset: (u16, u16),
    max_offset: (u16, u16),
    dimensions: (u16, u16),
}

impl InfoPopUp {
    /// Generate a pop-up with a title and an arbitrary information.
    pub fn generate_info_popup(title: &str, info: &str) -> Box<dyn PopUp> {
        let mut lines = 0;
        let mut columns = 0;

        for line in info.lines() {
            lines += 1;
            let line_len = line.len() as u16;
            if line_len > columns {
                columns = line_len;
            }
        }

        let dimensions = (30, 50); // TODO: Calculate percentage based on the lines and screen size

        Box::new(InfoPopUp {
            title: title.to_string(),
            info: info.to_string(),
            offset: (0, 0),
            max_offset: (lines, columns),
            dimensions,
        })
    }
}

impl PopUp for InfoPopUp {
    fn dimensions(&self) -> (u16, u16) {
        self.dimensions
    }

    /// Renders a centered overlaying pop-up with a title and an arbitrary info.
    fn render(&self, f: &mut ratatui::Frame, chunk: Rect) {
        let bold_blue = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Blue);
        let block = Block::default()
            .title(self.title.clone())
            .title_alignment(Alignment::Center)
            .title_style(bold_blue)
            .title_bottom(Line::styled("(ESC / q) Close", bold_blue))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(Style::default());

        let pop_up = Paragraph::new(self.info.clone())
            .block(block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .scroll(self.offset);

        f.render_widget(Clear, chunk);
        f.render_widget(pop_up, chunk);
    }

    /// Handles simple one-char width navigation.
    fn handle(&mut self, key: ratatui::crossterm::event::KeyEvent) -> color_eyre::Result<()> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.offset.0 > 0 {
                    self.offset.0 -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.offset.0 < self.max_offset.0 {
                    self.offset.0 += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.offset.1 > 0 {
                    self.offset.1 -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.offset.1 < self.max_offset.1 {
                    self.offset.1 += 1;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn create_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_dimensions_from_constructor() {
        let popup = InfoPopUp::generate_info_popup("Title", "Info");
        assert_eq!(popup.dimensions(), (30, 50));
    }

    #[test]
    fn test_offset_calculation_logic() {
        let info = "Line 1\nLine 2 is longer\nLine 3";

        let mut lines = 0;
        let mut columns = 0;
        for line in info.lines() {
            lines += 1;
            let line_len = line.len() as u16;
            if line_len > columns {
                columns = line_len;
            }
        }

        assert_eq!(lines, 3);
        assert_eq!(columns, 16);
    }

    #[test]
    fn test_handle_vertical_scrolling() {
        let mut popup = InfoPopUp {
            title: "Test".to_string(),
            info: "Line 1\nLine 2\nLine 3".to_string(),
            offset: (0, 0),
            max_offset: (2, 10),
            dimensions: (30, 50),
        };

        popup.handle(create_key_event(KeyCode::Char('j'))).unwrap();
        assert_eq!(popup.offset.0, 1);

        popup.handle(create_key_event(KeyCode::Down)).unwrap();
        assert_eq!(popup.offset.0, 2);

        popup.handle(create_key_event(KeyCode::Down)).unwrap();
        assert_eq!(popup.offset.0, 2);

        popup.handle(create_key_event(KeyCode::Char('k'))).unwrap();
        assert_eq!(popup.offset.0, 1);

        popup.handle(create_key_event(KeyCode::Up)).unwrap();
        assert_eq!(popup.offset.0, 0);

        popup.handle(create_key_event(KeyCode::Up)).unwrap();
        assert_eq!(popup.offset.0, 0);
    }

    #[test]
    fn test_handle_horizontal_scrolling() {
        let mut popup = InfoPopUp {
            title: "Test".to_string(),
            info: "Long Line".to_string(),
            offset: (0, 0),
            max_offset: (5, 5),
            dimensions: (30, 50),
        };

        popup.handle(create_key_event(KeyCode::Char('l'))).unwrap();
        assert_eq!(popup.offset.1, 1);

        popup.handle(create_key_event(KeyCode::Right)).unwrap();
        assert_eq!(popup.offset.1, 2);

        popup.handle(create_key_event(KeyCode::Char('h'))).unwrap();
        assert_eq!(popup.offset.1, 1);

        popup.handle(create_key_event(KeyCode::Left)).unwrap();
        assert_eq!(popup.offset.1, 0);
    }

    #[test]
    fn test_handle_ignores_unrelated_keys() {
        let mut popup = InfoPopUp {
            title: "Test".to_string(),
            info: "Info".to_string(),
            offset: (0, 0),
            max_offset: (10, 10),
            dimensions: (30, 50),
        };

        popup.handle(create_key_event(KeyCode::Char('z'))).unwrap();
        assert_eq!(popup.offset, (0, 0));

        popup.handle(create_key_event(KeyCode::Enter)).unwrap();
        assert_eq!(popup.offset, (0, 0));
    }
}
