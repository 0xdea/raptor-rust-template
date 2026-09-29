#![doc = env!("CARGO_PKG_DESCRIPTION")]
#![doc = ""]
#![cfg_attr(doc, doc = include_str!("../README.md"))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/0xdea/{{project-name}}/master/.img/logo.png"
)]

// Standard library imports.

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
// blocks: associated constants, constructors, other associated functions,
// getters/setters, anything else.

// Free functions, public ones first.

/// Dispatches to function implementing the selected action.
///
/// This could also be moved to main.rs and made private.
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

/// Short explanation of what the item does.
///
/// Short explanation of return values with [`link1`] or
/// [`link2`](Link::Example2) where appropriate.
///
/// [`link1`]: Link::Example1
///
/// # Errors
///
/// Short explanation of errors and their possible causes.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # fn main() -> anyhow::Result<()> {
/// todo!();
/// # Ok(())
/// # }
/// ```
///
/// More explanations and code examples in case some specific cases have to be
/// explained in detail.
///

// Other functions ...

#[cfg(test)]
mod tests {
    use super::*;

    // Test constants.
    const EXPECTED_SUM: i32 = 4;
    const EXPECTED_RESULT: &str = "Expected result string";

    #[test]
    fn it_works() {
        // Arrange.
        // Act.
        // Assert.
        assert_eq!(2 + 2, EXPECTED_SUM, "It should work!");
    }
}
