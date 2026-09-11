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
    pub fn parse_file(
        content: &str,
        relative_path: &str,
        _language: &str,
    ) -> Result<Vec<ParsedSymbol>> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_docstring: Option<String> = None;

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("///") || trimmed.starts_with("/**") || trimmed.starts_with("# ")
            {
                let doc = trimmed
                    .trim_start_matches("///")
                    .trim_start_matches("/**")
                    .trim();
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
                || trimmed.starts_with("abstract class ")
                || trimmed.starts_with("final class ")
                || trimmed.starts_with("readonly class ")
                || trimmed.starts_with("export class ")
                || trimmed.starts_with("export default class ")
                || trimmed.starts_with("pub trait ")
                || trimmed.starts_with("trait ")
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
                    let visibility = if trimmed.starts_with("pub ")
                        || trimmed.starts_with("export ")
                        || trimmed.starts_with("public ")
                    {
                        "public"
                    } else if trimmed.starts_with("protected ") {
                        "protected"
                    } else {
                        "private"
                    };

                    let module_prefix = relative_path
                        .replace(['/', '\\'], "::")
                        .replace(".rs", "")
                        .replace(".ts", "")
                        .replace(".tsx", "")
                        .replace(".js", "")
                        .replace(".jsx", "")
                        .replace(".php", "")
                        .replace(".py", "");
                    let qualified_name = format!("{module_prefix}::{name}");

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
                || trimmed.starts_with("public function ")
                || trimmed.starts_with("protected function ")
                || trimmed.starts_with("private function ")
                || trimmed.starts_with("public static function ")
                || trimmed.starts_with("protected static function ")
                || trimmed.starts_with("private static function ")
                || trimmed.starts_with("static function ")
                || trimmed.starts_with("function ")
            {
                let name = extract_fn_name(trimmed);
                if !name.is_empty() {
                    let visibility =
                        if trimmed.starts_with("pub ") || trimmed.starts_with("public ") {
                            "public"
                        } else if trimmed.starts_with("protected ") {
                            "protected"
                        } else {
                            "private"
                        };
                    let module_prefix = relative_path
                        .replace(['/', '\\'], "::")
                        .replace(".rs", "")
                        .replace(".ts", "")
                        .replace(".tsx", "")
                        .replace(".js", "")
                        .replace(".jsx", "")
                        .replace(".php", "")
                        .replace(".py", "");
                    let qualified_name = format!("{module_prefix}::{name}");

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
        .replace("func ", "")
        .replace("public static function ", "")
        .replace("protected static function ", "")
        .replace("private static function ", "")
        .replace("public function ", "")
        .replace("protected function ", "")
        .replace("private function ", "")
        .replace("static function ", "")
        .replace("function ", "");

    cleaned.split('(').next().unwrap_or("").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_php_symbols() {
        let code = r#"<?php
namespace App\Features\AI\Services;

class OmniRouteClient {
    public function routeRequest(string $prompt): string {
        return "ok";
    }

    protected static function validateToken(): bool {
        return true;
    }
}
"#;
        let symbols =
            AstParser::parse_file(code, "app/Features/AI/Services/OmniRouteClient.php", "php")
                .unwrap();
        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].short_name, "OmniRouteClient");
        assert_eq!(symbols[0].symbol_kind, "class");
        assert_eq!(symbols[1].short_name, "routeRequest");
        assert_eq!(symbols[1].symbol_kind, "function");
        assert_eq!(symbols[1].visibility, "public");
        assert_eq!(symbols[2].short_name, "validateToken");
        assert_eq!(symbols[2].symbol_kind, "function");
        assert_eq!(symbols[2].visibility, "protected");
    }

    #[test]
    fn test_parse_rust_symbols() {
        let code = r#"
pub struct EngineRunner {
    pub id: String,
}

impl EngineRunner {
    pub async fn run_task(&self) {
    }
}
"#;
        let symbols = AstParser::parse_file(code, "src/runner.rs", "rust").unwrap();
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].short_name, "EngineRunner");
        assert_eq!(symbols[1].short_name, "run_task");
    }
}
