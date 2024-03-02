use anyhow::Result;
use std::fs;
use std::io::{self};
use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<()> {
    let tests_path = Path::new("../../tests/main.rs");
    let examples_path = Path::new("../../examples");

    let mut src: Vec<String> = [
        "mod tests {",
        "\textern crate rslox;",
        "",
        "\tuse rslox::lox::Lox;",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let mut other_files = Vec::new();
    let entries = get_entries(examples_path)?;
    for entry in entries {
        let s = path_to_string(&entry)?;
        if s.starts_with('_') {
            continue;
        }
        if entry.is_dir() {
            let files = get_entries(&entry)?;
            src.extend_from_slice(&method(&s, &files)?);
        } else {
            other_files.push(entry);
        }
    }

    src.extend_from_slice(&method("others", &other_files)?);
    src.push("}".to_string());

    fs::write(tests_path, src.join("\n"))?;
    Ok(())
}

fn get_entries(path: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|r| r.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    Ok(entries)
}

fn path_to_string(path: &Path) -> Result<String> {
    let s = path
        .file_stem()
        .expect("Failed to convert path to string")
        .to_string_lossy()
        .to_string();

    Ok(s)
}

fn method(dir: &str, entries: &[PathBuf]) -> Result<Vec<String>> {
    let mut method: Vec<String> = ["", "\t#[test]"].iter().map(|s| s.to_string()).collect();
    method.push(format!("\tfn check_{}() {{", dir));
    method.extend_from_slice(&asserts(entries)?);
    method.push("\t}".to_string());

    Ok(method)
}

fn asserts(files: &[PathBuf]) -> Result<Vec<String>> {
    let mut scripts = Vec::new();
    for f in files {
        let file_name = path_to_string(f)?;
        let suffix = if file_name.starts_with("ok_") {
            ".is_ok()"
        } else if file_name.starts_with("err_") {
            ".is_err()"
        } else {
            continue;
        };

        let assert = format!(
            "\t\tassert!(Lox::run_file(\"{}\"){});",
            f.strip_prefix("../../")?.to_string_lossy(),
            suffix
        );

        scripts.push(assert);
    }

    Ok(scripts)
}
