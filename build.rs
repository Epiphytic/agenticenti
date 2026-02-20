use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join("prompts.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let prompts_dir = Path::new(&manifest_dir).join("prompts");

    let mut code = String::new();

    // Generate modules for each category
    code.push_str(&generate_category(&prompts_dir, "roles"));
    code.push_str(&generate_category(&prompts_dir, "overlays"));
    code.push_str(&generate_category(&prompts_dir, "testing-modes"));
    code.push_str(&generate_category(&prompts_dir, "appendices"));

    // Generate lookup functions
    code.push_str(&generate_lookup_fn("role", "roles", &prompts_dir));
    code.push_str(&generate_lookup_fn("overlay", "overlays", &prompts_dir));
    code.push_str(&generate_lookup_fn(
        "testing_mode",
        "testing-modes",
        &prompts_dir,
    ));
    code.push_str(&generate_lookup_fn("appendix", "appendices", &prompts_dir));

    // Generate name list functions
    code.push_str(&generate_names_fn("role", "roles", &prompts_dir));
    code.push_str(&generate_names_fn("overlay", "overlays", &prompts_dir));
    code.push_str(&generate_names_fn(
        "testing_mode",
        "testing-modes",
        &prompts_dir,
    ));
    code.push_str(&generate_names_fn("appendix", "appendices", &prompts_dir));

    fs::write(&dest_path, code).unwrap();

    // Tell Cargo to re-run if any prompt file changes
    for category in &["roles", "overlays", "testing-modes", "appendices"] {
        let dir = prompts_dir.join(category);
        for (_, path) in discover_prompts(&dir) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn discover_prompts(dir: &Path) -> Vec<(String, PathBuf)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md") {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                entries.push((name, path));
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

fn to_const_name(name: &str) -> String {
    name.replace('-', "_").to_uppercase()
}

fn generate_category(prompts_dir: &Path, category: &str) -> String {
    let dir = prompts_dir.join(category);
    let entries = discover_prompts(&dir);
    let mod_name = category.replace('-', "_");

    let mut code = format!("pub mod {} {{\n", mod_name);
    for (name, path) in &entries {
        let const_name = to_const_name(name);
        let path_str = path.display();
        code.push_str(&format!(
            "    pub const {}: &str = include_str!(\"{}\");\n",
            const_name, path_str
        ));
    }
    code.push_str("}\n\n");
    code
}

fn generate_lookup_fn(fn_suffix: &str, category: &str, prompts_dir: &Path) -> String {
    let dir = prompts_dir.join(category);
    let entries = discover_prompts(&dir);

    let mut code = format!(
        "pub fn embedded_{}(name: &str) -> Option<&'static str> {{\n    match name {{\n",
        fn_suffix
    );
    for (name, _) in &entries {
        let const_name = to_const_name(name);
        let mod_name = category.replace('-', "_");
        code.push_str(&format!(
            "        \"{}\" => Some({}::{}),\n",
            name, mod_name, const_name
        ));
    }
    code.push_str("        _ => None,\n    }\n}\n\n");
    code
}

fn generate_names_fn(fn_suffix: &str, category: &str, prompts_dir: &Path) -> String {
    let dir = prompts_dir.join(category);
    let entries = discover_prompts(&dir);

    let names: Vec<String> = entries.iter().map(|(n, _)| format!("\"{}\"", n)).collect();
    format!(
        "pub fn all_{}_names() -> &'static [&'static str] {{\n    &[{}]\n}}\n\n",
        fn_suffix,
        names.join(", ")
    )
}
