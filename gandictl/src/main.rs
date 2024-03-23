#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

//! gandictl controls the gandi.net management console.

mod cli;
mod output;

use cli::*;
use gandi_v5_livedns_api::{records::UpsertRecord, Api, Endpoint};
use output::handler_yaml;
use std::{env, error::Error, process::ExitCode};

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> ExitCode {
    let terminated = main_delegation().await;

    match terminated {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e.as_ref());

            ExitCode::FAILURE
        }
    }
}

#[cfg(not(tarpaulin_include))]
async fn main_delegation() -> Result<(), Box<dyn Error>> {
    let cli = Cli::init();

    let personal_access_token = env::var("GANDI_V5_PAT")?;

    let endpoint = if cli.sandbox {
        Endpoint::Sandbox
    } else {
        Endpoint::Prod
    };

    let api = Api::build(endpoint, &personal_access_token)?;

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

#[cfg(test)]
mod tests {
    use crate::livedns_apply;
    use crate::livedns_create;
    use crate::livedns_delete;
    use crate::livedns_get;
    use crate::Api;
    use std::env;

    #[tokio::test]
    async fn get_domains() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_get(crate::LiveDnsGetCommands::Domains {}, &api).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn get_domain_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_get(
            crate::LiveDnsGetCommands::Domain {
                fqdn: "pygoscelis-sandbox.org".to_owned(),
            },
            &api,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org)");
    }

    #[tokio::test]
    async fn get_records_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_get(
            crate::LiveDnsGetCommands::Records {
                fqdn: "pygoscelis-sandbox.org".to_owned(),
                rrset_name: None,
            },
            &api,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records)");
    }

    #[tokio::test]
    async fn get_records_name_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_get(
            crate::LiveDnsGetCommands::Records {
                fqdn: "pygoscelis-sandbox.org".to_owned(),
                rrset_name: Some("test".to_owned()),
            },
            &api,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test)");
    }

    #[tokio::test]
    async fn get_record_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_get(
            crate::LiveDnsGetCommands::Record {
                fqdn: "pygoscelis-sandbox.org".to_owned(),
                rrset_name: "test".to_owned(),
                rrset_type: "A".to_owned(),
            },
            &api,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn create_record_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_create(
            crate::LiveDnsCreateCommands::Record {
                fqdn: "pygoscelis-sandbox.org".to_owned(),
                rrset_name: "test".to_owned(),
                rrset_type: "A".to_owned(),
                rrset_values: vec!["127.0.0.1".to_owned()],
                rrset_ttl: None,
            },
            &api,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn apply_record_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_apply(
            crate::LiveDnsApplyCommands::Record {
                fqdn: "pygoscelis-sandbox.org".to_owned(),
                rrset_name: "test".to_owned(),
                rrset_type: "A".to_owned(),
                rrset_values: vec!["127.0.0.1".to_owned()],
                rrset_ttl: None,
            },
            &api,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn delete_record_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();
        let api = Api::build(crate::Endpoint::Sandbox, &pat).unwrap();

        let res = livedns_delete(
            crate::LiveDnsDeleteCommands::Record {
                fqdn: "pygoscelis-sandbox.org".to_owned(),
                rrset_name: "test".to_owned(),
                rrset_type: "A".to_owned(),
            },
            &api,
        )
        .await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }
}
