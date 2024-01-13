use anyhow::Result;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use std::path::Path;

fn main() -> Result<()> {
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
    let entries = get_entries(&PathBuf::from("../examples"))?;
    for entry in entries {
        if entry.is_dir() {
            if let Some(dir) = dir_name(&entry)? {
                let files = get_entries(&entry)?;
                src.extend_from_slice(&method(&dir, &files)?);
            }
        } else {
            other_files.push(entry);
        }
    }

    src.extend_from_slice(&method("others", &other_files)?);
    src.push("}".to_string());

    fs::write("../tests/main.rs", src.join("\n"))?;
    Ok(())
}

fn get_entries(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|r| r.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    Ok(entries)
}

fn dir_name(path: &Path) -> Result<Option<String>> {
    let dir = path
        .file_stem()
        .expect("Failed to get a directory name")
        .to_string_lossy()
        .to_string();

    match dir.starts_with('_') {
        true => Ok(None),
        false => Ok(Some(dir)),
    }
}

fn method(dir: &str, entries: &[PathBuf]) -> Result<Vec<String>> {
    let mut method: Vec<String> = ["", "\t#[test]"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    method.push(format!("\tfn {}() {{", dir));
    method.extend_from_slice(&asserts(entries)?);
    method.push("\t}".to_string());

    Ok(method)
}

fn asserts(files: &[PathBuf]) -> Result<Vec<String>> {
    let mut scripts = Vec::new();
    for f in files {
        scripts.push(assert(f)?);
    }

    Ok(scripts)
}

fn assert(file: &Path) -> Result<String> {
    let file_name = file
    .file_stem()
    .expect("Failed to get a file name")
    .to_string_lossy()
    .to_string();

    let result = if file_name.starts_with("ok_")
    {
        ".is_ok()"
    } else {
        ".is_err()"
    };

    Ok(format!(
        "\t\tassert!(Lox::run_file(\"{}\"){});",
        file.strip_prefix("../")?.to_string_lossy(), 
        result
    ))
}
