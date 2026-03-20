mod args;
mod strings;
mod utils;
mod formatting;

use std::env;
use std::fs;
use std::process::exit;
use std::path::PathBuf;
use args::Args;
use clap::Parser;

fn main() {

    let args = Args::parse();

    // Only print with dev profile.
    #[cfg(debug_assertions)]
    println!("{args:?}");

    let Some(path_str) = env::var_os("PATH") else {
        eprintln!("No $PATH variable found.");
        exit(libc::EINVAL);
    };

    let mut result = Ok(libc::EXIT_SUCCESS);

    let paths: Vec<PathBuf> = env::split_paths(&path_str).collect();
    let num_padding = paths.len().to_string().len();

    for (i, path) in paths.iter().enumerate() {
        let mut output = String::new();

        if args.enumerate {
            // Weird fmt syntax.
            if args.zero_padding {
                output.push_str(&format!("{i:0>x$}: ", x = num_padding));
            } else {
                output.push_str(&format!("{i: >x$}: ", x = num_padding));
            }
        }

        if !args.status_style.is_none() {            
            let status_text = args.status_style
                .get_status_str(args.colorize);

            if let Some((ok_str, err_str)) = status_text {
                // I changed this statement like 10 times.
                let status_str = match fs::metadata(path) {
                    Ok(m) if m.is_dir() => ok_str,
                    Ok(_) => {
                        result = Ok(libc::ENOTDIR);
                        err_str
                    }
                    Err(e) => {
                        result = Err(e);
                        err_str
                    }
                };

                output.push_str(&status_str);
                output.push(' ');
            }
        }

        let path_display: String = match path.to_str() {
            Some(string) =>
                string.chars()
                    .flat_map(|ch| ch.escape_default())
                    .collect(),
            None => format!("{path:?}")
        };

        if path_display.is_empty() {
            output.push_str("<empty>");
        } else {
            output.push_str(&path_display);
        }

        println!("{output}");
    }

    exit(result.unwrap_or_else(|e| e.raw_os_error().unwrap_or(1)))
}

#[cfg(not(any(unix, windows)))]
fn main() {
    compile_error!("This program is only valid for POSIX or Windows systems.");
}
