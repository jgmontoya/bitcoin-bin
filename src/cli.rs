use clap::{ArgMatches, Command, error::ErrorKind};
use std::env;

#[derive(Debug, PartialEq)]
pub struct CommandRequest {
    pub subcommand: String,
    pub args: Vec<String>,
}

pub fn build_cli() -> Command {
    Command::new("bitcoin")
        .about("Unified Bitcoin CLI router")
        .long_about("Unified Bitcoin CLI router. Use '--' to separate options for the underlying binary (e.g. 'bitcoin cli -- -h')")
        .version(env!("CARGO_PKG_VERSION"))
        .override_usage("bitcoin <COMMAND> [ARGS]...")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .external_subcommand_value_parser(clap::value_parser!(String))
        .subcommand(
            Command::new("cli")
                .about("Invoke bitcoin-cli (command-line RPC interface)")
                .arg(clap::Arg::new("args")
                    .num_args(0..)
                    .allow_hyphen_values(true)
                    .trailing_var_arg(true))
        )
        .subcommand(
            Command::new("daemon")
                .about("Start the Bitcoin daemon (bitcoind)")
                .arg(clap::Arg::new("args")
                    .num_args(0..)
                    .allow_hyphen_values(true)
                    .trailing_var_arg(true))
        )
}

pub fn parse_args() -> Result<CommandRequest, clap::Error> {
    let matches = build_cli().get_matches();
    parse_args_with_matches(matches)
}

pub fn parse_args_with_matches(matches: ArgMatches) -> Result<CommandRequest, clap::Error> {
    match matches.subcommand() {
        Some((cmd @ ("cli" | "daemon"), sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("args")
                .map(|v| v.map(|s| s.to_string()).collect())
                .unwrap_or_default();
            Ok(CommandRequest {
                subcommand: cmd.to_string(),
                args,
            })
        }
        Some((external_cmd, sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("")
                .map(|v| v.map(|s| s.to_string()).collect())
                .unwrap_or_default();
            Ok(CommandRequest {
                subcommand: external_cmd.to_string(),
                args,
            })
        }
        None => {
            // This should never happen due to subcommand_required(true)
            Err(clap::Error::raw(
                ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
                "No subcommand specified",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_cli() {
        let cmd = build_cli();
        assert_eq!(cmd.get_name(), "bitcoin");
        assert!(cmd.get_subcommands().any(|sc| sc.get_name() == "cli"));
        assert!(cmd.get_subcommands().any(|sc| sc.get_name() == "daemon"));
    }

    #[test]
    fn test_parse_args_cli() {
        let request = build_cli()
            .try_get_matches_from(vec!["bitcoin", "cli", "getblockcount"])
            .map(|matches| {
                let (subcommand, sub_matches) = matches.subcommand().unwrap();
                CommandRequest {
                    subcommand: subcommand.to_string(),
                    args: sub_matches
                        .get_many::<String>("args")
                        .map(|v| v.map(|s| s.into()).collect())
                        .unwrap_or_default(),
                }
            })
            .unwrap();
        assert_eq!(request.subcommand, "cli");
        assert_eq!(request.args, vec!["getblockcount"]);
    }

    #[test]
    fn test_parse_args_daemon() {
        let cmd = build_cli();
        let matches = cmd
            .try_get_matches_from(["bitcoin", "daemon", "--", "--testnet"])
            .unwrap();
        let request = parse_args_with_matches(matches).unwrap();
        assert_eq!(request.subcommand, "daemon");
        assert_eq!(request.args, vec!["--testnet"]);
    }

    #[test]
    fn test_parse_args_with_double_dash() {
        let request = build_cli()
            .try_get_matches_from(vec!["bitcoin", "cli", "--", "-h"])
            .map(|matches| {
                let (subcommand, sub_matches) = matches.subcommand().unwrap();
                CommandRequest {
                    subcommand: subcommand.to_string(),
                    args: sub_matches
                        .get_many::<String>("args")
                        .map(|v| v.map(|s| s.into()).collect())
                        .unwrap_or_default(),
                }
            })
            .unwrap();
        assert_eq!(request.subcommand, "cli");
        assert_eq!(request.args, vec!["-h"]);
    }

    #[test]
    fn test_parse_args_multiple_args() {
        let request = build_cli()
            .try_get_matches_from(vec![
                "bitcoin",
                "cli",
                "getblock",
                "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
            ])
            .map(|matches| {
                let (subcommand, sub_matches) = matches.subcommand().unwrap();
                CommandRequest {
                    subcommand: subcommand.to_string(),
                    args: sub_matches
                        .get_many::<String>("args")
                        .map(|v| v.map(|s| s.into()).collect())
                        .unwrap_or_default(),
                }
            })
            .unwrap();
        assert_eq!(request.subcommand, "cli");
        assert_eq!(
            request.args,
            vec![
                "getblock",
                "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
            ]
        );
    }

    #[test]
    fn test_parse_args_with_matches_direct() {
        let cmd = build_cli();
        let matches = cmd
            .try_get_matches_from(["bitcoin", "cli", "getblockcount"])
            .unwrap();
        let request = parse_args_with_matches(matches).unwrap();
        assert_eq!(request.subcommand, "cli");
        assert_eq!(request.args, vec!["getblockcount"]);
    }

    #[test]
    fn test_parse_args_with_matches_unknown() {
        let cmd = build_cli();
        let matches = cmd
            .try_get_matches_from(["bitcoin", "unknown", "arg1", "arg2"])
            .unwrap();
        let request = parse_args_with_matches(matches).unwrap();
        assert_eq!(request.subcommand, "unknown");
        assert_eq!(request.args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_args_with_matches_no_args() {
        let cmd = build_cli();
        let matches = cmd.try_get_matches_from(["bitcoin"]).unwrap_err();
        assert_eq!(
            matches.kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }

    #[test]
    fn test_parse_args_with_matches_help() {
        let cmd = build_cli();
        let matches = cmd.try_get_matches_from(["bitcoin", "--help"]).unwrap_err();
        assert_eq!(matches.kind(), ErrorKind::DisplayHelp);
    }

    #[test]
    fn test_parse_args_with_matches_version() {
        let cmd = build_cli();
        let matches = cmd
            .try_get_matches_from(["bitcoin", "--version"])
            .unwrap_err();
        assert_eq!(matches.kind(), ErrorKind::DisplayVersion);
    }

    #[test]
    fn test_parse_args_with_matches_invalid_option() {
        let cmd = build_cli();
        let matches = cmd
            .try_get_matches_from(["bitcoin", "--invalid"])
            .unwrap_err();
        assert_eq!(matches.kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_parse_args_with_matches_external() {
        let cmd = build_cli();
        let matches = cmd
            .try_get_matches_from(["bitcoin", "external", "--flag", "value"])
            .unwrap();
        let request = parse_args_with_matches(matches).unwrap();
        assert_eq!(request.subcommand, "external");
        assert_eq!(request.args, vec!["--flag", "value"]);
    }
}
