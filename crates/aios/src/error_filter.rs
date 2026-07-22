//! # AIOS Error Filter
//!
//! Parses raw command output (from cargo, gradle, go, tsc, python, etc.)
//! into a compact, structured list of errors. This is the "smart filter" that
//! sits between the raw build/test log and the AI context window.
//!
//! **Token budget:**
//! Full build log might be 50 000 chars (~12 500 tokens).
//! After filtering, the AI receives only `FilteredOutput` (~800–2 000 tokens).

use serde::{Deserialize, Serialize};

// ── Public types ─────────────────────────────────────────────────────────────

/// A single error (or fatal warning) extracted from command output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedError {
    /// Source file, if identifiable (e.g. `src/main.rs`, `app/MainActivity.kt`).
    pub file: Option<String>,
    /// Line number within the file, if present.
    pub line: Option<u32>,
    /// Column number, if present.
    pub col: Option<u32>,
    /// Error code such as `E0596`, `TS2345`, `NG0200`.
    pub code: Option<String>,
    /// The human-readable error message (trimmed).
    pub message: String,
    /// Which platform emitted this (`rust`, `android`, `typescript`, `python`,
    /// `go`, `java`, `generic`).
    pub platform: String,
}

/// The compact, AI-ready summary produced by the filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredOutput {
    /// Exit code of the original command (`None` if not available).
    pub exit_code: Option<i32>,
    /// One-line human summary, e.g. "Build FAILED: 3 errors, 14 warnings".
    pub summary: String,
    /// Deduplicated, capped list of parsed errors.
    pub errors: Vec<ParsedError>,
    /// Warning count (not individually included to save tokens).
    pub warning_count: usize,
    /// The last `TAIL_LINES` lines of the raw output — gives AI context for
    /// messages that didn't match any known pattern.
    pub raw_tail: String,
    /// Estimated token cost of this struct (rough: chars / 4).
    pub estimated_tokens: usize,
}

impl FilteredOutput {
    /// Format as a compact Markdown block suitable for AI context injection.
    /// Stays under ~1 000 tokens for typical outputs.
    pub fn to_ai_context(&self) -> String {
        let mut out = String::new();
        out.push_str("## AIOS Build/Test Report\n\n");
        out.push_str(&format!("**{}**\n", self.summary));
        if self.warning_count > 0 {
            out.push_str(&format!(
                "*{} warnings suppressed — only errors shown.*\n",
                self.warning_count
            ));
        }
        out.push('\n');

        if self.errors.is_empty() {
            out.push_str("No errors detected.\n");
        } else {
            out.push_str("### Errors\n\n");
            for (i, e) in self.errors.iter().enumerate() {
                let loc = match (&e.file, e.line) {
                    (Some(f), Some(l)) => format!("`{f}:{l}`"),
                    (Some(f), None) => format!("`{f}`"),
                    _ => "(unknown location)".to_string(),
                };
                let code = e
                    .code
                    .as_deref()
                    .map(|c| format!(" [{c}]"))
                    .unwrap_or_default();
                out.push_str(&format!("{}. **{}**{}: {}\n", i + 1, loc, code, e.message));
            }
        }

        if !self.raw_tail.is_empty() {
            out.push_str("\n### Output tail\n```\n");
            out.push_str(&self.raw_tail);
            out.push_str("\n```\n");
        }

        out.push_str(&format!(
            "\n*~{} tokens (AIOS filtered from full log)*\n",
            self.estimated_tokens
        ));
        out
    }
}

// ── Filter entry point ───────────────────────────────────────────────────────

/// Maximum errors to include (keeps AI context bounded).
const MAX_ERRORS: usize = 20;
/// Lines from the end of raw output to always include.
const TAIL_LINES: usize = 15;

