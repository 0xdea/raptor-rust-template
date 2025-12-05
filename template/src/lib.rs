#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/0xdea/{{project-name}}/master/.img/logo.png"
)]

// Standard library imports

// External crate imports

// Internal crate imports

// Public re-exports

// Modules and public modules

// const NAME: type = ...;

// static NAME: type = ...;

// Error types

// Structs, enums, impls blocks, impl blocks with constraints, trait impl blocks (std, ext, int)
// Associated constants, associated functions, constructors, getters/setters, anything else

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

/// Short explanation of what the item does.
///
/// ## Errors
///
/// Short explanation of return values and errors with [`link`] where appropriate.
///
/// [`link`]: LinkExample
///
/// ## Examples
///
/// Basic usage:
/// ```
/// # fn main() -> anyhow::Result<()> {
/// todo!();
/// # Ok(())
/// # }
/// ```
///

// Other functions ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Arrange
        // Act
        // Assert
        assert_eq!(2 + 2, 4, "It should work!");
    }
}
