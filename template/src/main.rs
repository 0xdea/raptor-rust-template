//! main.rs

// Standard library imports
use std::path::Path;
use std::{env, process};

// External crate imports

// Internal crate imports

// const NAME: type = ...;

// static NAME: type = ...;

const PROGRAM: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{PROGRAM} {VERSION} - {{short-desc}}");
    println!("Copyright (c) {{year}} Marco Ivaldi <raptor@0xdeadbeef.info>");
    println!();

    // Parse command line arguments
    let args = env::args().collect::<Vec<_>>();

    let prog = Path::new(&args[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or(PROGRAM);

    let action = match args.len() {
        1 => "default",
        2 => &args[1].clone(),
        _ => "-",
    };
    if action.starts_with('-') {
        usage(prog);
    }

    // Let's do it
    match {{project-name}}::run(action) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("[!] Error: {err:#}");
            process::exit(1);
        }
    }
}

/// Print usage information and exit
fn usage(prog: &str) {
    println!("Usage:");
    println!("{prog} TODO");
    println!("\nExamples:");
    println!("{prog}");
    println!("{prog} TODO");

    process::exit(0);
}