/// Filter raw command output into a compact `FilteredOutput`.
///
/// `output`    — the full stdout+stderr combined text
/// `exit_code` — process exit code (`None` if unknown)
pub fn filter_output(output: &str, exit_code: Option<i32>) -> FilteredOutput {
    let lines: Vec<&str> = output.lines().collect();

    let mut errors: Vec<ParsedError> = Vec::new();
    let mut warning_count = 0usize;
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        // ── Rust / Cargo ──────────────────────────────────────────────────
        // Pattern: `error[E0596]: message`
        //           `  --> src/main.rs:42:5`
        if let Some(e) = parse_rust_error(line, lines.get(i + 1).copied()) {
            dedup_push(&mut errors, e, &mut seen, MAX_ERRORS);
            i += 2; // skip the --> line too
            continue;
        }

        // ── TypeScript / tsc ──────────────────────────────────────────────
        // Pattern: `src/app.ts(12,5): error TS2345: message`
        if let Some(e) = parse_tsc_error(line) {
            dedup_push(&mut errors, e, &mut seen, MAX_ERRORS);
            i += 1;
            continue;
        }

        // ── Android / Kotlin / Gradle ─────────────────────────────────────
        // Pattern: `e: file:///path/to/File.kt: (12, 5): message`
        // Pattern: `app/src/main/java/Foo.java:42: error: message`
        if let Some(e) = parse_android_error(line) {
            dedup_push(&mut errors, e, &mut seen, MAX_ERRORS);
            i += 1;
            continue;
        }

        // ── Go ────────────────────────────────────────────────────────────
        // Pattern: `./cmd/main.go:15:3: undefined: foo`
        if let Some(e) = parse_go_error(line) {
            dedup_push(&mut errors, e, &mut seen, MAX_ERRORS);
            i += 1;
            continue;
        }

        // ── Python ────────────────────────────────────────────────────────
        // Pattern: `  File "app.py", line 42`
        //          `SomeError: message`
        if let Some(e) = parse_python_error(line, lines.get(i + 1).copied()) {
            dedup_push(&mut errors, e, &mut seen, MAX_ERRORS);
            i += 2;
            continue;
        }

        // ── Generic ───────────────────────────────────────────────────────
        // Any line containing "error:" / "ERROR:" / "FAILED" not yet caught
        if let Some(e) = parse_generic_error(line) {
            dedup_push(&mut errors, e, &mut seen, MAX_ERRORS);
        } else if is_warning_line(line) {
            warning_count += 1;
        }

        i += 1;
    }

    // Tail: last N raw lines
    let tail_start = lines.len().saturating_sub(TAIL_LINES);
    let raw_tail = lines[tail_start..].join("\n");

    let error_count = errors.len();
    let success = exit_code.map(|c| c == 0).unwrap_or(false);
    let summary = if success && error_count == 0 {
        "Build/Test PASSED".to_string()
    } else {
        format!("FAILED — {error_count} error(s), {warning_count} warning(s)")
    };

    let serialized = serde_json::to_string(&errors).unwrap_or_default();
    let tail_chars = raw_tail.len();
    let estimated_tokens = (serialized.len() + tail_chars + summary.len()) / 4;

    FilteredOutput {
        exit_code,
        summary,
        errors,
        warning_count,
        raw_tail,
        estimated_tokens,
    }
}

// ── Platform parsers ─────────────────────────────────────────────────────────

fn parse_rust_error(line: &str, next_line: Option<&str>) -> Option<ParsedError> {
    // `error[E0596]: cannot borrow ...`
    // `error: ...`
    let trimmed = line.trim();
    let (code, msg_start) = if let Some(rest) = trimmed.strip_prefix("error[") {
        let bracket_end = rest.find(']')?;
        let code = rest[..bracket_end].to_string();
        let msg = rest[bracket_end + 2..].trim().to_string(); // skip ']: '
        (Some(code), msg)
    } else if let Some(rest) = trimmed.strip_prefix("error: ") {
        (None, rest.trim().to_string())
    } else {
        return None;
    };

    if msg_start.is_empty() {
        return None;
    }

    // Next line: `  --> src/main.rs:42:5`
    let (file, line_no, col_no) = next_line
        .and_then(|l| {
            let l = l.trim();
            let l = l.strip_prefix("-->")?.trim();
            parse_file_line_col(l)
        })
        .unwrap_or((None, None, None));

    Some(ParsedError {
        file,
        line: line_no,
        col: col_no,
        code,
        message: msg_start,
        platform: "rust".to_string(),
    })
}

