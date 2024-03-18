use std::error::Error;

use crate::api::Api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub rrset_name: String,
    pub rrset_type: String,
    pub rrset_values: Vec<String>,
    pub rrset_ttl: Option<u32>,
}

impl Api {
    pub async fn records(&self, fqdn: &str) -> Result<Vec<Record>, Box<dyn Error>> {
        self.engine
            .get(&format!("/livedns/domains/{}/records", fqdn))
            .await
    }

    pub async fn records_by_name(
        &self,
        fqdn: &str,
        rrset_name: &str,
    ) -> Result<Vec<Record>, Box<dyn Error>> {
        self.engine
            .get(&format!("/livedns/domains/{}/records/{}", fqdn, rrset_name))
            .await
    }

    pub async fn record_by_name_and_type(
        &self,
        fqdn: &str,
        rrset_name: &str,
        rrset_type: &str,
    ) -> Result<Record, Box<dyn Error>> {
        self.engine
            .get(&format!(
                "/livedns/domains/{}/records/{}/{}",
                fqdn, rrset_name, rrset_type
            ))
            .await
    }
}
