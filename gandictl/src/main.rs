mod cli;
mod output;

use std::process::ExitCode;

use cli::{ApiCommands, Cli, LiveDnsCommands, LiveDnsGetCommands};
use gandi_v5_livedns_api::Api;
use output::handler_yaml;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::init();

    let api = match Api::build() {
        Ok(api) => api,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    match cli.command {
        ApiCommands::LiveDNS { command } => cli_livedns(command, &api).await,
    }
}

async fn cli_livedns(command: LiveDnsCommands, api: &Api) -> ExitCode {
    match command {
        LiveDnsCommands::Get { command } => match command {
            LiveDnsGetCommands::Domains {} => handler_yaml(api.domains.list().await),
            LiveDnsGetCommands::Domain { fqdn, records } => {
                if !records {
                    handler_yaml(api.domains.information(&fqdn).await)
                } else {
                    handler_yaml(api.domains.list_records(&fqdn).await)
                }
            }
        },
    }
}