fn parse_tsc_error(line: &str) -> Option<ParsedError> {
    // `src/app.ts(12,5): error TS2345: Argument ...`
    let paren = line.find('(')?;
    let file_part = &line[..paren];
    if !file_part.ends_with(".ts") && !file_part.ends_with(".tsx") && !file_part.ends_with(".js") {
        return None;
    }
    let rest = &line[paren + 1..];
    let close = rest.find(')')?;
    let loc = &rest[..close]; // "12,5"
    let (line_no, col_no) = loc
        .split_once(',')
        .and_then(|(l, c)| Some((l.parse::<u32>().ok()?, c.parse::<u32>().ok()?)))
        .unzip();
    let after = rest[close + 1..].trim();
    let after = after.strip_prefix(':')?.trim();
    if !after.starts_with("error") {
        return None;
    }
    let code = after
        .split_whitespace()
        .nth(1)
        .filter(|s| s.starts_with("TS"))
        .map(|s| s.trim_end_matches(':').to_string());
    let msg = after
        .splitn(3, ':')
        .nth(2)
        .unwrap_or(after)
        .trim()
        .to_string();
    Some(ParsedError {
        file: Some(file_part.to_string()),
        line: line_no,
        col: col_no,
        code,
        message: msg,
        platform: "typescript".to_string(),
    })
}

fn parse_android_error(line: &str) -> Option<ParsedError> {
    // Kotlin: `e: file:///abs/path/Foo.kt: (12, 5): error message`
    if let Some(rest) = line.trim().strip_prefix("e: file:///") {
        let colon = rest.find(": (")?;
        let file = rest[..colon].to_string();
        let rest = &rest[colon + 3..];
        let close = rest.find(')')?;
        let loc = &rest[..close];
        let (line_no, col_no) = loc
            .split_once(',')
            .and_then(|(l, c)| Some((l.trim().parse::<u32>().ok()?, c.trim().parse::<u32>().ok()?)))
            .unzip();
        let msg = rest[close + 1..].trim_start_matches([':', ' ']).to_string();
        return Some(ParsedError {
            file: Some(file),
            line: line_no,
            col: col_no,
            code: None,
            message: msg,
            platform: "android".to_string(),
        });
    }
    // Java: `app/src/Foo.java:42: error: message`
    if line.contains(".java:") && line.contains("error:") {
        let parts: Vec<&str> = line.splitn(3, ':').collect();
        if parts.len() >= 3 {
            let file = parts[0].trim().to_string();
            let line_no = parts[1].trim().parse::<u32>().ok();
            let msg = parts[2..]
                .join(":")
                .trim()
                .trim_start_matches("error:")
                .trim()
                .to_string();
            return Some(ParsedError {
                file: Some(file),
                line: line_no,
                col: None,
                code: None,
                message: msg,
                platform: "android".to_string(),
            });
        }
    }
    None
}

fn parse_go_error(line: &str) -> Option<ParsedError> {
    // `./cmd/main.go:15:3: undefined: foo`
    let trimmed = line.trim();
    if !trimmed.ends_with(".go") && !trimmed.contains(".go:") {
        return None;
    }
    // Must contain "error" or look like file:line:col: message
    let parts: Vec<&str> = trimmed.splitn(4, ':').collect();
    if parts.len() < 4 {
        return None;
    }
    let file = parts[0]
        .trim_start_matches('.')
        .trim_start_matches('/')
        .to_string();
    let line_no = parts[1].trim().parse::<u32>().ok();
    let col_no = parts[2].trim().parse::<u32>().ok();
    let msg = parts[3].trim().to_string();
    if msg.is_empty() {
        return None;
    }
    Some(ParsedError {
        file: Some(file),
        line: line_no,
        col: col_no,
        code: None,
        message: msg,
        platform: "go".to_string(),
    })
}

