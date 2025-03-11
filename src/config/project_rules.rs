use anyhow::Result;

#[derive(Debug)]
pub struct Rule {
    pub description: String,
}

pub fn load_project_rules() -> Result<Vec<Rule>> {
    Ok(vec![
        Rule {
            description: "For Python code, conform to PEP8".to_string(),
        },
        Rule {
            description: "For Rust Code, don't enforce safety checks".to_string(),
        },
        Rule {
            description:
                "For every hook in src/layer/* make sure a test is implemented in e2e/file.rs"
                    .to_string(),
        },
    ])
}
