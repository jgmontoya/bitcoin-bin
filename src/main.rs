mod cli;
mod router;

use cli::parse_args;
use router::route_command;

fn main() {
    let request = match parse_args() {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match route_command(&request) {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
