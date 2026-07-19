//! # AIOS Constitution Prompt Builder
//!
//! Parses the AIOS constitution from `aios/constitution/constitution.json`
//! and generates a system-prompt block that can be injected into the
//! HelpOfAi model guidance pipeline.

use std::path::Path;

use crate::types::Constitution;

/// Load the AIOS constitution and format it as a system-prompt section.
///
/// Returns a Markdown string suitable for injection after the main
/// HelpOfAi Constitution block.
pub fn load_constitution_prompt(aios_root: &Path) -> anyhow::Result<String> {
    let constitution_path = aios_root.join("constitution").join("constitution.json");

    let raw = std::fs::read_to_string(&constitution_path)
        .map_err(|e| anyhow::anyhow!("cannot read AIOS constitution: {e}"))?;

    let constitution: Constitution = serde_json::from_str(&raw)
        .map_err(|e| anyhow::anyhow!("invalid AIOS constitution: {e}"))?;

    Ok(format_constitution_prompt(&constitution))
}

/// Format the constitution as a prompt block.
fn format_constitution_prompt(c: &Constitution) -> String {
    let mut block = String::new();

    block.push_str("## AIOS Constitution (v1.x Enterprise)\n\n");
    block.push_str(&format!("**{}**\n\n", c.description));

    if let Some(ref golden) = c.golden_rule {
        block.push_str(&format!("**Golden Rule**: {}\n\n", golden.statement));
    }

    block.push_str("### Engineering Principles\n\n");

    for principle in &c.principles {
        block.push_str(&format!(
            "- **{}** ({}) — *{}*: {}\n",
            principle.id, principle.category, principle.name, principle.description
        ));
    }

    block.push('\n');
    block.push_str("These principles govern all AIOS agent behavior. ");
    block.push_str(
        "When in conflict, higher-numbered principles yield to lower-numbered principles.\n\n",
    );

    if !c.supremacy_order.is_empty() {
        block.push_str(&format!(
            "**Authority order**: {}\n",
            c.supremacy_order.join(" → ")
        ));
    }

    block.push('\n');
    block
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Principle;

    #[test]
    fn format_minimal_constitution() {
        let c = Constitution {
            id: "AIOS-CONST-000001".into(),
            name: "AIOS Constitution".into(),
            version: "1.0.0".into(),
            description: "Test constitution".into(),
            principles: vec![Principle {
                id: "AIOS-PRINCIPLE-001".into(),
                name: "Capability-First".into(),
                category: "architecture".into(),
                description: "Load only needed capabilities".into(),
                enforcement: "strict".into(),
            }],
            golden_rule: None,
            ..Default::default()
        };

        let prompt = format_constitution_prompt(&c);
        assert!(prompt.contains("AIOS Constitution"));
        assert!(prompt.contains("Capability-First"));
        assert!(prompt.contains("architecture"));
    }
}
