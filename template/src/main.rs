//! main.rs.

// Standard library imports.
use std::env;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::ExitCode;

// External crate imports.

// Internal crate imports.

// const NAME: type = ...;

// static NAME: type = ...;

/// Package name.
const PROGRAM: &str = env!("CARGO_PKG_NAME");
/// Package version.
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Package authors.
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> ExitCode {
    /*
    // Handle verbose output with a macro.
    let verbose = env::args_os().nth(1).is_some();
    macro_rules! vprintln {
        ($($arg:tt)*) => {
            if verbose {
                eprintln!($($arg)*);
            }
        };
    }

    vprintln!("{PROGRAM} {VERSION} - {{short-desc}}");
    vprintln!("Copyright (c) {{year}} {AUTHORS}");
    vprintln!();

    match {{project-name}}::run() {
        Ok(()) => {
            {{project-name}}::clear_terminal(LINES);
            ExitCode::SUCCESS
        }
        Err(err) => {
            vprintln!("[!] Error: {err:#}");
            ExitCode::FAILURE
        }
    }
    */

    eprintln!("{PROGRAM} {VERSION} - {{short-desc}}");
    eprintln!("Copyright (c) {{year}} {AUTHORS}");
    eprintln!();

    // Parse command line arguments.
    let mut args = env::args_os();
    let argv0 = args.next().unwrap_or_else(|| OsString::from(PROGRAM));
    let is_help = |a: &OsStr| a == OsStr::new("-h") || a == OsStr::new("--help");

    let prog = Path::new(&argv0)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(PROGRAM);

    let action = match (args.next(), args.next()) {
        (None, _) => OsString::from("default"),
        (Some(arg), None) if !is_help(&arg) => arg,
        _ => return usage(prog),
    };

    // Let's do it.
    match {{project-name}}::run(action) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[!] Error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

/// Prints usage information and exits.
fn usage(prog: &str) -> ExitCode {
    eprintln!("Usage:");
    eprintln!("{prog} TODO");
    eprintln!("\nExamples:");
    eprintln!("{prog}");
    eprintln!("{prog} TODO");

    ExitCode::FAILURE
}
