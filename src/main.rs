mod args;
mod strings;
mod utils;
mod formatting;

use args::ParsedArgs;
use utils::fatal;
use formatting::{ format_line, Variables, LongestValueSizes };
use clap::Parser;
use std::env::{ var_os, split_paths };
use std::fs;

#[cfg(not(any(unix, windows)))]
fn main() {
    compile_error!("This program is only valid for POSIX or Windows systems.");
}

#[cfg(any(unix, windows))]
fn main() {
    let args = ParsedArgs::parse();

    #[cfg(debug_assertions)]
    println!("args = {args:#?}");

    let Some(path_str) = var_os("PATH") else {
        fatal("No $PATH variable found.");
    };

    let mut exit_code: i32 = libc::EXIT_SUCCESS;

    let paths: Vec<_> = split_paths(&path_str).collect();
    let num_padding = paths.len().to_string().len();

    let mut lvs = LongestValueSizes::default();
    let mut lines: Vec<Variables> = vec![];

    for (i, path) in paths.iter().enumerate() {
        let line = args.enumerate.then(|| {
            // Weird fmt syntax.
            let line = if args.zero_padding {
                format!("{:0>x$}", i + 1, x = num_padding)
            } else {
                format!("{: >x$}", i + 1, x = num_padding)
            };
            if line.len() > lvs.line {
                lvs.line = line.len();
            }
            line
        });

        // I changed this statement like 15 times.
        let status = args.status_style
            .get_status_str(args.colorize)
            .map(|(ok_str, err_str)| {
                let status = match fs::metadata(path) {
                    Ok(m) if m.is_dir() => ok_str,
                    Ok(_) => {
                        exit_code = libc::ENOTDIR;
                        err_str
                    }
                    Err(e) => {
                        exit_code = e.raw_os_error().unwrap_or(1);
                        err_str
                    }
                };
                if status.len() > lvs.status {
                    lvs.status = status.len();
                }
                status
            });

        let path_display: String = match path.to_str() {
            Some(string) => {
                let path: String = string.chars()
                    .flat_map(|ch| ch.escape_default())
                    .collect();
                if path.len() > lvs.path {
                    lvs.path = path.len()
                }
                path
            }
            None => format!("{path:?}")
        };

        // let path_display = if path_display.is_empty() {
        //     "<empty>".to_string()
        // } else {
        //     path_display
        // };

        let description = None;

        lines.push(Variables {
            description,
            line,
            path: path_display,
            status
        });
    }

    for vars in lines {
        let output = format_line(&args.format, vars, &lvs);
        println!("{output}");
    }

    std::process::exit(exit_code);
}
