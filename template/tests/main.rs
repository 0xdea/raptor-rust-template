//! Integration tests driving the compiled `{{project-name}}` binary end-to-end to exercise `run`'s functionality.

#![expect(clippy::expect_used, reason = "tests can use `expect`")]

/// Absolute path to the compiled `{{project-name}}` binary under test.
const BIN: &str = env!("CARGO_BIN_EXE_{{project-name}}");

#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};

    use super::BIN;

    /// Asserts that `stderr` mentions `expected_message`, given verbose mode was enabled for the run.
    fn assert_stderr_contains(stderr: &str, expected_message: &str) {
        assert!(
            stderr.contains(expected_message),
            "expected stderr to contain {expected_message:?}, got: {stderr:?}"
        );
    }

    #[test]
    fn run_without_a_tty_fails() {
        let output = Command::new(BIN)
            .arg("-v")
            .stdin(Stdio::null())
            .output()
            .expect("failed to spawn the binary under test");

        assert!(
            !output.status.success(),
            "expected the binary to fail without a controlling tty"
        );

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_stderr_contains(&stderr, "stdin does not refer to a terminal");
    }
}
