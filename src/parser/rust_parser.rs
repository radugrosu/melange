use crate::rules::generic::{RuleWithCode, extract_rule_map};
use std::{fs, sync::Arc};
use syn::{File, Item, spanned::Spanned};

#[derive(Debug)]
pub struct ModuleNode {
    name: String,
    kind: String,
    visibility: String,
    is_async: bool,
    children: Vec<ModuleNode>,
}

pub fn parse_rust_file(file_path: &str) -> Vec<RuleWithCode> {
    let content = Arc::new(fs::read_to_string(file_path).expect("Failed to read file"));
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
                Item::Mod(item_mod) => ("mod", item_mod.ident.to_string()),
                _ => {
                    eprintln!("Unsupported item type");
                    continue;
                }
            };
            let rule = RuleWithCode::new(
                rule.to_string(),
                Arc::clone(&content),
                code_type.to_string(),
                item_name.clone(),
                item.span().byte_range(),
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
