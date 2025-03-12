use anyhow::Result;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::Range,
    sync::{Arc, LazyLock},
};

use regex::Regex;

static AIRULE: LazyLock<Regex> = LazyLock::new(|| Regex::new("// +#AIRULE: +(.+)").unwrap());

#[derive(Debug, Clone)]
pub struct Rule {
    description: String,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
#[allow(dead_code)]
#[derive(Debug)]
struct RuleMetaData {
    code_type: String, // "enum", "function", "struct", etc.
    item_name: String, // Name of the item
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RuleWithCode {
    rule: String,
    file_content: Arc<String>,
    byte_range: Range<usize>,
    meta: RuleMetaData,
}

impl RuleWithCode {
    pub fn get_code_block(&self) -> &str {
        &self.file_content[self.byte_range.clone()]
    }
}

impl RuleWithCode {
    pub fn new(
        rule: String,
        file_content: Arc<String>,
        code_type: String,
        item_name: String,
        byte_range: Range<usize>,
    ) -> Self {
        let meta = RuleMetaData {
            code_type,
            item_name,
        };
        Self {
            rule,
            file_content,
            byte_range,
            meta,
        }
    }

    pub fn to_prompt(&self) -> String {
        format!(
            r#"
        <rule>{}<rule>
        <code>{}<code>
        "#,
            self.rule,
            self.get_code_block()
        )
    }
}
pub fn extract_rule_map(content: &str) -> HashMap<usize, Rule> {
    content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if let Some(caps) = AIRULE.captures(line) {
                let rule = Rule {
                    description: caps[1].to_string(),
                };
                Some((i + 2, rule))
            } else {
                None
            }
        })
        .collect::<HashMap<usize, Rule>>()
}

pub fn load_project_rules() -> Result<Vec<Rule>> {
    Ok(vec![Rule {
        description: "For Rust Code, don't enforce safety checks".to_string(),
    }])
}

