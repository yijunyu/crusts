use jwalk::WalkDir;
use std::{
    path::Path,
    process::{Command, Stdio},
};
use flate2::read::GzDecoder;
use tar::Archive;
use uuid::Uuid;
pub fn is_file_with_ext(p: &Path, file_ext: &str) -> bool {
    let ext = match p.extension() {
        Some(e) => e,
        None => return false,
    };
    ext.to_string_lossy() == file_ext
}

fn main() {
    if !std::path::Path::new("Cargo.toml").exists() { // This is a C project
        if !std::path::Path::new("compile_commands.json").exists()
        {
            if !std::path::Path::new("Makefile").exists() && !std::path::Path::new("makefile").exists() && ! std::path::Path::new("configure").exists() && ! std::path::Path::new("configure.ac").exists()  {
                let mut c_files = Vec::new();
                WalkDir::new(".").sort(true).into_iter().for_each(|entry| {
                    if let Ok(e) = entry {
                        let p = e.path();
                        if !is_file_with_ext(&p, "c") && !is_file_with_ext(&p, "cpp") {
                            return;
                        }
                        let file = format!("{}", &p.into_os_string().to_string_lossy());
                        c_files.push(file);
                    }
                });
                let mut obj = "".to_string();
                for c_file in c_files {
                    obj.push_str(" \\\n");
                    obj.push_str(&c_file.replace(".c", ".o"));
                }
                std::fs::write("Makefile", format!("main: {}\n\tgcc -o main {}\n\n.c.o: \n\tgcc -c $<\n\n.cpp.o: \n\tg++ -c $<\n\nclean::\n\trm -rf Makefile main c2rust crusts compile_commands.json txl10.8b.linux64", obj, obj)).ok();
            }
            if !std::path::Path::new("Makefile").exists() && !std::path::Path::new("configure").exists() && std::path::Path::new("configure.ac").exists()  {
                if let Ok(command) = Command::new("autoreconf")
                    .args(["-fi"])
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    if let Ok(output) = command.wait_with_output() {
                        println!("{:?}", output);
                    }
                }
            }
            if !std::path::Path::new("Makefile").exists() && std::path::Path::new("configure").exists()  {
                if let Ok(command) = Command::new("./configure")
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    if let Ok(output) = command.wait_with_output() {
                        println!("{:?}", output);
                    }
                }
            }
            if std::path::Path::new("Makefile").exists() {
                if let Ok(command) = Command::new("intercept-build")
                    .args(["make", "-k"])
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    if let Ok(output) = command.wait_with_output() {
                        println!("{:?}", output);
                    }
                }
            }
        }
        match Command::new("c2rust-transpile")
            .args(["-e", "-b", "main", "-o", ".", "compile_commands.json"])
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
    }
    WalkDir::new(".").sort(true).into_iter().for_each(|entry| {
        if let Ok(e) = entry {
            let p = e.path();
            if !is_file_with_ext(&p, "rs") {
                return;
            }
            let file = &format!("{}", &p.into_os_string().to_string_lossy());
            crusts(file);
        }
    });
}

extern crate reqwest;

#[cfg(target_os = "macos")]
const FOLDER: &str = "txl10.8b.macosx64";
#[cfg(target_os = "macos")]
const URL: &str = "http://bertrust.s3.amazonaws.com/crusts-macosx.tar.gz";
#[cfg(target_os = "linux")]
const FOLDER: &str = "txl10.8b.linux64";
#[cfg(target_os = "linux")]
const URL: &str = "http://bertrust.s3.amazonaws.com/crusts-linux.tar.gz";
#[cfg(target_os = "windows")]
const FOLDER: &str = "Txl108bwin64";
#[cfg(target_os = "windows")]
const URL: &str = "http://bertrust.s3.amazonaws.com/crusts-windows.tar.gz";

fn crusts(file: &str) {
    if !std::path::Path::new(&format!("{}/lib/Rust/unsafe.x", FOLDER)).exists() {
        if let Ok(resp) = reqwest::blocking::get(URL) {
            if let Ok(bytes) = resp.bytes() {
               let tar = GzDecoder::new(&bytes[..]);
               let mut archive = Archive::new(tar);
               archive.unpack(format!("{}/lib", FOLDER)).ok();
            }
        }
    }
    let rules = vec!["formalizeCode.x",
        "varTypeNoBounds.x",
        "null.x",
        "array.x",
        "fn.x",
        "errnoLocation.x",
        "atoi.x",
        "time.x",
        "const2mut.x",
        "main.x",
        "stdio.x",
        "unsafe.x",
       ];
    std::env::set_var("PATH", format!("{}/lib/Rust", FOLDER));
    std::env::set_var("txl_rules", format!("{}/lib/Rust", FOLDER));
    let uuid = format!("{:?}.rs", Uuid::new_v4());
    for r in rules {
        let args = vec![
            "-q".to_string(),
            "-s".to_string(),
            "3000".to_string(),
            file.to_string(),
            format!("{}/lib/Rust/{}", FOLDER, r.to_string()),
        ];
        // if let Ok(output) = txl_rs::txl(args) {
        if let Ok(txl_command) = Command::new(r)
            .args(args)
            .stdout(Stdio::piped())
            .spawn() {
            if let Ok(output) = txl_command.wait_with_output() {
                std::fs::write(&uuid, format!("{:?}", output).as_bytes().as_ref()).ok();
                if let Ok(command) = Command::new("rustfmt")
                    .args([&uuid])
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    if let Ok(_output) = command.wait_with_output() {
                        if let Ok(s) = std::fs::read_to_string(file) {
                            std::fs::write(file, s).ok();
                        }
                    }
                }
            }
        } else {
            eprintln!("{r} has got a problem!");
        }
    }
    std::fs::remove_file(uuid).ok();
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
        std::fs::write("abc/Makefile", "main: main.c\n\tgcc -o main main.c\n\nclean::\n\trm -rf main compile_commands.json src Cargo.toml *.rs rust-toolchain txl10.8b.linux64").ok();
        std::env::set_current_dir(dir).ok();
        main();
        if let Ok(s) = std::fs::read_to_string("src/main.rs") {
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
    }
}
