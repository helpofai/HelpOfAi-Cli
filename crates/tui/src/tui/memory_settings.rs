//! Sub-TUI form popup configuration for persist-memory settings.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget},
};

use crate::config::Config;
use crate::palette;
use crate::tui::app::App;
use crate::tui::views::{ModalKind, ModalView, ViewAction, ViewEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusField {
    Enabled,
    MaxSize,
    Path,
    SaveButton,
}

pub struct MemorySettingsView {
    enabled: bool,
    max_size_input: String,
    path_input: String,
    focus: FocusField,
    error_message: Option<String>,
}

impl MemorySettingsView {
    pub fn new(app: &App) -> Self {
        let max_size = app.memory_max_size_kb;
        let (enabled, max_size_kb, custom_path) =
            match Config::load(app.config_path.clone(), app.config_profile.as_deref()) {
                Ok(cfg) => {
                    let enabled = cfg
                        .memory
                        .as_ref()
                        .and_then(|m| m.enabled)
                        .unwrap_or(app.use_memory);
                    let max_size = cfg
                        .memory
                        .as_ref()
                        .and_then(|m| m.max_size_kb)
                        .unwrap_or(max_size);
                    let path = cfg.memory_path.clone();
                    (enabled, max_size, path)
                }
                Err(_) => (app.use_memory, max_size, None),
            };

        Self {
            enabled,
            max_size_input: max_size_kb.to_string(),
            path_input: custom_path.unwrap_or_default(),
            focus: FocusField::Enabled,
            error_message: None,
        }
    }

    fn handle_backspace(&mut self) {
        match self.focus {
            FocusField::MaxSize => {
                self.max_size_input.pop();
            }
            FocusField::Path => {
                self.path_input.pop();
            }
            _ => {}
        }
    }

    fn handle_char(&mut self, c: char) {
        match self.focus {
            FocusField::MaxSize => {
                if c.is_ascii_digit() {
                    self.max_size_input.push(c);
                }
            }
            FocusField::Path => {
                self.path_input.push(c);
            }
            _ => {}
        }
    }

    fn focus_next(&mut self) {
        self.focus = match self.focus {
            FocusField::Enabled => FocusField::MaxSize,
            FocusField::MaxSize => FocusField::Path,
            FocusField::Path => FocusField::SaveButton,
            FocusField::SaveButton => FocusField::Enabled,
        };
    }

    fn focus_prev(&mut self) {
        self.focus = match self.focus {
            FocusField::Enabled => FocusField::SaveButton,
            FocusField::MaxSize => FocusField::Enabled,
            FocusField::Path => FocusField::MaxSize,
            FocusField::SaveButton => FocusField::Path,
        };
    }

    fn validate_and_submit(&mut self) -> ViewAction {
        self.error_message = None;
        let max_size = match self.max_size_input.trim().parse::<usize>() {
            Ok(val) if val >= 64 && val <= 4096 => val,
            _ => {
                self.error_message = Some("Max size must be between 64 and 4096 KiB".to_string());
                return ViewAction::None;
            }
        };

        let path = if self.path_input.trim().is_empty() {
            None
        } else {
            Some(self.path_input.trim().to_string())
        };

        ViewAction::EmitAndClose(ViewEvent::MemorySettingsApplied {
            enabled: self.enabled,
            max_size_kb: max_size,
            memory_path: path,
        })
    }
}

impl ModalView for MemorySettingsView {
    fn kind(&self) -> ModalKind {
        ModalKind::MemorySettings
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn handle_key(&mut self, key: KeyEvent) -> ViewAction {
        match key.code {
            KeyCode::Esc => ViewAction::Close,
            KeyCode::Tab => {
                self.focus_next();
                ViewAction::None
            }
            KeyCode::BackTab => {
                self.focus_prev();
                ViewAction::None
            }
            KeyCode::Up => {
                self.focus_prev();
                ViewAction::None
            }
            KeyCode::Down => {
                self.focus_next();
                ViewAction::None
            }
            KeyCode::Backspace => {
                self.handle_backspace();
                ViewAction::None
            }
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.handle_backspace();
                ViewAction::None
            }
            KeyCode::Char(' ') if self.focus == FocusField::Enabled => {
                self.enabled = !self.enabled;
                ViewAction::None
            }
            KeyCode::Enter => match self.focus {
                FocusField::Enabled => {
                    self.enabled = !self.enabled;
                    ViewAction::None
                }
                FocusField::SaveButton => self.validate_and_submit(),
                _ => {
                    self.focus_next();
                    ViewAction::None
                }
            },
            KeyCode::Char(c) => {
                self.handle_char(c);
                ViewAction::None
            }
            _ => ViewAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let popup_width = 72.min(area.width.saturating_sub(4)).max(46);
        let popup_height = 14.min(area.height.saturating_sub(4)).max(12);

        let popup_area = Rect {
            x: area.x + (area.width.saturating_sub(popup_width)) / 2,
            y: area.y + (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };

        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(Line::from(Span::styled(
                " Memory Settings ",
                Style::default()
                    .fg(palette::DEEPSEEK_SKY)
                    .add_modifier(Modifier::BOLD),
            )))
            .title_bottom(Line::from(vec![
                Span::styled(" Tab ", Style::default().fg(palette::TEXT_MUTED)),
                Span::raw("next "),
                Span::styled(" Shift+Tab ", Style::default().fg(palette::TEXT_MUTED)),
                Span::raw("prev "),
                Span::styled(" Enter ", Style::default().fg(palette::TEXT_MUTED)),
                Span::raw("save/toggle "),
                Span::styled(" Esc ", Style::default().fg(palette::TEXT_MUTED)),
                Span::raw("cancel "),
            ]))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette::BORDER_COLOR))
            .style(Style::default().bg(palette::DEEPSEEK_INK))
            .padding(Padding::uniform(1));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Enabled
                Constraint::Length(2), // Max Size
                Constraint::Length(2), // Path
                Constraint::Length(2), // Error / Tip
                Constraint::Length(1), // Save Button
            ])
            .split(inner);

        // --- 1. Enabled Toggle ---
        let enabled_style = if self.focus == FocusField::Enabled {
            Style::default()
                .fg(palette::SELECTION_TEXT)
                .bg(palette::SELECTION_BG)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette::TEXT_PRIMARY)
        };
        let checkbox = if self.enabled { "[x]" } else { "[ ]" };
        Paragraph::new(vec![Line::from(vec![
            Span::styled(format!("{checkbox} Enable User Memory"), enabled_style),
            Span::raw("  "),
            Span::styled(
                "(opt-in persistence context)",
                Style::default().fg(palette::TEXT_MUTED),
            ),
        ])])
        .render(chunks[0], buf);

        // --- 2. Max Size Input ---
        let size_style = if self.focus == FocusField::MaxSize {
            Style::default()
                .fg(palette::SELECTION_TEXT)
                .bg(palette::SELECTION_BG)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette::TEXT_PRIMARY)
        };
        let input_size_text = format!(" [ {} ]", self.max_size_input);
        Paragraph::new(vec![Line::from(vec![
            Span::raw("Max Size (64-4096 KiB): "),
            Span::styled(input_size_text, size_style),
        ])])
        .render(chunks[1], buf);

        // --- 3. Memory Path Input ---
        let path_style = if self.focus == FocusField::Path {
            Style::default()
                .fg(palette::SELECTION_TEXT)
                .bg(palette::SELECTION_BG)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette::TEXT_PRIMARY)
        };
        let input_path_text = if self.path_input.is_empty() {
            " [ (default: ~/.helpofai/memory.md) ]".to_string()
        } else {
            format!(" [ {} ]", self.path_input)
        };
        Paragraph::new(vec![Line::from(vec![
            Span::raw("Memory File Path:        "),
            Span::styled(input_path_text, path_style),
        ])])
        .render(chunks[2], buf);

        // --- 4. Error Message / Tip ---
        if let Some(ref err) = self.error_message {
            Paragraph::new(vec![Line::from(Span::styled(
                format!("Error: {err}"),
                Style::default().fg(palette::DEEPSEEK_RED),
            ))])
            .render(chunks[3], buf);
        } else {
            Paragraph::new(vec![Line::from(Span::styled(
                "Use arrow keys or Tab to navigate fields.",
                Style::default().fg(palette::TEXT_MUTED),
            ))])
            .render(chunks[3], buf);
        }

        // --- 5. Save Button ---
        let button_style = if self.focus == FocusField::SaveButton {
            Style::default()
                .fg(palette::SELECTION_TEXT)
                .bg(palette::SELECTION_BG)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette::TEXT_MUTED)
        };
        let button_text = " [ Save Settings ] ";
        Paragraph::new(vec![Line::from(Span::styled(button_text, button_style))])
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[4], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::tui::app::{App, TuiOptions};
    use tempfile::TempDir;

    fn create_test_app(tmpdir: &TempDir) -> App {
        let options = TuiOptions {
            model: "deepseek-v4-pro".to_string(),
            workspace: tmpdir.path().to_path_buf(),
            config_path: None,
            config_profile: None,
            allow_shell: false,
            use_alt_screen: true,
            use_mouse_capture: false,
            use_bracketed_paste: true,
            max_subagents: 1,
            skills_dir: tmpdir.path().join("skills"),
            memory_path: tmpdir.path().join("memory.md"),
            notes_path: tmpdir.path().join("notes.txt"),
            mcp_config_path: tmpdir.path().join("mcp.json"),
            use_memory: true,
            start_in_agent_mode: false,
            skip_onboarding: true,
            yolo: false,
            resume_session_id: None,
            initial_input: None,
        };
        App::new(options, &Config::default())
    }

    #[test]
    fn test_memory_settings_navigation() {
        let tmpdir = TempDir::new().expect("tempdir");
        let app = create_test_app(&tmpdir);
        let mut view = MemorySettingsView::new(&app);

        assert_eq!(view.focus, FocusField::Enabled);

        // Test Tab navigation
        view.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(view.focus, FocusField::MaxSize);

        view.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(view.focus, FocusField::Path);

        view.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(view.focus, FocusField::SaveButton);

        view.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(view.focus, FocusField::Enabled);

        // Test BackTab/Up navigation
        view.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(view.focus, FocusField::SaveButton);
    }

    #[test]
    fn test_memory_settings_toggle() {
        let tmpdir = TempDir::new().expect("tempdir");
        let app = create_test_app(&tmpdir);
        let mut view = MemorySettingsView::new(&app);

        // Toggle via space
        let original = view.enabled;
        view.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
        assert_ne!(view.enabled, original);
    }

    #[test]
    fn test_memory_settings_validation() {
        let tmpdir = TempDir::new().expect("tempdir");
        let app = create_test_app(&tmpdir);
        let mut view = MemorySettingsView::new(&app);

        // Set invalid size
        view.max_size_input = "invalid".to_string();
        let action = view.validate_and_submit();
        assert!(matches!(action, ViewAction::None));
        assert!(view.error_message.is_some());

        // Set out of range size
        view.max_size_input = "10".to_string();
        let action = view.validate_and_submit();
        assert!(matches!(action, ViewAction::None));
        assert!(view.error_message.is_some());

        // Set valid size
        view.max_size_input = "256".to_string();
        view.path_input = "custom_path.md".to_string();
        let action = view.validate_and_submit();
        match action {
            ViewAction::EmitAndClose(ViewEvent::MemorySettingsApplied {
                enabled,
                max_size_kb,
                memory_path,
            }) => {
                assert_eq!(enabled, view.enabled);
                assert_eq!(max_size_kb, 256);
                assert_eq!(memory_path, Some("custom_path.md".to_string()));
            }
            other => panic!("expected EmitAndClose, got {other:?}"),
        }
    }
}
