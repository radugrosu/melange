use quote::ToTokens;
use std::fs;
use syn::{File, Item};

#[derive(Debug, Clone)]
pub struct Rule {
    description: String,
    line_number: usize,
}

#[derive(Debug)]
pub struct RuleWithCode {
    rule: Rule,
    code_type: String,  // "enum", "function", "struct", etc.
    item_name: String,  // Name of the item
    code_block: String, // The actual code as a string
    start_line: usize,  // Starting line of the code block
    end_line: usize,    // Ending line of the code block
}

pub fn parse_rust_file(file_path: &str) -> Vec<RuleWithCode> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File = syn::parse_file(&content).expect("Failed to parse Rust file");

    let mut rules_with_code = Vec::new();

    for (line_number, line) in content.lines().enumerate() {
        if let Some(rule) = extract_airule_comment(line, line_number + 1) {
            println!("found rule: {:?}", rule);
            for item in &syntax_tree.items {
                match item {
                    Item::Enum(item_enum) => {
                        rules_with_code.push(create_rule_with_code(
                            rule.clone(),
                            "enum",
                            &item_enum.ident.to_string(),
                            item_enum.to_token_stream().to_string(),
                            line_number + 1,
                        ));
                    }
                    Item::Fn(item_fn) => {
                        rules_with_code.push(create_rule_with_code(
                            rule.clone(),
                            "function",
                            &item_fn.sig.ident.to_string(),
                            item_fn.to_token_stream().to_string(),
                            line_number + 1,
                        ));
                    }
                    Item::Struct(item_struct) => {
                        rules_with_code.push(create_rule_with_code(
                            rule.clone(),
                            "struct",
                            &item_struct.ident.to_string(),
                            item_struct.to_token_stream().to_string(),
                            line_number + 1,
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    rules_with_code
}

fn extract_airule_comment(line: &str, line_number: usize) -> Option<Rule> {
    if line.trim_start().starts_with("// #AIRULE:") {
        let description = line.trim_start_matches("// #AIRULE:").trim().to_string();
        return Some(Rule {
            description,
            line_number,
        });
    }
    None
}

fn create_rule_with_code(
    rule: Rule,
    code_type: &str,
    item_name: &str,
    code_block: String,
    start_line: usize,
) -> RuleWithCode {
    RuleWithCode {
        rule,
        code_type: code_type.to_string(),
        item_name: item_name.to_string(),
        code_block,
        start_line,
        end_line: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_file() {
        let rules = parse_rust_file("./examples/rust_enum.rs");
        assert!(!rules.is_empty());
        for rule in rules {
            println!("{:?}", rule);
        }
    }
}
