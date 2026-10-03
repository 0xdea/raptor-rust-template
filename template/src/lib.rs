#![doc = env!("CARGO_PKG_DESCRIPTION")]
#![doc = ""]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/0xdea/{{project-name}}/master/.img/logo.png"
)]

// Standard library imports.
use std::ffi::OsStr;

// External crate imports.

// Internal crate imports.

// Modules and public modules.

// Public re-exports (next to the modules they expose).

// Macros (their scope is textual, so they come before any code that uses them).

// const NAME: type = ...;

// static NAME: type = ...;

// type Name = u32;

// Error types.

// Traits (before the types that implement them).

// Structs and enums, each followed by its impl blocks: inherent impl blocks,
// impl blocks with constraints, trait impl blocks (std, ext, int). Inside impl
// blocks: associated constants, constructors, other associated functions, then
// methods (getters and setters first).

// Free functions, public ones first.

/// Dispatches `action` to the function implementing it.
///
/// This could also be moved to main.rs and made private.
///
/// # Errors
///
/// Returns an error if `action` is empty or if the selected action fails.
pub fn run(action: &OsStr) -> anyhow::Result<()> {
    anyhow::ensure!(!action.is_empty(), "empty action");

    // TODO: dispatch to the function implementing `action`.
    Ok(())
}

// Doc comment template:
//
// /// Short explanation of what the item does.
// ///
// /// Short explanation of return values with [`link1`] or
// /// [`link2`](Link::Example2) where appropriate.
// ///
// /// [`link1`]: Link::Example1
// ///
// /// # Errors
// ///
// /// Short explanation of errors and their possible causes.
// ///
// /// # Examples
// ///
// /// Basic usage:
// /// ```
// /// # fn main() -> anyhow::Result<()> {
// /// todo!();
// /// # Ok(())
// /// # }
// /// ```
// ///
// /// More explanations and code examples in case some specific cases have to
// /// be explained in detail.

// Other functions ...

#[cfg(test)]
#[expect(clippy::panic_in_result_fn, reason = "panics are allowed in test code")]
mod tests {
    use super::*;

    /// Action accepted by `run`.
    const ACTION: &str = "default";

    #[test]
    fn run_accepts_a_non_empty_action() -> anyhow::Result<()> {
        run(OsStr::new(ACTION))
    }

    #[test]
    fn run_rejects_an_empty_action() -> anyhow::Result<()> {
        let Err(err) = run(OsStr::new("")) else {
            anyhow::bail!("an empty action should be rejected");
        };
        assert_eq!(err.to_string(), "empty action", "unexpected error message");

        Ok(())
    }
}
