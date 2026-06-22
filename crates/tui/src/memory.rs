//! User-level memory file.
//!
//! v0.8.8 ships an MVP that lets the user keep a persistent personal
//! note file the model sees on every turn:
//!
//! - **Load** `~/.helpofai/memory.md` (path is configurable via
//!   `memory_path` in `config.toml` and `DEEPSEEK_MEMORY_PATH` env),
//!   wrap it in a `<user_memory>` block, and prepend it to the system
//!   prompt alongside the existing `<project_instructions>` block.
//! - **`# foo`** typed in the composer appends `foo` to the memory
//!   file as a timestamped bullet — fast capture without leaving the TUI.
//! - **`/memory`** shows the resolved file path and current contents, and
//!   **`/memory edit`** prints a copy-pasteable `$VISUAL` / `$EDITOR`
//!   command for opening the file yourself.
//! - **`remember` tool** lets the model itself append a bullet when it
//!   notices a durable preference or convention worth keeping across
//!   sessions.
//!
//! Default behavior is **opt-in**: load + use the memory file only when
//! `[memory] enabled = true` in `config.toml` or `DEEPSEEK_MEMORY=on`.
//! That keeps existing users on zero-overhead behavior and makes the
//! feature explicit.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use chrono::Utc;

/// Read the user memory file at `path`, returning `None` when the file
/// doesn't exist or is empty after trimming.
#[must_use]
pub fn load(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    if content.trim().is_empty() {
        return None;
    }
    Some(content)
}

/// Wrap memory content in a `<user_memory>` block ready to prepend to the
/// system prompt. The `source` value is rendered verbatim into a
/// `source="…"` attribute — pass the path so the model can see where the
/// memory came from. Returns `None` for empty content.
#[must_use]
pub fn as_system_block(content: &str, source: &Path, max_size_kb: usize) -> Option<String> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return None;
    }

    let display = source.display().to_string();
    let max_size_bytes = max_size_kb * 1024;
    let payload = if content.len() > max_size_bytes {
        let cutoff = truncation_cutoff(content, &display, max_size_bytes);
        let omitted_bytes = content.len() - cutoff;
        let mut head = content[..cutoff].to_string();
        head.push_str(&truncation_marker(omitted_bytes, &display));
        head
    } else {
        trimmed.to_string()
    };

    Some(format!(
        "<user_memory source=\"{display}\">\n{payload}\n</user_memory>"
    ))
}

fn truncation_cutoff(content: &str, source: &str, max_size_bytes: usize) -> usize {
    let mut cutoff = previous_char_boundary(content, max_size_bytes);
    loop {
        let omitted_bytes = content.len() - cutoff;
        let max_head_len =
            max_size_bytes.saturating_sub(truncation_marker(omitted_bytes, source).len());
        let next_cutoff = previous_char_boundary(content, cutoff.min(max_head_len));
        if next_cutoff == cutoff {
            return cutoff;
        }
        cutoff = next_cutoff;
    }
}

fn truncation_marker(omitted_bytes: usize, source: &str) -> String {
    format!("\n<truncated bytes={omitted_bytes} source=\"{source}\">")
}

fn previous_char_boundary(value: &str, mut index: usize) -> usize {
    while !value.is_char_boundary(index) {
        index -= 1;
    }
    index
}

/// Compose the `<user_memory>` block for the system prompt, honouring the
/// opt-in toggle. Returns `None` when the feature is disabled or the file
/// is missing / empty so the caller doesn't have to check both conditions.
#[must_use]
pub fn compose_block(enabled: bool, path: &Path, max_size_kb: usize) -> Option<String> {
    if !enabled {
        return None;
    }
    let content = load(path)?;
    as_system_block(&content, path, max_size_kb)
}

/// Wrap project memory content in a `<project_memory>` block ready to prepend to the
/// system prompt. The `source` value is rendered verbatim into a
/// `source="…"` attribute — pass the path so the model can see where the
/// memory came from. Returns `None` for empty content.
#[must_use]
pub fn as_project_system_block(content: &str, source: &Path, max_size_kb: usize) -> Option<String> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return None;
    }

    let display = source.display().to_string();
    let max_size_bytes = max_size_kb * 1024;
    let payload = if content.len() > max_size_bytes {
        let cutoff = truncation_cutoff(content, &display, max_size_bytes);
        let omitted_bytes = content.len() - cutoff;
        let mut head = content[..cutoff].to_string();
        head.push_str(&truncation_marker(omitted_bytes, &display));
        head
    } else {
        trimmed.to_string()
    };

    Some(format!(
        "<project_memory source=\"{display}\">\n{payload}\n</project_memory>"
    ))
}

/// Compose the `<project_memory>` block for the system prompt, honouring the
/// opt-in toggle. Returns `None` when the feature is disabled or the file
/// is missing / empty.
#[must_use]
pub fn compose_project_block(enabled: bool, path: &Path, max_size_kb: usize) -> Option<String> {
    if !enabled {
        return None;
    }
    let content = load(path)?;
    as_project_system_block(&content, path, max_size_kb)
}

