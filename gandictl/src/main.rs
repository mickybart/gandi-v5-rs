mod cli;
mod livedns;

use std::process::ExitCode;

use cli::{ApiCommands, Cli, LiveDnsCommands, LiveDnsGetCommands};
use gandi_v5_livedns_api::Api;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::init();

    let api = match Api::build() {
        Ok(api) => api,
        Err(_) => {
            eprintln!("An error occurred during the Api initialization !");
            return ExitCode::FAILURE
        }
    };

    match cli.command {
        ApiCommands::LiveDNS { command } => cli_livedns(command, &api).await,
    }
}

async fn cli_livedns(command: LiveDnsCommands, api: &Api) -> ExitCode {
    match command {
        LiveDnsCommands::Get { command } => match command {
            LiveDnsGetCommands::Domains { fqdn } => {
                if let Some(fqdn) = fqdn {
                    livedns::domains::information(&api, &fqdn).await
                } else {
                    livedns::domains::list(&api).await
                }
            }
        },
    }
}
