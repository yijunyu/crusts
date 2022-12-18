use jwalk::WalkDir;
use std::{
    path::Path,
    process::{Command, Stdio},
};
pub fn is_file_with_ext(p: &Path, file_ext: &str) -> bool {
    let ext = match p.extension() {
        Some(e) => e,
        None => return false,
    };
    ext.to_string_lossy() == file_ext
}

fn main() {
    println!("Generating compile_commands.json from Makefile ...");
    if !std::path::Path::new("compile_commands.json").exists()
        & &(std::path::Path::new("Makefile").exists() || std::path::Path::new("Makefile").exists())
    {
        if let Ok(command) = Command::new("intercept-build")
            .args(["make"])
            .stdout(Stdio::piped())
            .spawn()
        {
            if let Ok(output) = command.wait_with_output() {
                println!("{:?}", output);
            } else {
                let _ = Command::new("pip")
                    .args(["install", "scan-build"])
                    .spawn();
                println!("Please add ~/.local/bin to your PATH, and start again.");
            }
        }
    }
    println!("Apply C2Rust transpilation ...");
    match Command::new("c2rust-transpile")
        .args(["-e", "-b", "main", "-o", "c2rust", "compile_commands.json"])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(command) => {
            if let Ok(output) = command.wait_with_output() {
                println!("{:?}", output);
            }
        }
        Err(_) => {
            Command::new("cargo")
                .args(["install", "c2rust"])
                .stdout(Stdio::piped())
                .spawn()
                .ok();
        }
    }
    println!("Apply C2Rust transpilation again ...");
    match Command::new("c2rust-transpile")
        .args(["-e", "-b", "main", "-o", "crusts", "compile_commands.json"])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(command) => {
            if let Ok(output) = command.wait_with_output() {
                println!("{:?}", output);
            }
        }
        Err(_) => {
            Command::new("cargo")
                .args(["install", "c2rust-transpile"])
                .stdout(Stdio::piped())
                .spawn()
                .ok();
        }
    }
    println!("Apply TXL transformations to reduce unsafe code ...");
    WalkDir::new("./crusts").sort(true).into_iter().for_each(|entry| {
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
    #[test]
    fn test_main() {
        let dir = std::path::Path::new("abc");
        if dir.exists() {
            std::fs::remove_dir_all(dir).ok();
        }
        std::fs::create_dir_all(dir).ok();
        std::fs::write(
            "abc/main.c",
            r#"
#include <stdio.h>
int main() {
    printf("Hello, world!");
    return 0;
}
"#,
        )
        .ok();
        std::fs::write("abc/Makefile", "main: main.c\n\tgcc -o main main.c\n").ok();
        std::env::set_current_dir(dir).ok();
        main();
        if let Ok(s) = std::fs::read_to_string("c2rust/src/main.rs") {
            insta :: assert_snapshot! (s, @ r###"
            #![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
            #![register_tool(c2rust)]
            #![feature(register_tool)]
            use ::c2rust::*;
            extern "C" {
                fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
            }
            unsafe fn main_0() -> libc::c_int {
                printf(b"Hello, world!\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }
            pub fn main() {
                unsafe { ::std::process::exit(main_0() as i32) }
            }
            "###);
        }
        if let Ok(s) = std::fs::read_to_string("crusts/src/main.rs") {
            insta :: assert_snapshot! (s, @ r###"
            #![allow(
                dead_code,
                mutable_transmutes,
                non_camel_case_types,
                non_snake_case,
                non_upper_case_globals,
                unused_assignments,
                unused_mut
            )]
            #![register_tool(c2rust)]
            #![feature(register_tool)]
            use crusts::*;
            extern "C" {
                fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
            }
            fn main_0() -> libc::c_int {
                printf(b"Hello, world!\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }

            pub fn main() {
                ::std::process::exit(main_0() as i32);
            }
            "###);
        }
    }
}