/// Helper function to append a line to a specific markdown heading section in the memory file.
/// If headings are missing, it initializes the file with the standard structure:
/// # User Memory
/// ## Preferences
/// ## Active Tasks
/// ## File Change Log
pub fn append_to_section(path: &Path, section: &str, line_to_append: &str) -> io::Result<()> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let has_preferences = lines.iter().any(|l| l.trim() == "## Preferences");
    let has_tasks = lines.iter().any(|l| l.trim() == "## Active Tasks");
    let has_changelog = lines.iter().any(|l| l.trim() == "## File Change Log");

    if !has_preferences || !has_tasks || !has_changelog {
        let mut new_lines = Vec::new();
        if lines
            .first()
            .map_or(true, |l| !l.starts_with("# User Memory"))
        {
            new_lines.push("# User Memory".to_string());
            new_lines.push("".to_string());
        }

        // Copy existing non-structural lines
        for l in &lines {
            let trimmed = l.trim();
            if trimmed != "# User Memory"
                && trimmed != "## Preferences"
                && trimmed != "## Active Tasks"
                && trimmed != "## File Change Log"
            {
                new_lines.push(l.clone());
            }
        }

        // Add headers at the end if they were missing, ensuring clean structure
        new_lines.push("## Preferences".to_string());
        new_lines.push("".to_string());
        new_lines.push("## Active Tasks".to_string());
        new_lines.push("".to_string());
        new_lines.push("## File Change Log".to_string());
        new_lines.push("".to_string());

        lines = new_lines;
    }

    // Find section start index
    let mut section_start = None;
    for (i, l) in lines.iter().enumerate() {
        if l.trim() == section {
            section_start = Some(i);
            break;
        }
    }

    if let Some(start) = section_start {
        // Find insert position at the end of the section (before next heading or EOF)
        let mut insert_at = lines.len();
        for j in (start + 1)..lines.len() {
            if lines[j].trim().starts_with("## ") {
                insert_at = j;
                break;
            }
        }

        // Back up to skip trailing empty lines before the next heading
        while insert_at > start + 1 && lines[insert_at - 1].trim().is_empty() {
            insert_at -= 1;
        }

        lines.insert(insert_at, line_to_append.to_string());

        // Ensure there is an empty line before next heading
        if insert_at + 1 < lines.len() && !lines[insert_at + 1].trim().is_empty() {
            lines.insert(insert_at + 1, "".to_string());
        }
    } else {
        lines.push(line_to_append.to_string());
    }

    let new_content = lines.join("\n") + "\n";

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, new_content)?;
    Ok(())
}

/// Append `entry` to the memory file under `## Preferences` (default section).
/// Keeps compatibility with legacy `# foo` quick-adds and the basic `remember` tool.
pub fn append_entry(path: &Path, entry: &str) -> io::Result<()> {
    append_preference(path, entry)
}

/// Append a user preference to memory under `## Preferences`.
pub fn append_preference(path: &Path, entry: &str) -> io::Result<()> {
    let trimmed = entry.trim_start_matches('#').trim();
    if trimmed.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "memory preference entry is empty",
        ));
    }

    let timestamp = Utc::now().format("%Y-%m-%d %H:%M UTC");
    let bullet = format!("- ({timestamp}) {trimmed}");
    append_to_section(path, "## Preferences", &bullet)
}

/// Append a task to memory under `## Active Tasks`.
pub fn append_task(path: &Path, task: &str) -> io::Result<()> {
    let trimmed = task.trim();
    if trimmed.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "memory task is empty",
        ));
    }

    let timestamp = Utc::now().format("%Y-%m-%d %H:%M UTC");
    let bullet = format!("- [ ] ({timestamp}) {trimmed}");
    append_to_section(path, "## Active Tasks", &bullet)
}

/// Append a file change entry to memory under `## File Change Log`.
pub fn append_file_change(
    path: &Path,
    file_path: &str,
    lines: Option<&str>,
    description: &str,
) -> io::Result<()> {
    let trimmed_desc = description.trim();
    if file_path.trim().is_empty() || trimmed_desc.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "file_path or description is empty",
        ));
    }

    let timestamp = Utc::now().format("%Y-%m-%d %H:%M UTC");
    let line_info = lines.map(|l| format!(":{}", l.trim())).unwrap_or_default();
    let bullet = format!("- ({timestamp}) [{file_path}{line_info}] {trimmed_desc}");
    append_to_section(path, "## File Change Log", &bullet)
}

