use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub fn exists_dir(dir: &str) -> bool {
    let path = Path::new(&dir);
    if !path.is_dir() {
        eprintln!("\"{}\" does not exist", dir);
        return false;
    }
    return true;
}

fn replace(file: &str, args: &super::Args) -> String {
    if args.dst.len() != 1 {
        return file.replace(&args.src, &args.dst);
    } else {
        let mut result = String::from(file);
        for c in args.src.chars() {
            result = result.replace(c, &args.dst);
        }
        return result;
    }
}

fn handle_file(path: &PathBuf, args: &super::Args, regex: &Regex) {
    // extract path elements (dir, file, ext)
    let filename = match path.file_name() {
        Some(result) => result,
        None => {
            eprintln!("Error: cannot extract filename from {:?}", path);
            return;
        }
    };
    let filename = Path::new(filename);
    let file = filename.file_stem().unwrap().to_str().unwrap();
    let ext = filename.extension();
    let dir = path.parent();

    // do the replacement
    if args.verbose {
        println!("# Computing '{}'...", file);
    }
    let dst: String;
    if args.replace {
        dst = replace(&file, &args);
    } else {
        dst = regex.replace_all(file, &args.dst[..]).to_string();
    }

    // if replacement occurred
    if file != dst {
        // build source and destination
        let mut target = PathBuf::new();
        if args.targetdir != super::DEFAULTDIR {
            target.push(&args.targetdir);
        } else {
            target.push(match dir {
                Some(result) => result.to_str().unwrap(),
                None => "/",
            });
        }
        target.push(&dst);
        target.set_extension(match ext {
            Some(result) => result,
            None => OsStr::new(""),
        });
        // safe to unwrap since it comes from the filesystem
        let src = path.to_str().unwrap();
        let dst = match target.to_str() {
            Some(result) => result,
            None => {
                eprintln!("Error: invalid destination {:?}", dst);
                return;
            }
        };

        // do the actual move / copy or pretend
        if args.pretend {
            if args.copy {
                println!("cp \"{}\" \"{}\"", src, dst);
            } else {
                println!("mv \"{}\" \"{}\"", src, dst);
            }
        } else {
            if args.copy {
                if args.verbose {
                    println!("# \"{}\" ==> \"{}\"", src, dst);
                }
                #[cfg(not(debug_assertions))]
                match fs::copy(src, dst) {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("Error: cannot copy to {} because of {:?}", dst, error);
                        return;
                    }
                };
            } else {
                if args.verbose {
                    println!("# \"{}\" --> \"{}\"", src, dst);
                }
                #[cfg(not(debug_assertions))]
                match fs::rename(src, dst) {
                    Ok(_) => (),
                    Err(error) => {
                        eprintln!("Error: cannot move to {} because of {:?}", dst, error);
                        return;
                    }
                };
            }
        }
    } else {
        if args.verbose {
            println!("# No change: skipping '{}'...", file);
        }
    }
}

fn handle_dir(rootdir: &str, args: &super::Args, regex: &Regex) {
    let subdirs = match fs::read_dir(Path::new(&rootdir)) {
        Ok(result) => result,
        Err(error) => {
            if error.kind() == ErrorKind::PermissionDenied {
                eprintln!("Skipping \"{}\": permission denied", rootdir);
            } else {
                eprintln!(
                    "Unexpected error \"{:?}\" while reading subdirs of \"{}\"",
                    error, rootdir
                );
            }
            return;
        }
    };
    if args.verbose {
        println!("# Reading '{}'...", rootdir);
    }
    for entry in subdirs {
        let entry = match entry {
            Ok(result) => result,
            Err(error) => {
                eprintln!(
                    "Unexpected error \"{:?}\" while browsing entries of \"{}\"",
                    error, rootdir
                );
                continue;
            }
        };
        let path = entry.path();
        let s = match path.to_str() {
            Some(result) => result,
            None => {
                eprintln!("Unexpected error while converting path \"{:?}\"", path);
                continue;
            }
        };
        if path.is_dir() {
            if args.recurse {
                handle_dir(s, args, regex);
            }
        } else {
            handle_file(&path, args, regex);
        }
    }
}

pub fn process_dir(args: &super::Args) {
    // we know args.src compiles since we went through validate_args
    let regex = Regex::new(&args.src).unwrap();
    if args.pretend {
        println!("#!/bin/sh");
        println!("");
    }
    handle_dir(&args.rootdir, args, &regex);
}
