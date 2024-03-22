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
