use crate::cli::CommandRequest;
use std::process::Command;

/// Maps a subcommand name to its corresponding binary name
fn map_subcommand(subcommand: &str) -> String {
    match subcommand {
        "daemon" => "bitcoind".to_string(),
        "cli" => "bitcoin-cli".to_string(),
        other => format!("bitcoin-{}", other),
    }
}

/// Routes a command request to the appropriate binary
pub fn route_command(request: &CommandRequest) -> Result<i32, String> {
    let binary_name = map_subcommand(&request.subcommand);

    // Spawn the process and wait for completion
    let status = Command::new(&binary_name)
        .args(&request.args)
        .status()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                format!(
                    "subcommand '{}' not recognized (no executable '{}' found in PATH)",
                    request.subcommand, binary_name
                )
            } else {
                format!("failed to execute '{}': {}", binary_name, e)
            }
        })?;

    Ok(status.code().unwrap_or(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    fn setup_test_path() -> (PathBuf, Option<String>) {
        // Get the absolute path to the test helpers directory
        let mut test_helpers_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_helpers_path.push("tests");
        test_helpers_path.push("helpers");

        // Store the original PATH
        let original_path = env::var_os("PATH").map(|p| p.to_string_lossy().to_string());

        // Add our helpers directory to the beginning of PATH
        let path_separator = if cfg!(windows) { ";" } else { ":" };
        let mut new_path = test_helpers_path.to_string_lossy().to_string();

        if let Some(path) = env::var_os("PATH") {
            new_path.push_str(path_separator);
            new_path.push_str(&path.to_string_lossy());
        }

        // Set the new PATH
        unsafe {
            env::set_var("PATH", &new_path);
        }

        (test_helpers_path, original_path)
    }

    fn restore_path(original_path: Option<String>) {
        unsafe {
            if let Some(path) = original_path {
                env::set_var("PATH", path);
            } else {
                env::remove_var("PATH");
            }
        }
    }

    struct TestPathGuard {
        original_path: Option<String>,
    }

    impl TestPathGuard {
        fn new() -> Self {
            let (_, original_path) = setup_test_path();
            Self { original_path }
        }
    }

    impl Drop for TestPathGuard {
        fn drop(&mut self) {
            restore_path(self.original_path.take());
        }
    }

    #[test]
    fn test_map_subcommand() {
        assert_eq!(map_subcommand("daemon"), "bitcoind");
        assert_eq!(map_subcommand("cli"), "bitcoin-cli");
        assert_eq!(map_subcommand("wallet"), "bitcoin-wallet");
    }

    #[test]
    fn test_route_command_not_found() {
        let _guard = TestPathGuard::new();
        let request = CommandRequest {
            subcommand: "nonexistent".to_string(),
            args: vec![],
        };
        let result = route_command(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not recognized"));
    }

    #[test]
    fn test_route_command_cli() {
        let _guard = TestPathGuard::new();
        let request = CommandRequest {
            subcommand: "cli".to_string(),
            args: vec!["--version".into()],
        };
        let result = route_command(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_command_daemon() {
        let _guard = TestPathGuard::new();
        let request = CommandRequest {
            subcommand: "daemon".to_string(),
            args: vec!["--version".into()],
        };
        let result = route_command(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_command_with_multiple_args() {
        let _guard = TestPathGuard::new();
        let request = CommandRequest {
            subcommand: "cli".to_string(),
            args: vec![
                "getblock".into(),
                "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f".into(), // Genesis block
            ],
        };
        let result = route_command(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_command_with_double_dash() {
        let _guard = TestPathGuard::new();
        let request = CommandRequest {
            subcommand: "cli".to_string(),
            args: vec!["--".into(), "-h".into()],
        };
        let result = route_command(&request);
        assert!(result.is_ok());
    }
}
