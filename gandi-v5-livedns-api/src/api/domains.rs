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
    pub async fn list(&self) -> Result<Vec<Domain>,String> {
        let response = self.common.client.get(
            format!("{}/livedns/domains", self.common.endpoint)
        ).send().await;

        let response = match response {
            Ok(response) => response,
            Err(_) => return Err("Network issue !".to_owned()),
        };
    
        if response.status().is_success() {
            match response.json::<Vec<Domain>>().await {
                Ok(domains) => Ok(domains),
                Err(_) => Err("Payload can't be decoded !".to_owned()),
            }
        } else {
            Err(format!("Server returned {} !", response.status()))
        }
    }

    pub async fn information(&self, fqdn: &str) -> Result<DomainInfo,String> {
        let response = self.common.client.get(
            format!("{}/livedns/domains/{}", self.common.endpoint, fqdn)
        ).send().await;

        let response = match response {
            Ok(response) => response,
            Err(_) => return Err("Network issue !".to_owned()),
        };

        if response.status().is_success() {
            match response.json::<DomainInfo>().await {
                Ok(domain_info) => Ok(domain_info),
                Err(_) => Err("Payload can't be decoded !".to_owned()),
            }
        } else {
            Err(format!("Server returned {} !", response.status()))
        }
    }
}
