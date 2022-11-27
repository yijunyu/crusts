use std::{
    process::{Command, Stdio}, path::Path,
};
use jwalk::WalkDir;

pub fn is_file_with_ext(p: &Path, file_ext: &str) -> bool {
    let ext = match p.extension() {
        Some(e) => e,
        None => return false,
    };
    ext.to_string_lossy() == file_ext
}

fn main() {
    WalkDir::new(".").sort(true).into_iter().for_each(|entry| {
        if let Ok(e) = entry {
            let p = e.path();
            if !is_file_with_ext(&p, "rs") {
                return; 
            }
            let file = &format!("{}", &p.into_os_string().to_string_lossy());
            fix_unsafe(file);
        }
    });
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
unsafe fn main() {
    println!("Hello, world!");
}
"#;
            std::fs::write("abc/src/main.rs", code).ok();
        }
        std::env::set_current_dir(dir).ok();
        main();
        if let Ok(s) = std::fs::read_to_string("src/main.rs") {
            insta :: assert_snapshot! (s, @ r###"
            fn main() {
                println!("Hello, world!");
            }
            "###);
        }
    }
}
