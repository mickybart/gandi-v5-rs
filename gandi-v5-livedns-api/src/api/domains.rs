use std::error::Error;

use crate::api::Domains;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub fqdn: String,
    pub domain_href: String,
    pub domain_records_href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainInfo {
    pub fqdn: String,
    pub automatic_snapshot: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub rrset_name: String,
    pub rrset_type: String,
    pub rrset_values: Vec<String>,
    pub rrset_ttl: Option<u32>,
}

impl Domains {
    pub async fn list(&self) -> Result<Vec<Domain>, Box<dyn Error>> {
        self.engine.get("/livedns/domains").await
    }

    pub async fn information(&self, fqdn: &str) -> Result<DomainInfo, Box<dyn Error>> {
        self.engine.get(&format!("/livedns/domains/{}", fqdn)).await
    }

    pub async fn list_records(&self, fqdn: &str) -> Result<Vec<Record>, Box<dyn Error>> {
        self.engine
            .get(&format!("/livedns/domains/{}/records", fqdn))
            .await
    }
}
