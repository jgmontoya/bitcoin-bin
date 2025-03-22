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

    #[test]
    fn test_map_subcommand() {
        assert_eq!(map_subcommand("daemon"), "bitcoind");
        assert_eq!(map_subcommand("cli"), "bitcoin-cli");
        assert_eq!(map_subcommand("wallet"), "bitcoin-wallet");
    }

    #[test]
    fn test_route_command_not_found() {
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
        let request = CommandRequest {
            subcommand: "cli".to_string(),
            args: vec!["--version".into()],
        };
        let result = route_command(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_command_daemon() {
        let request = CommandRequest {
            subcommand: "daemon".to_string(),
            args: vec!["--version".into()],
        };
        let result = route_command(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_command_with_multiple_args() {
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
        let request = CommandRequest {
            subcommand: "cli".to_string(),
            args: vec!["--".into(), "-h".into()],
        };
        let result = route_command(&request);
        assert!(result.is_ok());
    }
}
