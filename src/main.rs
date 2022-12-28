use flate2::read::GzDecoder;
use jwalk::WalkDir;
use std::{
    path::Path,
    process::{Command, Stdio},
};
use tar::Archive;
pub fn is_file_with_ext(p: &Path, file_ext: &str) -> bool {
    let ext = match p.extension() {
        Some(e) => e,
        None => return false,
    };
    ext.to_string_lossy() == file_ext
}

fn main() {
    if !std::path::Path::new("Cargo.toml").exists() {
        // This is a C project
        if !std::path::Path::new("compile_commands.json").exists() {
            if !std::path::Path::new("Makefile").exists()
                && !std::path::Path::new("makefile").exists()
                && !std::path::Path::new("configure").exists()
                && !std::path::Path::new("configure.ac").exists()
            {
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
                std::fs::write("Makefile", format!("main: {}\n\tgcc -o main {}\n\n.c.o: \n\tgcc -c $<\n\n.cpp.o: \n\tg++ -c $<\n\nclean::\n\trm -rf Makefile main c2rust crusts compile_commands.json txl10.8b.linux64 txl10.8b.macosx64 Cargo.lock target", obj, obj)).ok();
            }
            if !std::path::Path::new("Makefile").exists()
                && !std::path::Path::new("configure").exists()
                && std::path::Path::new("configure.ac").exists()
            {
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
            if !std::path::Path::new("Makefile").exists()
                && std::path::Path::new("configure").exists()
            {
                if let Ok(command) = Command::new("./configure").stdout(Stdio::piped()).spawn() {
                    if let Ok(output) = command.wait_with_output() {
                        println!("{:?}", output);
                    }
                }
            }
            if std::path::Path::new("Makefile").exists() {
                if let Ok(bear_version) = Command::new(BEAR)
                    .args(["--version"])
                    .stdout(Stdio::piped())
                    .spawn() 
                {
                    if let Ok(output) = bear_version.wait_with_output() {
                        let s = String::from_utf8_lossy(&output.stdout);
                        if s.contains("bear 2.4.3") {
                            if let Ok(command) = Command::new(BEAR)
                                .args(["make"])
                                .stdout(Stdio::piped())
                                .spawn()
                            {
                                if let Ok(output) = command.wait_with_output() {
                                    println!("{}", String::from_utf8_lossy(&output.stdout));
                                } 
                            }
                        } else {
                            if let Ok(command) = Command::new(BEAR)
                                .args(BEAR_ARGS)
                                .stdout(Stdio::piped())
                                .spawn()
                            {
                                if let Ok(output) = command.wait_with_output() {
                                    println!("{}", String::from_utf8_lossy(&output.stdout));
                                } 
                            }
                        }
                    }
                } else {
                        if let Ok(command) = Command::new("intercept-build")
                            .args(["make"])
                            .stdout(Stdio::piped())
                            .spawn()
                        {
                            if let Ok(output) = command.wait_with_output() {
                                println!("{}", String::from_utf8_lossy(&output.stdout));
                            } 
                        } else {
                            panic!("Please install bear or scan-build\n");
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
                    println!("{}", String::from_utf8_lossy(&output.stdout));
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
    crusts();
}

extern crate reqwest;

#[cfg(target_os = "macos")]
const FOLDER: &str = "txl10.8b.macosx64";
#[cfg(target_os = "macos")]
const URL: &str = "http://bertrust.s3.amazonaws.com/crusts-macosx.tar.gz";
#[cfg(target_os = "macos")]
const BEAR: &str = "bear";
#[cfg(target_os = "macos")]
const BEAR_ARGS: [&str; 2] = ["--", "make"];
#[cfg(target_os = "linux")]
const FOLDER: &str = "txl10.8b.linux64";
// const FOLDER: &str = "/usr/local";
#[cfg(target_os = "linux")]
const URL: &str = "http://bertrust.s3.amazonaws.com/crusts-linux.tar.gz";
#[cfg(target_os = "linux")]
const BEAR: &str = "bear";
#[cfg(target_os = "linux")]
const BEAR_ARGS: [&str; 2] = ["--", "make"];
#[cfg(target_os = "windows")]
const FOLDER: &str = "Txl108bwin64";
#[cfg(target_os = "windows")]
const URL: &str = "http://bertrust.s3.amazonaws.com/crusts-windows.tar.gz";
#[cfg(target_os = "windows")]
const BEAR: &str = "intercept-build";
#[cfg(target_os = "windows")]
const BEAR_ARGS: [&str; 1] = ["make"];

fn crusts() {
    if !std::path::Path::new(&format!("{}/lib/Rust/unsafe.x", FOLDER)).exists() {
        println!("downloading txl ... ");
        std::fs::create_dir(&format!("{}/lib/Rust", FOLDER)).ok();
        if let Ok(resp) = reqwest::blocking::get(URL) {
            if let Ok(bytes) = resp.bytes() {
                let tar = GzDecoder::new(&bytes[..]);
                let mut archive = Archive::new(tar);
                archive.unpack(format!("{}/lib", FOLDER)).ok();
            } else {
                eprintln!("Couldn't download, please check your network connection.");
            }
            println!("downloaded ... ");
        } else {
            eprintln!("Couldn't download, please check your network connection.");
            return;
        }
    }
    let rules = vec![
        "formalizeCode.x",
        "varTypeNoBounds.x",
        "null.x",
        "array.x",
        "fn.x",
        "errnoLocation.x",
        "atoi.x",
        "time.x",
        "const2mut.x",
        // "main.x",
        "stdio.x",
        "unsafe.x",
    ];
    std::env::set_var("PATH", format!("{}/lib/Rust:/usr/local/cargo/bin:{:?}", FOLDER, std::env::var("PATH")));
    for r in rules {
        println!("applying {r}...");
        WalkDir::new(".").sort(true).into_iter().for_each(|entry| {
            if let Ok(e) = entry {
                let p = e.path();
                if !is_file_with_ext(&p, "rs") {
                    return;
                }
                let file = &format!("{}", &p.into_os_string().to_string_lossy());
                let _txl_command = Command::new(r)
                    .args(vec![
                        file.to_string(),
                        "-".to_string(),
                        format!("{}/lib/Rust", FOLDER)])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed txl command"); 
                let _rustfmt = Command::new("rustfmt")
                    .stdin(_txl_command.stdout.unwrap())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed rustfmt command"); 
                let output = _rustfmt.wait_with_output().expect("failed to write to stdout");
                std::fs::write(&file, &output.stdout).expect("can't write to the file");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_unsafe() {
        let dir = std::path::Path::new("abc");
        if dir.exists() {
            std::fs::remove_dir_all(dir).ok();
        }
        std::fs::create_dir_all(dir).ok();
        std::fs::write(
            "abc/main.rs",
            r#"
use libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn add_value(mut p: *mut tvm_program_t, val: i32) -> *mut i32 {
        (*p).values = realloc(
            (*p).values as *mut libc::c_void,
            (::core::mem::size_of::<*mut i32>() as u64)
                .wrapping_mul(((*p).num_values + 1i32) as u64),
        ) as *mut *mut i32;
        let ref mut fresh7 = *((*p).values).offset((*p).num_values as isize);
        *fresh7 = calloc(1, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        **((*p).values).offset((*p).num_values as isize) = val;
        let fresh8 = (*p).num_values;
        (*p).num_values = (*p).num_values + 1;
        return *((*p).values).offset(fresh8 as isize);
}
"#,
        )
        .ok();
        std::env::set_current_dir(dir).ok();
        crusts();
        std::env::set_current_dir(std::env::current_dir().unwrap().parent().unwrap()).ok();
        if let Ok(s) = std::fs::read_to_string("abc/main.rs") {
            insta :: assert_snapshot! (s, @ r###"
            use libc;
            extern "C" {
                fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
            }
            #[no_mangle]
            pub extern "C" fn add_value(mut p: *mut tvm_program_t, val: i32) -> *mut i32 {
                unsafe {
                    (*p).values = realloc(
                        (*p).values as *mut libc::c_void,
                        (::core::mem::size_of::<*mut i32>() as u64)
                            .wrapping_mul(((*p).num_values + 1i32) as u64),
                    ) as *mut *mut i32;
                    let ref mut fresh7 = *((*p).values).offset((*p).num_values as isize);
                    *fresh7 = calloc(1, ::core::mem::size_of::<i32>() as u64) as *mut i32;
                    **((*p).values).offset((*p).num_values as isize) = val;
                    let fresh8 = (*p).num_values;
                    (*p).num_values = (*p).num_values + 1;
                    return *((*p).values).offset(fresh8 as isize);
                }
            }
            "###);
        }
    }

    fn test_crusts() {
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
    printf("Hello, world!\n");
    return 0;
}
"#,
        )
        .ok();
        std::fs::write("abc/Makefile", "main: main.c\n\tgcc -o main main.c\n\nclean::\n\trm -rf main compile_commands.json src Cargo.toml *.rs rust-toolchain rust-toolchain.toml txl10.8b.linux64 txl10.8b.macosx64 Cargo.lock target").ok();
        std::env::set_current_dir(dir).ok();
        main();
        if let Ok(s) = std::fs::read_to_string("src/main.rs") {
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
            use c2rust_out::*;
            extern "C" {}
            fn main_0() -> i32 {
                print!("Hello, world!\n");
                return 0;
            }

            pub fn main() {
                ::std::process::exit(main_0() as i32);
            }
            "###);
        }
        std::env::set_current_dir(std::env::current_dir().unwrap().parent().unwrap()).ok();
    }

    #[test]
    fn test_main() {
        // test_unsafe();
        test_crusts();
    }
}
