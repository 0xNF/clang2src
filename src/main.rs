pub mod generator_csharp;
pub mod generator_dart;
pub mod generator_go2;
pub mod lexer;
pub mod meta;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{path::Path, process::exit};
use uuid::Uuid;

use crate::lexer::{parse, tokenize};

fn main() {
    let args = Arguments::parse();

    let token_str = dump_clang_tokes(Path::new(&args.header_file)).unwrap();
    let tokens = tokenize(&token_str);
    let header = parse(tokens).unwrap();

    match args.cmd {
        SubCommand::CSharp {
            namespace,
            dll_location,
        } => {
            generator_csharp::generate(header, &namespace, &dll_location);
        }
        SubCommand::Go {
            package_name,
            ld_flags,
            header_file_location,
        } => {
            let res =
                generator_go2::generate(header, &package_name, &ld_flags, &header_file_location);
            if check_program_exists("gofmt") {
                if let Ok(val) = std::process::Command::new("echo")
                    .args(vec![res.to_owned(), "|".to_owned(), "gofmt".to_owned()])
                    .stdout(std::process::Stdio::piped())
                    .output()
                {
                    println!("{}", String::from_utf8(val.stdout).unwrap());
                    return;
                };
            }
            println!("{}", &res);
        }

        SubCommand::Dart {
            library_path,
            library_name,
        } => {
            let res = generator_dart::generate(header, &library_path, &library_name);
            if check_program_exists("dart") {
                let fpath = match save_temporary_output(&res, ".dart") {
                    Ok(p) => Some(p),
                    Err(e) => {
                        eprintln!("Failed to write temporary output for dart formatting {}", e);
                        None
                    }
                };

                if let Some(p) = fpath {
                    match std::process::Command::new("dart")
                        .args(vec![
                            "format",
                            "--fix",
                            // "-o",
                            // "show",
                            (p.as_os_str().to_str()).unwrap(),
                        ])
                        .stdout(std::process::Stdio::piped())
                        .output()
                    {
                        Ok(_) => match std::fs::read_to_string(&p) {
                            Ok(s) => {
                                println!("{}", s);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to run dart format: File could not be read: {}",
                                    e
                                );
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to run dart format: {}", e);
                        }
                    };

                    if let Err(e) = delete_temporary_output(&p) {
                        eprintln!("Failed to delete temporary file {}", e);
                    }
                }
            }
            // println!("{}", &res);
        }
    }
}

fn save_temporary_output(output: &str, extension: &str) -> Result<PathBuf, std::io::Error> {
    let file_name = &format!("{}.{}", Uuid::new_v4().to_string(), extension);
    let mut file = File::create(file_name)?;
    file.write(output.as_bytes())?;
    Ok(Path::new(file_name).to_path_buf())
}

fn delete_temporary_output(path: &PathBuf) -> Result<(), std::io::Error> {
    std::fs::remove_file(path)
}

fn check_program_exists(program_name: &str) -> bool {
    #[cfg(target_os = "windows")]
    return match std::process::Command::new("where")
        .args(vec![program_name])
        .stdout(std::process::Stdio::piped())
        .output()
    {
        Ok(val) => {
            if val.stdout.starts_with("INFO:".as_bytes()) {
                false
            } else {
                true
            }
        }
        Err(_) => false,
    };
    #[cfg(target_os = "macos")]
    return false;
    #[cfg(target_os = "linux")]
    return match std::process::Command::new("which")
        .args(vec![program_name])
        .stdout(std::process::Stdio::piped())
        .output()
    {
        Ok(val) => {
            if val.stdout.len() == 0 {
                false
            } else {
                true
            }
        }
        Err(_) => false,
    };
}

fn dump_clang_tokes(p: &Path) -> Result<String, ()> {
    if !p.exists() {
        eprintln!("No file found: {}", p.to_str().unwrap());
        exit(-1);
    }

    let c = std::process::Command::new("clang")
        .arg("-fsyntax-only")
        .arg("-Xclang")
        .arg("-dump-raw-tokens")
        .arg(p.to_owned())
        .output();
    match c {
        Err(e) => {
            eprintln!("Failed to generate clang tokens: {}", e);
            return Err(());
        }
        Ok(output) => {
            /* FYI(nf): Clang dumps output to stderr for some reason */
            // let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            return Ok(stderr);
        }
    }
}

#[derive(Parser)]
pub struct Arguments {
    /// Path to generated C header file (.h) to parse.
    ///
    /// Header file must not contain any function bodies or variable assignemnts.
    ///
    /// Is assumed to be generated solely by `Bindcgen`
    header_file: String,

    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    CSharp {
        /// C# Namespace of the generated file to use
        ///
        /// e.g., `namespace FFI {
        ///     public struct OAuthManager{}
        ///  [...]
        /// }`
        namespace: String,

        /// Location, relative to the runtime location of the final libary, where the dll
        /// will be loaded from. Should include the extension, e.g.,
        ///
        /// `some/relative/folder/liboauthtool.dll`
        dll_location: String,
    },

    Go {
        /// Name of the Go Package to place the generated bindings under
        package_name: String,
        /// Where to load the lib files relative to the go binary
        ld_flags: String,
        /// header file location relative to the building location of the go binary
        header_file_location: String,
    },

    Dart {
        /// Location of the folder containing DLLs/SO files
        library_path: String,
        /// Name of the lib file, without the extension
        library_name: String,
    },
}
