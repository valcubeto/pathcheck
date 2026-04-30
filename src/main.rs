mod args;
mod strings;
mod utils;
mod formatting;

#[cfg(not(any(unix, windows)))]
fn main() {
    compile_error!("This program is only valid for POSIX or Windows systems.");
}

use crate::args::ProgramFlags;
use crate::utils::{ fatal, visible_len };
use crate::formatting::{ format_line, Variables, LongestValueSizes };
use std::env::{ var_os, split_paths };
use std::ffi::OsStr;
use std::collections::HashSet;
use std::fs;
use clap::Parser;
use faccess::PathExt;

#[cfg(any(unix, windows))]
fn main() {
    let args = ProgramFlags::parse();

    #[cfg(debug_assertions)]
    println!("args = {args:#?}");

    let Some(path_str) = var_os("PATH") else {
        fatal("No PATH variable found.");
    };

    let mut exit_code: i32 = libc::EXIT_SUCCESS;

    let paths: Vec<_> = split_paths(&path_str).collect();

    // Line numbers start from 1
    let num_padding = (paths.len() + 1).to_string().len();

    let path_label = "Path".to_string();
    let line_label = "Index".to_string();
    let status_label = "Status".to_string();
    let description_label = "Description".to_string();

    let mut lvs = if args.header {
        LongestValueSizes {
            path: path_label.len(),
            line: line_label.len(),
            status: status_label.len(),
            description: description_label.len()
        }
    } else {
        LongestValueSizes {
            line: num_padding,
            ..LongestValueSizes::default()
        }
    };

    let mut valid_paths = 0;
    
    let status_set = args::get_repr(
        args.status_style.filter(|_| !args.no_status)
    );

    let mut lines: Vec<Variables> = vec![];

    let mut seen_paths = HashSet::<&OsStr>::new();

    for (i, path) in paths.iter().enumerate() {
        let i = i + 1;

        let is_empty = path.as_os_str().is_empty();
        let is_repeated = if is_empty {
            false
        } else {
            !seen_paths.insert(path.as_os_str())
        };

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

        let line = args.enumerate.then(|| {
            if args.zero_padding {
                // Weird fmt syntax.
                format!("{i:0>x$}", x = num_padding)
            } else {
                i.to_string()
            }
        });

        let mut description: String = String::new();
        // I changed this statement like 20 times.
        let status = status_set.clone().map(|set| {
            if is_empty || is_repeated {
                description = if is_empty { "Empty" } else { "Repeated" }.to_owned();
                return utils::yellow(set.warning, args.colorize);
            }
            match fs::metadata(path) {
                Ok(m) if !m.is_dir() => {
                    exit_code = libc::ENOTDIR;
                    description = "Not a directory".to_owned();
                    utils::red(set.error, args.colorize)
                }
                Err(e) => {
                    exit_code = e.raw_os_error().unwrap_or(1);
                    description = format!("{e}");
                    utils::red(set.error, args.colorize)
                }
                Ok(_) => {
                    if path.readable() && path.executable() {
                        valid_paths += 1;
                        description = "A directory".to_owned();
                        utils::green(set.ok, args.colorize)
                    } else {
                        description = "Not enough permissions".to_owned();
                        utils::red(set.error, args.colorize)
                    }
                }
            }
        })
        .inspect(|status| {
            let status_len = visible_len(status);
            if status_len > lvs.status {
                lvs.status = status_len;
            }
            let description_len = visible_len(&description);
            if description_len > lvs.description {
                lvs.description = description_len;
            }
        });

        lines.push(Variables {
            description: args.descriptions.then_some(description),
            line,
            path: path_display,
            status
        });
    }

    if args.header {
        lines.insert(0, Variables {
            path: path_label,
            line:
                args.enumerate
                    .then_some(line_label),
            status:
                (args.status_style.is_some()
                 && !args.no_status)
                    .then_some(status_label),
            description:
                args.descriptions
                    .then_some(description_label),
        });
    }

    for vars in lines {
        let output = format_line(&args.format, vars, &lvs);
        println!("{output}");
    }

    if args.footer && !args.no_status {
        println!();
        println!("{valid_paths} valid paths.");
    }

    std::process::exit(if args.succeed { 0 } else { exit_code });
}
