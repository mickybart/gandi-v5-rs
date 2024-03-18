use std::error::Error;

use crate::api::Api;
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

impl Api {
    pub async fn domains(&self) -> Result<Vec<Domain>, Box<dyn Error>> {
        self.engine.get("/livedns/domains").await
    }

    pub async fn domain(&self, fqdn: &str) -> Result<DomainInfo, Box<dyn Error>> {
        self.engine.get(&format!("/livedns/domains/{}", fqdn)).await
    }
}
