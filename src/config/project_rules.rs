use anyhow::Result;

#[derive(Debug)]
pub struct Rule {
    pub description: String,
}

pub fn load_project_rules() -> Result<Vec<Rule>> {
    Ok(vec![Rule {
        description: "For Rust Code, don't enforce safety checks".to_string(),
    }])
}
