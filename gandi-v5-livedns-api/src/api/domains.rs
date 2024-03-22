//! Domains scope

use std::error::Error;

use crate::api::Api;
use serde::{Deserialize, Serialize};

/// Type representing a Domain
#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub fqdn: String,
    pub domain_href: String,
    pub domain_records_href: String,
}

/// Type representing Domain's properties
#[derive(Debug, Deserialize, Serialize)]
pub struct DomainInfo {
    pub fqdn: String,
    pub automatic_snapshot: Option<bool>,
}

impl Api {
    /// List of domains handled by LiveDNS
    /// 
    /// GET on <https://api.gandi.net/v5/livedns/domains>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod)?;
    /// 
    /// let domains = api.domains().await?;
    /// 
    /// println!("{:?}", domains);
    /// ```
    pub async fn domains(&self) -> Result<Vec<Domain>, Box<dyn Error>> {
        self.engine.get("/livedns/domains").await
    }

    /// Show domain's properties
    /// 
    /// GET on <https://api.gandi.net/v5/livedns/domains/{fqdn}>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod)?;
    /// 
    /// let domain_info = api.domain("example.org").await?;
    /// 
    /// println!("{:?}", domain_info);
    /// ```
    pub async fn domain(&self, fqdn: &str) -> Result<DomainInfo, Box<dyn Error>> {
        self.engine.get(&format!("/livedns/domains/{}", fqdn)).await
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::Api;

    #[tokio::test]
    async fn domains_empty() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.domains().await;

        assert!(res.is_ok());

        let res = res.unwrap();

        assert!(res.is_empty());
    }

    #[tokio::test]
    async fn domain_404() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.domain("pygoscelis-sandbox.org").await;

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org)");
    }

    #[tokio::test]
    async fn domain_403() {
        let backup_pat = env::var("GANDI_V5_PAT").unwrap();
        env::set_var("GANDI_V5_PAT", "INVALID");

        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.domain("pygoscelis-sandbox.org").await;

        env::set_var("GANDI_V5_PAT", backup_pat);

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().as_ref().to_string(), "HTTP status client error (403 Forbidden) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org)");
    }
}
