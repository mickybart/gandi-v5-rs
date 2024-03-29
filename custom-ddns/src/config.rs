use std::env;

use config::{Config, ConfigError, File, Map};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AppConfig {
    pub(crate) listen: String,
    pub(crate) default_rrset_ttl: u32,
    whitelist: Map<String, Whitelist>,
}

#[derive(Deserialize)]
struct Whitelist {
    personal_access_token: String,
    records: Vec<Record>,
    authorizations: Vec<String>,
}

#[derive(Deserialize)]
struct Record {
    rrset_name: String,
    rrset_type: String,
    rrset_ttl_max: u32,
}

impl AppConfig {
    pub(crate) fn build() -> Result<Self, ConfigError> {
        let cddns_profile = env::var("CDDNS_PROFILE").unwrap_or("prod".to_owned());

        Config::builder()
            .add_source(File::with_name(&format!("config/{}.yaml", cddns_profile)).required(false))
            .add_source(File::with_name("config/local.yaml").required(false))
            .build()?
            .try_deserialize()
    }

    /// Returns a personal access token if the record to update
    /// is whitelisted and authorized to be updated by the authorization
    pub(crate) fn get_pat_if_authorized(
        &self,
        fqdn: &str,
        rrset_name: &str,
        rrset_type: &str,
        rrset_ttl: u32,
        authorization: &str,
    ) -> Result<String, String> {
        let whitelist = self
            .whitelist
            .get(fqdn)
            .ok_or(format!("config: fqdn {} is not whitelisted", fqdn))?;

        _ = whitelist
            .records
            .iter()
            .find(|record| {
                record.rrset_name == rrset_name
                    && record.rrset_type == rrset_type
                    && rrset_ttl <= record.rrset_ttl_max
            })
            .ok_or(format!(
                "config: record {}.{} type {} with ttl {} is not whitelisted",
                rrset_name, fqdn, rrset_type, rrset_ttl
            ))?;

        if whitelist.authorizations.contains(&authorization.to_owned()) {
            Ok(whitelist.personal_access_token.to_owned())
        } else {
            Err(format!(
                "config: access forbidden to update record {}.{} type {}",
                rrset_name, fqdn, rrset_type
            ))
        }
    }
}