/// Lists all tasks under the ## Active Tasks heading.
/// Returns a list of (completed, task_description) tuples.
pub fn list_tasks(path: &Path) -> io::Result<Vec<(bool, String)>> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let lines: Vec<&str> = content.lines().collect();

    let mut section_start = None;
    for (i, l) in lines.iter().enumerate() {
        if l.trim() == "## Active Tasks" {
            section_start = Some(i);
            break;
        }
    }

    let Some(start) = section_start else {
        return Ok(Vec::new());
    };

    let mut tasks = Vec::new();
    for j in (start + 1)..lines.len() {
        if lines[j].trim().starts_with("## ") {
            break;
        }
        let trimmed = lines[j].trim();
        if trimmed.starts_with("- [ ]") {
            tasks.push((
                false,
                trimmed
                    .strip_prefix("- [ ]")
                    .unwrap_or(trimmed)
                    .trim()
                    .to_string(),
            ));
        } else if trimmed.starts_with("- [x]") {
            tasks.push((
                true,
                trimmed
                    .strip_prefix("- [x]")
                    .unwrap_or(trimmed)
                    .trim()
                    .to_string(),
            ));
        }
    }

    Ok(tasks)
}

/// Marks task at task_index (1-indexed) as completed.
/// Returns the updated task line if successful, or None if index is out of bounds.
pub fn complete_task(path: &Path, task_index: usize) -> io::Result<Option<String>> {
    let content = fs::read_to_string(path)?;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut section_start = None;
    for (i, l) in lines.iter().enumerate() {
        if l.trim() == "## Active Tasks" {
            section_start = Some(i);
            break;
        }
    }

    let Some(start) = section_start else {
        return Ok(None);
    };

    let mut task_lines_indices = Vec::new();
    for j in (start + 1)..lines.len() {
        if lines[j].trim().starts_with("## ") {
            break;
        }
        if lines[j].trim().starts_with("- [ ]") || lines[j].trim().starts_with("- [x]") {
            task_lines_indices.push(j);
        }
    }

    if task_index == 0 || task_index > task_lines_indices.len() {
        return Ok(None);
    }

    let line_idx = task_lines_indices[task_index - 1];
    let original_line = lines[line_idx].clone();
    if original_line.trim().starts_with("- [ ]") {
        lines[line_idx] = original_line.replace("- [ ]", "- [x]");
        let new_content = lines.join("\n") + "\n";
        fs::write(path, new_content)?;
        Ok(Some(lines[line_idx].clone()))
    } else {
        Ok(Some(original_line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_returns_none_for_missing_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("never-existed.md");
        assert!(load(&path).is_none());
    }

    #[test]
    fn load_returns_none_for_whitespace_only_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("memory.md");
        fs::write(&path, "   \n   \n").unwrap();
        assert!(load(&path).is_none());
    }

    #[test]
    fn load_returns_content_for_real_file() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("memory.md");
        fs::write(&path, "remember the milk").unwrap();
        assert_eq!(load(&path).as_deref(), Some("remember the milk"));
    }

    #[test]
    fn as_system_block_produces_xml_wrapper() {
        let block = as_system_block("note 1", Path::new("/tmp/m.md"), 100).unwrap();
        assert!(block.contains("<user_memory source=\"/tmp/m.md\">"));
        assert!(block.contains("note 1"));
        assert!(block.ends_with("</user_memory>"));
    }

    #[test]
    fn as_system_block_returns_none_for_empty_content() {
        assert!(as_system_block("   ", Path::new("/tmp/m.md"), 100).is_none());
    }

    #[test]
    fn as_system_block_truncates_oversize_input() {
        let limit_kb = 10;
        let limit_bytes = limit_kb * 1024;
        let big = "x".repeat(limit_bytes + 100);
        let block = as_system_block(&big, Path::new("/tmp/m.md"), limit_kb).unwrap();
        let payload = user_memory_payload(&block);
        assert_eq!(payload.len(), limit_bytes);
        assert!(payload.ends_with("<truncated bytes=140 source=\"/tmp/m.md\">"));
    }

    fn user_memory_payload(block: &str) -> &str {
        block
            .strip_prefix("<user_memory source=\"/tmp/m.md\">\n")
            .unwrap()
            .strip_suffix("\n</user_memory>")
            .unwrap()
    }

    #[test]
    fn append_to_sections_places_content_correctly() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("memory.md");

        append_preference(&path, "Indentation is 4 spaces").unwrap();
        append_task(&path, "Implement search feature").unwrap();
        append_file_change(
            &path,
            "crates/tui/src/memory.rs",
            Some("12-25"),
            "Added search helper",
        )
        .unwrap();

        let body = fs::read_to_string(&path).unwrap();
        assert!(body.contains("## Preferences"));
        assert!(body.contains("Indentation is 4 spaces"));
        assert!(body.contains("## Active Tasks"));
        assert!(body.contains("Implement search feature"));
        assert!(body.contains("## File Change Log"));
        assert!(body.contains("[crates/tui/src/memory.rs:12-25] Added search helper"));
    }
}
