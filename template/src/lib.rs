//!
//! {{project-name}} - {{short-desc}}
//! Copyright (c) {{year}} Marco Ivaldi <raptor@0xdeadbeef.info>
//!
//! > "It's important to be quotable."  
//! >  
//! > -- Halvar Flake  
//!
//! {{long-desc}}
//!
//! ## Features
//! * TODO
//!
//! ## Blog post
//! * TODO
//!
//! ## See also
//! * TODO
//!
//! ## Installing
//! The easiest way to get the latest release is via [crates.io](https://crates.io/crates/{{project-name}}):
//! ```sh
//! $ cargo install {{project-name}}
//! ```
//!
//! ## Compiling
//! Alternatively, you can build from [source](https://github.com/0xdea/{{project-name}}):
//! ```sh
//! $ git clone https://github.com/0xdea/{{project-name}}
//! $ cd {{project-name}}
//! $ cargo build --release
//! ```
//!
//! ## Usage
//! Run {{project-name}} as follows:
//! ```sh
//! TODO
//! ```
//!
//! ## Examples
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! TODO:
//! ```sh
//! TODO
//! ```
//!
//! ## Tested on
//! * TODO
//!
//! ## Changelog
//! * <https://github.com/0xdea/{{project-name}}/blob/master/CHANGELOG.md>
//!
//! ## Credits
//! * TODO
//!
//! ## TODO
//! * TODO
//!

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/0xdea/{{project-name}}/master/.img/logo.png"
)]

// Standard library imports

// External crate imports

// Internal imports

// const NAME: type = ...;

// static NAME: type = ...;

/// Dispatch to function implementing the selected action
pub fn run(action: &str) -> anyhow::Result<()> {
    todo!();
    /*
    match action {
        "action1" => func1()?,
        _ => func2(action)?,
    }

    Ok(())
    */
}

// Other functions ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
