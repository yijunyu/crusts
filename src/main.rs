use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use walkdir::{DirEntry, WalkDir};

pub fn is_file_with_ext(entry: &DirEntry, file_ext: &str) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }
    let p = entry.path();
    let ext = match p.extension() {
        Some(e) => e,
        None => return false,
    };
    // to_string_lossy is ok since we only want to match against an ASCII
    // compatible extension and we do not keep the possibly lossy result
    // around.
    ext.to_string_lossy() == file_ext
}

fn find_rs_files_in_dir(dir: &Path) -> impl Iterator<Item = PathBuf> {
    let walker = WalkDir::new(dir).into_iter();
    walker.filter_map(|entry| {
        let entry = entry.expect("walkdir error."); // TODO: Return result.
        if !is_file_with_ext(&entry, "rs") {
            return None;
        }
        Some(
            entry
                .path()
                .canonicalize()
                .expect("Error converting to canonical path"),
        ) // TODO: Return result.
    })
}

fn main() {
    for path in find_rs_files_in_dir(Path::new("src")) {
        let file = &format!("{}", &path.into_os_string().to_string_lossy());
        fix_unsafe(file);
    }
}

extern crate reqwest;

const URL: &str = "http://bertrust.s3.amazonaws.com/unsafe.txl";
fn fix_unsafe(file: &str) {
    if !std::path::Path::new("crusts.tar.gz").exists() {
        if let Ok(resp) = reqwest::blocking::get(URL) {
            if let Ok(bytes) = resp.bytes() {
                std::fs::write("unsafe.txl", bytes).ok();
            }
        }
    }
    let args = vec![
        "-q".to_string(),
        "-s".to_string(),
        "3000".to_string(),
        file.to_string(),
        "unsafe.txl".to_string(),
    ];
    if let Ok(output) = txl_rs::txl(args) {
        std::fs::write(file, output).ok();
        if let Ok(command) = Command::new("rustfmt")
            .args([file])
            .stdout(Stdio::piped())
            .spawn()
        {
            if let Ok(_output) = command.wait_with_output() {
                if let Ok(s) = std::fs::read_to_string(file) {
                    println!("{file}");
                    std::fs::write(file, s).ok();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    #[test]
    fn test_main() {
        let dir = std::path::Path::new("abc");
        if !dir.exists() {
            Command::new("cargo")
                .args(["init", "--bin", "abc"])
                .spawn()
                .ok();
            let code = r#"
fn main() {
    let s = std::fs::read_to_string("Cargo.toml").unwrap();
    println!("{s}");
}
"#;
            std::fs::write("test/src/main.rs", code).ok();
        }
        std::env::set_current_dir(dir).ok();
        main();
    }
}
