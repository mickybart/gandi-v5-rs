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
    pub async fn list(&self) -> Vec<Domain> {
        let response = self.common.client.get(
            format!("{}/livedns/domains", self.common.endpoint)
        ).send().await;

        let response = response.expect("error");
    
        // let text = response.text().await.expect("error");
        let domains = response.json::<Vec<Domain>>().await.expect("error");
    
        domains
    }

    pub async fn information(&self, fqdn: &str) -> DomainInfo {
        let response = self.common.client.get(
            format!("{}/livedns/domains/{}", self.common.endpoint, fqdn)
        ).send().await;

        let response = response.expect("error");

        let domain_info = response.json::<DomainInfo>().await.expect("error");

        domain_info
    }
}
