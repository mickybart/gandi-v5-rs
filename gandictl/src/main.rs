mod cli;
mod output;

use std::{error::Error, process::ExitCode};

use cli::*;
use gandi_v5_livedns_api::{records::UpsertRecord, Api};
use output::handler_yaml;

#[tokio::main]
async fn main() -> ExitCode {
    let terminated = main_delegation().await;

    match terminated {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e.as_ref());

            ExitCode::FAILURE
        },
    }
}

async fn main_delegation() -> Result<(), Box<dyn Error>> {
    let cli = Cli::init();

    let api = Api::build()?;

    match cli.command {
        ApiCommands::LiveDNS { command } => match command {
            LiveDnsCommands::Get { command } => livedns_get(command, &api).await,
            LiveDnsCommands::Apply { command } => livedns_apply(command, &api).await,
            LiveDnsCommands::Create { command } => livedns_create(command, &api).await,
            LiveDnsCommands::Delete { command } => livedns_delete(command, &api).await,
        },
    }
}

async fn livedns_get(command: LiveDnsGetCommands, api: &Api) -> Result<(), Box<dyn Error>> {
    match command {
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
    }
}

async fn livedns_apply(command: LiveDnsApplyCommands, api: &Api) -> Result<(), Box<dyn Error>> {
    match command {
        LiveDnsApplyCommands::Record {
            fqdn,
            rrset_name,
            rrset_type,
            rrset_values,
            rrset_ttl,
        } => {
            let record = UpsertRecord {
                rrset_values,
                rrset_ttl,
            };

            api.upsert_record_by_name_and_type(&fqdn, &rrset_name, &rrset_type, &record)
                .await?;

            println!(
                "Record {}.{} type {} applied !",
                rrset_name, fqdn, rrset_type
            );

            Ok(())
        }
    }
}

async fn livedns_create(command: LiveDnsCreateCommands, api: &Api) -> Result<(), Box<dyn Error>> {
    match command {
        cli::LiveDnsCreateCommands::Record {
            fqdn,
            rrset_name,
            rrset_type,
            rrset_values,
            rrset_ttl,
        } => {
            let record = UpsertRecord {
                rrset_values,
                rrset_ttl,
            };

            api.create_record_by_name_and_type(&fqdn, &rrset_name, &rrset_type, &record)
                .await?;

            println!(
                "Record {}.{} type {} created !",
                rrset_name, fqdn, rrset_type
            );

            Ok(())
        }
    }
}

async fn livedns_delete(command: LiveDnsDeleteCommands, api: &Api) -> Result<(), Box<dyn Error>> {
    match command {
        cli::LiveDnsDeleteCommands::Record {
            fqdn,
            rrset_name,
            rrset_type,
        } => {
            api.delete_record_by_name_and_type(&fqdn, &rrset_name, &rrset_type)
                .await?;

            println!(
                "Record {}.{} type {} deleted !",
                rrset_name, fqdn, rrset_type
            );

            Ok(())
        }
    }
}