fn parse_python_error(line: &str, next_line: Option<&str>) -> Option<ParsedError> {
    // `  File "app.py", line 42`
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("File \"")?;
    let quote_end = rest.find('"')?;
    let file = rest[..quote_end].to_string();
    let after = &rest[quote_end + 1..];
    let line_no = after
        .split(", line ")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| s.parse::<u32>().ok());
    let msg = next_line
        .map(|l| l.trim().to_string())
        .unwrap_or_else(|| "(see traceback)".to_string());
    Some(ParsedError {
        file: Some(file),
        line: line_no,
        col: None,
        code: None,
        message: msg,
        platform: "python".to_string(),
    })
}

fn parse_generic_error(line: &str) -> Option<ParsedError> {
    let lower = line.to_lowercase();
    if lower.contains("error:") || lower.contains("failed:") || lower == "build failed" {
        let msg = line.trim().to_string();
        if msg.len() < 5 {
            return None;
        }
        return Some(ParsedError {
            file: None,
            line: None,
            col: None,
            code: None,
            message: msg,
            platform: "generic".to_string(),
        });
    }
    None
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn is_warning_line(line: &str) -> bool {
    let l = line.trim().to_lowercase();
    l.starts_with("warning") || l.contains("warning:")
}

fn parse_file_line_col(s: &str) -> Option<(Option<String>, Option<u32>, Option<u32>)> {
    // `src/main.rs:42:5` or `src/main.rs:42`
    let parts: Vec<&str> = s.splitn(3, ':').collect();
    if parts.is_empty() {
        return None;
    }
    let file = Some(parts[0].to_string());
    let line_no = parts.get(1).and_then(|s| s.parse::<u32>().ok());
    let col_no = parts.get(2).and_then(|s| s.parse::<u32>().ok());
    Some((file, line_no, col_no))
}

fn dedup_push(
    errors: &mut Vec<ParsedError>,
    e: ParsedError,
    seen: &mut std::collections::HashSet<String>,
    max: usize,
) {
    if errors.len() >= max {
        return;
    }
    // Dedup key: file + line + first 60 chars of message
    let key = format!(
        "{}:{}",
        e.file.as_deref().unwrap_or(""),
        &e.message[..e.message.len().min(60)]
    );
    if seen.insert(key) {
        errors.push(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_rust_errors() {
        let output = r#"
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
  --> src/main.rs:42:5
warning: unused import: `std::collections::HashMap`
  --> src/lib.rs:1:5
"#;
        let result = filter_output(output, Some(1));
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.warning_count, 1);
        assert_eq!(result.errors[0].platform, "rust");
        assert_eq!(result.errors[0].file.as_deref(), Some("src/main.rs"));
        assert_eq!(result.errors[0].line, Some(42));
        assert_eq!(result.errors[0].code.as_deref(), Some("E0596"));
    }

    #[test]
    fn filters_tsc_errors() {
        let output = "src/app.ts(12,5): error TS2345: Argument of type 'string' is not assignable";
        let result = filter_output(output, Some(1));
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].platform, "typescript");
        assert_eq!(result.errors[0].line, Some(12));
    }

    #[test]
    fn to_ai_context_is_compact() {
        let output = "error[E0001]: test\n  --> src/a.rs:1:1";
        let filtered = filter_output(output, Some(1));
        let ctx = filtered.to_ai_context();
        // Should be under 500 chars for a single error
        assert!(ctx.len() < 500, "context too large: {} chars", ctx.len());
    }
}
