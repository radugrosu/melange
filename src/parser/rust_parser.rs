use crate::rules::generic::{RuleWithCode, extract_rule_map};
use quote::ToTokens;
use std::fs;
use syn::{File, Item, spanned::Spanned};

pub fn parse_rust_file(file_path: &str) -> Vec<RuleWithCode> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let syntax_tree: File = syn::parse_file(&content).expect("Failed to parse Rust file");

    let rule_map = extract_rule_map(&content);
    let mut rules = Vec::new();
    for item in &syntax_tree.items {
        let start_line = item.span().start().line;
        if let Some(rule) = rule_map.get(&start_line) {
            let (code_type, item_name) = match item {
                Item::Enum(item_enum) => ("enum", item_enum.ident.to_string()),
                Item::Fn(item_fn) => ("function", item_fn.sig.ident.to_string()),
                Item::Struct(item_struct) => ("struct", item_struct.ident.to_string()),
                _ => {
                    eprintln!("Unsupported item type");
                    continue;
                }
            };
            let rule = RuleWithCode::new(
                rule.clone().to_string(),
                code_type.to_string(),
                item_name,
                item.to_token_stream().to_string(),
                start_line,
                item.span().end().line,
            );
            rules.push(rule);
        }
    }
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_file() {
        let rules = parse_rust_file("./lint-examples/rust_enum.rs");
        assert!(!rules.is_empty());
    }
}
