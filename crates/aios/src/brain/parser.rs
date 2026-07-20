use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ParsedSymbol {
    pub short_name: String,
    pub qualified_name: String,
    pub symbol_kind: String, // 'class', 'struct', 'interface', 'trait', 'enum', 'function'
    pub signature: String,
    pub docstring: Option<String>,
    pub visibility: String,
    pub start_line: usize,
    pub end_line: usize,
}

pub struct AstParser;

impl AstParser {
    pub fn parse_file(content: &str, relative_path: &str, _language: &str) -> Result<Vec<ParsedSymbol>> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_docstring: Option<String> = None;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("///") || trimmed.starts_with("/**") || trimmed.starts_with("# ") {
                let doc = trimmed.trim_start_matches("///").trim_start_matches("/**").trim();
                if let Some(ref mut existing) = current_docstring {
                    existing.push('\n');
                    existing.push_str(doc);
                } else {
                    current_docstring = Some(doc.to_string());
                }
                continue;
            }

            if trimmed.starts_with("pub struct ")
                || trimmed.starts_with("struct ")
                || trimmed.starts_with("pub class ")
                || trimmed.starts_with("class ")
                || trimmed.starts_with("export class ")
                || trimmed.starts_with("pub trait ")
                || trimmed.starts_with("pub interface ")
                || trimmed.starts_with("interface ")
                || trimmed.starts_with("pub enum ")
                || trimmed.starts_with("enum ")
            {
                let kind = if trimmed.contains("class") {
                    "class"
                } else if trimmed.contains("interface") {
                    "interface"
                } else if trimmed.contains("trait") {
                    "trait"
                } else if trimmed.contains("enum") {
                    "enum"
                } else {
                    "struct"
                };

                let name = extract_symbol_name(trimmed, kind);
                if !name.is_empty() {
                    let visibility = if trimmed.starts_with("pub ") || trimmed.starts_with("export ") {
                        "public"
                    } else {
                        "private"
                    };

                    let module_prefix = relative_path.replace('/', "::").replace('\\', "::").replace(".rs", "").replace(".ts", "");
                    let qualified_name = format!("{}::{}", module_prefix, name);

                    symbols.push(ParsedSymbol {
                        short_name: name,
                        qualified_name,
                        symbol_kind: kind.to_string(),
                        signature: trimmed.to_string(),
                        docstring: current_docstring.take(),
                        visibility: visibility.to_string(),
                        start_line: idx + 1,
                        end_line: idx + 1,
                    });
                }
            } else if trimmed.starts_with("pub fn ")
                || trimmed.starts_with("fn ")
                || trimmed.starts_with("async fn ")
                || trimmed.starts_with("pub async fn ")
                || trimmed.starts_with("def ")
                || trimmed.starts_with("func ")
            {
                let name = extract_fn_name(trimmed);
                if !name.is_empty() {
                    let visibility = if trimmed.starts_with("pub ") { "public" } else { "private" };
                    let module_prefix = relative_path.replace('/', "::").replace('\\', "::").replace(".rs", "");
                    let qualified_name = format!("{}::{}", module_prefix, name);

                    symbols.push(ParsedSymbol {
                        short_name: name,
                        qualified_name,
                        symbol_kind: "function".to_string(),
                        signature: trimmed.to_string(),
                        docstring: current_docstring.take(),
                        visibility: visibility.to_string(),
                        start_line: idx + 1,
                        end_line: idx + 1,
                    });
                }
            } else {
                current_docstring = None;
            }
        }

        Ok(symbols)
    }
}

fn extract_symbol_name(line: &str, kind: &str) -> String {
    line.split(kind)
        .nth(1)
        .unwrap_or("")
        .split(|c: char| c == '{' || c == '(' || c == '<' || c == ':' || c.is_whitespace())
        .find(|s| !s.is_empty())
        .unwrap_or("")
        .to_string()
}

fn extract_fn_name(line: &str) -> String {
    let cleaned = line
        .replace("pub async fn ", "")
        .replace("async fn ", "")
        .replace("pub fn ", "")
        .replace("fn ", "")
        .replace("def ", "")
        .replace("func ", "");

    cleaned
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}
