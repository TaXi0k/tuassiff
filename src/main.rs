use std::{env, path::{Path, PathBuf}, process};

use tuassiff;

// =======================================

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const BRIGHT_BLUE: &str = "\x1b[94m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

// =======================================

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_usage();
        process::exit(2)
    }

    let source = PathBuf::from(&args[2]);
    let target = PathBuf::from(&args[3]);

    match args[1].to_lowercase().as_str() {
        "encode" => {
            let img = image::open(source).unwrap_or_else(
                |e| {
                    eprintln!("{RED}{BOLD}ERROR:{RESET}{RED} Failed to open source image with error:\n{RESET}{e}");
                    process::exit(1);
                }
            );
            let data: tuassiff::Data = tuassiff::Data::from_img(img);
            data.save(&target).unwrap_or_else(
                |e| {
                    eprintln!("{RED}{BOLD}ERROR:{RESET}{RED} Failed to save converted image with error:\n{RESET}{e}");
                    process::exit(1);
                }
            )
        },
        "decode" => {
            let data: tuassiff::Data = tuassiff::Data::open(&source).unwrap_or_else(
                |e| {
                    eprintln!("{RED}{BOLD}ERROR:{RESET}{RED} Failed to fetch data from source image with error:\n{RESET}{e}");
                    process::exit(1);
                }
            );
            let img = data.to_img();
            img.save(&target).unwrap_or_else(
                |e| {
                    eprintln!("{RED}{BOLD}ERROR:{RESET}{RED} Failed to save converted image with error:\n{RESET}{e}");
                    process::exit(1);
                }
            )
        },
        _ => {
            print_usage();
            process::exit(2)
        },
    }
}

// =======================================

fn print_usage() {
    eprintln!("{BOLD}{RED}Usage: {RESET}{YELLOW}tuassiff {RESET}{DIM}<{RESET}encode{DIM}|{RESET}decode{DIM}> <{RESET}source{DIM}> <{RESET}target{DIM}>{RESET}");
    eprintln!("{BRIGHT_BLUE} * {GREEN}encode{RESET} {DIM}-{RESET} convert {BOLD}PNG{RESET} file to {BOLD}TUASSIFF{RESET} file");
    eprintln!("{BRIGHT_BLUE} * {GREEN}decode{RESET} {DIM}-{RESET} convert {BOLD}TUASSIFF{RESET} file to {BOLD}PNG{RESET} file");
}