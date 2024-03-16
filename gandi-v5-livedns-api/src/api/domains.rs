use crate::api::Domains;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub fqdn: String,
    pub domain_href: String,
    pub domain_records_href: String,
}

#[derive(Debug, Deserialize)]
pub struct DomainInfo {
    pub fqdn: String,
    pub automatic_snapshot: Option<bool>,
}

impl Domains {
    pub async fn list(&self) -> Result<Vec<Domain>, String> {
        self.engine.get("/livedns/domains").await
    }

    pub async fn information(&self, fqdn: &str) -> Result<DomainInfo, String> {
        self.engine.get(&format!("/livedns/domains/{}", fqdn)).await
    }
}
