mod cli;
mod output;

use std::error::Error;

use cli::{ApiCommands, Cli, LiveDnsCommands, LiveDnsGetCommands};
use gandi_v5_livedns_api::Api;
use output::handler_yaml;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::init();

    let api = Api::build()?;

    match cli.command {
        ApiCommands::LiveDNS { command } => cli_livedns(command, &api).await,
    }
}

async fn cli_livedns(command: LiveDnsCommands, api: &Api) -> Result<(), Box<dyn Error>> {
    match command {
        LiveDnsCommands::Get { command } => match command {
            LiveDnsGetCommands::Domains {} => handler_yaml(api.domains().await?),
            LiveDnsGetCommands::Domain { fqdn } => handler_yaml(api.domain(&fqdn).await?),
            LiveDnsGetCommands::Records { fqdn, rrset_name } => match rrset_name {
                Some(rrset_name) => handler_yaml(api.records_by_name(&fqdn, &rrset_name).await?),
                None => handler_yaml(api.records(&fqdn).await?),
            },
            LiveDnsGetCommands::Record {
                fqdn,
                rrset_name,
                rrset_type,
            } => handler_yaml(
                api.record_by_name_and_type(&fqdn, &rrset_name, &rrset_type)
                    .await?,
            ),
        },
    }
}
