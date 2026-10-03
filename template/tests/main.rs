//! Integration tests driving the compiled `{{project-name}}` binary.

#![expect(clippy::expect_used, reason = "tests can use `expect`")]

#[cfg(test)]
mod tests {
    use std::process::{Command, Output};

    /// Absolute path to the compiled `{{project-name}}` binary under test.
    const BIN: &str = env!("CARGO_BIN_EXE_{{project-name}}");

    /// Runs the binary under test with `args` and returns its output.
    fn run_bin(args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .output()
            .expect("failed to spawn the binary under test")
    }

    /// Asserts that `stderr` of `output` contains `expected`.
    fn assert_stderr_contains(output: &Output, expected: &str) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(expected),
            "expected stderr to contain {expected:?}, got: {stderr:?}"
        );
    }

    #[test]
    fn no_arguments_runs_the_default_action() {
        let output = run_bin(&[]);

        assert!(output.status.success(), "the default action should succeed");
    }

    #[test]
    fn help_flag_prints_usage_and_fails() {
        for flag in ["-h", "--help"] {
            let output = run_bin(&[flag]);

            assert!(!output.status.success(), "{flag} should fail");
            assert_stderr_contains(&output, "Usage:");
        }
    }

    #[test]
    fn empty_action_reports_an_error() {
        let output = run_bin(&[""]);

        assert!(!output.status.success(), "an empty action should fail");
        assert_stderr_contains(&output, "[!] Error: empty action");
    }
}
