//! Multi-Terminal View
//!
//! A full-screen (or large) overlay that shows all background shell jobs
//! with a tabbed interface.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::palette;
use crate::tools::shell::{SharedShellManager, ShellStatus};
use crate::tui::views::{ModalKind, ModalView, ViewAction};

pub struct TerminalsView {
    active_tab: usize,
    shell_manager: Option<SharedShellManager>,
}

impl TerminalsView {
    #[must_use]
    pub fn new(shell_manager: Option<SharedShellManager>) -> Self {
        Self {
            active_tab: 0,
            shell_manager,
        }
    }
}

impl ModalView for TerminalsView {
    fn kind(&self) -> ModalKind {
        ModalKind::Terminals
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn handle_key(&mut self, key: KeyEvent) -> ViewAction {
        match key.code {
            KeyCode::Esc => ViewAction::Close,
            KeyCode::Char('t')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                ViewAction::Close
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.active_tab > 0 {
                    self.active_tab -= 1;
                }
                ViewAction::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.active_tab += 1;
                ViewAction::None
            }
            KeyCode::Char('w') | KeyCode::Char('n') => {
                let jobs = self
                    .shell_manager
                    .as_ref()
                    .map(|sm| sm.lock().unwrap().list_jobs())
                    .unwrap_or_default();
                let max_tab = jobs.len().saturating_sub(1);
                let safe_tab = self.active_tab.min(max_tab);
                if let Some(active_job) = jobs.get(safe_tab) {
                    let _ = crate::tools::shell::spawn_detached_terminal_window(
                        &active_job.command,
                        &active_job.cwd,
                    );
                } else {
                    let cwd =
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                    let _ = crate::tools::shell::spawn_detached_terminal_window("", &cwd);
                }
                ViewAction::None
            }
            _ => ViewAction::None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Overlay takes up most of the screen
        let popup_area = Rect {
            x: area.x + 2,
            y: area.y + 2,
            width: area.width.saturating_sub(4),
            height: area.height.saturating_sub(4),
        };

        // Clear background
        Clear.render(popup_area, buf);

        // Frame block
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette::SELECTION_BG))
            .title(vec![
                Span::styled(
                    " Multi-Terminal ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " (w: pop out new window, j/k: switch, Esc: close) ",
                    Style::default().fg(palette::TEXT_MUTED),
                ),
            ]);

        let inner_area = block.inner(popup_area);
        block.render(popup_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(25), Constraint::Min(0)])
            .split(inner_area);

        let sidebar_area = chunks[0];
        let content_area = chunks[1];

        let jobs = self
            .shell_manager
            .as_ref()
            .map(|sm| sm.lock().unwrap().list_jobs())
            .unwrap_or_default();

        let max_tab = jobs.len().saturating_sub(1);
        if self.active_tab > max_tab {
            // Because render requires &self (immutable), we cannot mutate self.active_tab here,
            // but we can adjust our local index used for rendering.
        }
        let safe_tab = self.active_tab.min(max_tab);

        let mut lines = Vec::new();
        if jobs.is_empty() {
            lines.push(Line::from(Span::styled(
                " No active jobs ",
                Style::default().fg(palette::TEXT_MUTED),
            )));
        } else {
            for (i, job) in jobs.iter().enumerate() {
                let style = if i == safe_tab {
                    Style::default()
                        .fg(palette::SELECTION_TEXT)
                        .bg(palette::SELECTION_BG)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(palette::TEXT_PRIMARY)
                };
                let status_indicator = if job.status == ShellStatus::Running {
                    "▶"
                } else {
                    "■"
                };
                // Keep the command short
                let cmd = if job.command.len() > 18 {
                    format!("{}…", &job.command[..17])
                } else {
                    job.command.clone()
                };
                lines.push(Line::from(Span::styled(
                    format!(" {status_indicator} {cmd} "),
                    style,
                )));
            }
        }

        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::RIGHT)
                    .border_style(Style::default().fg(palette::BORDER_COLOR)),
            )
            .render(sidebar_area, buf);

        // Render Active Job Output
        let content = if let Some(active_job) = jobs.get(safe_tab) {
            let mut out = String::new();
            if !active_job.stdout_tail.is_empty() {
                out.push_str(&active_job.stdout_tail);
            }
            if !active_job.stderr_tail.is_empty() {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&active_job.stderr_tail);
            }
            if out.is_empty() {
                "No output yet...".to_string()
            } else {
                out
            }
        } else {
            "Run a background shell command (like `cargo watch` or `python server.py`) to see it here.".to_string()
        };

        Paragraph::new(content)
            .block(Block::default())
            .render(content_area, buf);
    }
}
