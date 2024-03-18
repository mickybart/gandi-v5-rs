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
            LiveDnsGetCommands::Domains {} => handler_yaml(api.domains.list().await?),
            LiveDnsGetCommands::Domain { fqdn } => {
                handler_yaml(api.domains.information(&fqdn).await?)
            }
            LiveDnsGetCommands::Records { fqdn } => handler_yaml(api.domains.records(&fqdn).await?),
            LiveDnsGetCommands::Record {
                fqdn,
                rrset_name,
                rrset_type,
            } => match rrset_type {
                Some(rrset_type) => handler_yaml(
                    api.domains
                        .records_by_name_and_type(&fqdn, &rrset_name, &rrset_type)
                        .await?,
                ),
                None => handler_yaml(api.domains.records_by_name(&fqdn, &rrset_name).await?),
            },
        },
    }
}
