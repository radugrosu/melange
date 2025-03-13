use std::fs;
use std::path::{Path, PathBuf};
use syn::{Item, Visibility};

#[derive(Debug, Clone)]
pub struct ModuleNode {
    name: String,
    kind: String,
    visibility: String,
    is_async: bool,
    children: Vec<ModuleNode>,
    path: PathBuf,
}

fn get_crate_name(crate_path: &Path) -> String {
    let cargo_path = crate_path.join("Cargo.toml");

    match fs::read_to_string(&cargo_path) {
        Ok(content) => {
            let cargo_toml: toml::Value = content.parse().expect("Failed ot parse Cargo.toml");
            cargo_toml
                .get("package")
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| fallback_crate_name(crate_path))
        }
        Err(_) => fallback_crate_name(crate_path),
    }
}
fn fallback_crate_name(crate_path: &Path) -> String {
    crate_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown_crate")
        .to_string()
}

pub fn parse_crate(crate_path: &str) -> ModuleNode {
    let path = Path::new(crate_path);
    let crate_name = get_crate_name(path);
    // Start with the root module (crate)
    let mut root = ModuleNode {
        name: crate_name.to_string(),
        kind: "crate".to_string(),
        visibility: "pub".to_string(),
        is_async: false,
        children: Vec::new(),
        path: path.to_path_buf(),
    };

    // Check for lib.rs or main.rs
    let lib_path = path.join("src/lib.rs");
    let main_path = path.join("src/main.rs");

    if lib_path.exists() {
        process_file(&lib_path, &mut root);
    }
    if main_path.exists() {
        process_file(&main_path, &mut root);
    } else {
        eprintln!("Neither lib.rs nor main.rs found in {}", crate_path);
    }

    root
}

fn process_file(file_path: &Path, parent: &mut ModuleNode) {
    // Parse the file content
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path.display(), e);
            return;
        }
    };

    let syntax = match syn::parse_file(&file_content) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Failed to parse file {}: {}", file_path.display(), e);
            return;
        }
    };

    // Process items in the file
    for item in syntax.items {
        match item {
            Item::Mod(item_mod) => {
                let mod_name = item_mod.ident.to_string();
                let visibility = get_visibility(&item_mod.vis);

                let mut module = ModuleNode {
                    name: mod_name.clone(),
                    kind: "mod".to_string(),
                    visibility,
                    is_async: false,
                    children: Vec::new(),
                    path: file_path.to_path_buf(),
                };

                // Check if module content is inline
                if let Some((_, items)) = item_mod.content {
                    // Inline module
                    for child_item in items {
                        process_item(child_item, &mut module);
                    }
                } else {
                    // External module - check for file in same directory or subdir
                    let parent_dir = file_path.parent().unwrap_or(Path::new(""));

                    // Check for [module_name].rs
                    let mod_file = parent_dir.join(format!("{}.rs", mod_name));
                    // Check for [module_name]/mod.rs
                    let mod_dir_file = parent_dir.join(&mod_name).join("mod.rs");

                    if mod_file.exists() {
                        process_file(&mod_file, &mut module);
                    } else if mod_dir_file.exists() {
                        process_file(&mod_dir_file, &mut module);
                    }
                }
                parent.children.push(module);
            }
            item => process_item(item, parent),
        }
    }
}

fn process_item(item: Item, parent: &mut ModuleNode) {
    match item {
        Item::Struct(item_struct) => {
            parent.children.push(ModuleNode {
                name: item_struct.ident.to_string(),
                kind: "struct".to_string(),
                visibility: get_visibility(&item_struct.vis),
                is_async: false,
                children: Vec::new(),
                path: parent.path.clone(),
            });
        }
        Item::Enum(item_enum) => {
            parent.children.push(ModuleNode {
                name: item_enum.ident.to_string(),
                kind: "enum".to_string(),
                visibility: get_visibility(&item_enum.vis),
                is_async: false,
                children: Vec::new(),
                path: parent.path.clone(),
            });
        }
        Item::Fn(item_fn) => {
            parent.children.push(ModuleNode {
                name: item_fn.sig.ident.to_string(),
                kind: "function".to_string(),
                visibility: get_visibility(&item_fn.vis),
                is_async: item_fn.sig.asyncness.is_some(),
                children: Vec::new(),
                path: parent.path.clone(),
            });
        }
        Item::Mod(item_mod) => {
            let mod_name = item_mod.ident.to_string();
            let visibility = get_visibility(&item_mod.vis);

            let mut module = ModuleNode {
                name: mod_name.clone(),
                kind: "mod".to_string(),
                visibility,
                is_async: false,
                children: Vec::new(),
                path: parent.path.clone(),
            };

            if let Some((_, items)) = item_mod.content {
                for child_item in items {
                    process_item(child_item, &mut module);
                }
            }

            parent.children.push(module);
        }
        Item::Impl(item_impl) => {
            let self_ty = get_self_type(&item_impl.self_ty);
            // Look for a struct or enum in the current module's children
            if let Some(struct_node) = parent
                .children
                .iter_mut()
                .find(|n| (n.kind == "struct" || n.kind == "enum") && n.name == self_ty)
            {
                for impl_item in item_impl.items {
                    if let syn::ImplItem::Fn(impl_fn) = impl_item {
                        struct_node.children.push(ModuleNode {
                            name: impl_fn.sig.ident.to_string(),
                            kind: "method".to_string(),
                            visibility: get_visibility(&impl_fn.vis),
                            is_async: impl_fn.sig.asyncness.is_some(),
                            children: Vec::new(),
                            path: parent.path.clone(),
                        });
                    }
                }
            }
        }
        _ => {}
    }
}
fn get_self_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident.to_string()
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}
fn get_visibility(vis: &Visibility) -> String {
    match vis {
        Visibility::Public(_) => "pub".to_string(),
        Visibility::Restricted(restricted) => {
            if let Some(path) = restricted.path.segments.first() {
                if path.ident == "self" {
                    "pub(self)".to_string()
                } else if path.ident == "crate" {
                    "pub(crate)".to_string()
                } else if path.ident == "super" {
                    "pub(super)".to_string()
                } else {
                    format!("pub({})", path.ident)
                }
            } else {
                "restricted".to_string()
            }
        }
        Visibility::Inherited => "private".to_string(),
    }
}

pub fn print_module_tree(root: &ModuleNode) {
    println!("crate {}", root.name);
    print_modules(&root.children, &vec![]);
}

fn print_modules(modules: &[ModuleNode], is_last_ancestors: &[bool]) {
    for (idx, module) in modules.iter().enumerate() {
        let is_last = idx == modules.len() - 1;

        // Print ancestor connectors
        for is_last_ancestor in is_last_ancestors {
            if *is_last_ancestor {
                print!("    ");
            } else {
                print!("│   ");
            }
        }
        // Print current node
        let prefix = if is_last { "└── " } else { "├── " };
        let node_info = match module.kind.as_str() {
            "mod" => format!("mod {}", module.name),
            "struct" => format!("struct {}", module.name),
            "enum" => format!("enum {}", module.name),
            "function" | "method" => {
                let async_prefix = if module.is_async { "async " } else { "" };
                format!("fn {}{}", async_prefix, module.name)
            }
            _ => module.name.clone(),
        };

        let visibility_suffix = if module.visibility != "private" {
            format!(": {}", module.visibility)
        } else {
            String::new()
        };

        println!("{}{}{}", prefix, node_info, visibility_suffix);

        // Prepare ancestor state for children
        let mut child_ancestors = is_last_ancestors.to_vec();
        child_ancestors.push(is_last);
        print_modules(&module.children, &child_ancestors);
    }
}
