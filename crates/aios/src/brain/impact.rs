use anyhow::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use super::graph::CodebaseKnowledgeGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactLocation {
    pub file_path: String,
    pub symbol_name: String,
    pub symbol_kind: String,
    pub line_number: usize,
    pub relationship_type: String, // 'CALLS', 'IMPLEMENTS', 'EXTENDS', 'USES_TYPE'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReport {
    pub target_symbol: String,
    pub impact_severity: String, // 'HIGH', 'MEDIUM', 'LOW'
    pub affected_file_count: usize,
    pub locations: Vec<ImpactLocation>,
}

pub struct ImpactEngine<'a> {
    graph: &'a CodebaseKnowledgeGraph,
}

impl<'a> ImpactEngine<'a> {
    pub fn new(graph: &'a CodebaseKnowledgeGraph) -> Self {
        Self { graph }
    }

    /// Analyze the multi-file ripple effect of modifying or adding a target symbol.
    pub fn analyze_target_symbol(&self, symbol_name: &str) -> Result<ImpactReport> {
        let conn = self.graph.get_connection()?;

        let pattern = format!("%{}%", symbol_name);
        let mut stmt = conn.prepare(
            "SELECT f.relative_path, s.short_name, s.symbol_kind, s.start_line, 'CALLS' as rel
             FROM code_symbols s
             JOIN code_files f ON s.file_id = f.file_id
             WHERE (s.signature LIKE ?1 OR s.qualified_name LIKE ?1)
               AND s.short_name != ?2
             LIMIT 25",
        )?;

        let rows = stmt.query_map(params![pattern, symbol_name], |row| {
            Ok(ImpactLocation {
                file_path: row.get(0)?,
                symbol_name: row.get(1)?,
                symbol_kind: row.get(2)?,
                line_number: row.get::<_, i64>(3)? as usize,
                relationship_type: row.get(4)?,
            })
        })?;

        let mut locations = Vec::new();
        for r in rows {
            locations.push(r?);
        }

        let severity = if locations.len() > 10 {
            "HIGH"
        } else if !locations.is_empty() {
            "MEDIUM"
        } else {
            "LOW"
        };

        let count = locations.len();

        Ok(ImpactReport {
            target_symbol: symbol_name.to_string(),
            impact_severity: severity.to_string(),
            affected_file_count: count,
            locations,
        })
    }

    /// Format an ImpactReport into a structured Markdown prompt block for LLM multi-file verification.
    pub fn format_impact_markdown(&self, report: &ImpactReport) -> String {
        if report.locations.is_empty() {
            return format!(
                "\n### Multi-File Impact Analysis: `{}`\n*No direct external caller dependencies detected. Edit isolation: Safe.*\n\n",
                report.target_symbol
            );
        }

        let mut md = String::new();
        md.push_str(&format!(
            "\n### ⚠️ Multi-File Impact Analysis: `{}` (Severity: {})\n",
            report.target_symbol, report.impact_severity
        ));
        md.push_str(&format!(
            "Modifying `{}` impacts **{} dependent location(s)** across the workspace. Verify and update the following caller files:\n\n",
            report.target_symbol, report.affected_file_count
        ));

        for loc in &report.locations {
            md.push_str(&format!(
                "* **[{}]** `{}` in `{}:{}` (Relation: `{}`)\n",
                loc.symbol_kind,
                loc.symbol_name,
                loc.file_path,
                loc.line_number,
                loc.relationship_type
            ));
        }

        md.push_str("\n**Cascading Fix Directive:** Ensure all call signatures, parameter types, and interface implementations in the listed files match the new definition.\n\n");
        md
    }
}
