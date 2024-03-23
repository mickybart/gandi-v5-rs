//! Records scope

use std::error::Error;

use crate::api::Api;
use serde::{Deserialize, Serialize};

/// Type representing a record
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub rrset_name: String,
    pub rrset_type: String,
    pub rrset_values: Vec<String>,
    pub rrset_ttl: Option<u32>,
}

/// Type used to create or update a single record
/// 
/// Example:
/// ```no_run
/// let record = UpsertRecord { rrset_values: vec!["127.0.0.1".to_owned()], rrset_ttl: Some(300) };
/// ```
#[derive(Debug, Serialize)]
pub struct UpsertRecord {
    pub rrset_values: Vec<String>,
    pub rrset_ttl: Option<u32>,
}

impl Api {
    /// List records associated with a domain
    /// 
    /// GET on <https://api.gandi.net/v5/livedns/domains/{fqdn}/records>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod, "token")?;
    /// 
    /// let records = api.records("example.org").await?;
    /// 
    /// println!("{:?}", records);
    /// ```
    pub async fn records(&self, fqdn: &str) -> Result<Vec<Record>, Box<dyn Error>> {
        self.engine
            .get(&format!("/livedns/domains/{}/records", fqdn))
            .await
    }

    /// List records named {rrset_name} associated with this domain
    /// 
    /// GET on <https://api.gandi.net/v5/livedns/domains/{fqdn}/records/{rrset_name}>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod, "token")?;
    /// 
    /// // get records for test.example.org
    /// let records = api.records_by_name("example.org", "test").await?;
    /// 
    /// println!("{:?}", records);
    /// ```
    pub async fn records_by_name(
        &self,
        fqdn: &str,
        rrset_name: &str,
    ) -> Result<Vec<Record>, Box<dyn Error>> {
        self.engine
            .get(&format!("/livedns/domains/{}/records/{}", fqdn, rrset_name))
            .await
    }

    /// Get a single single record with its name and type
    /// 
    /// GET on <https://api.gandi.net/v5/livedns/domains/{fqdn}/records/{rrset_name}/{rrset_type}>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod, "token")?;
    /// 
    /// // get TXT record for test.example.org
    /// let record = api.record_by_name_and_type("example.org", "test", "TXT").await?;
    /// 
    /// println!("{:?}", record);
    /// ```
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

    /// Create a new record whose name and type are defined by the path
    /// 
    /// POST on <https://api.gandi.net/v5/livedns/domains/{fqdn}/records/{rrset_name}/{rrset_type}>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod, "token")?;
    /// 
    /// // create multiple A records for test.example.org
    /// let record = UpsertRecord { rrset_values: vec!["10.0.0.1".to_owned(), "10.0.0.2".to_owned()], rrset_ttl: Some(300) };
    /// api.create_record_by_name_and_type("example.org", "test", "A", &record).await?;
    /// ```
    pub async fn create_record_by_name_and_type(
        &self,
        fqdn: &str,
        rrset_name: &str,
        rrset_type: &str,
        record: &UpsertRecord,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "/livedns/domains/{}/records/{}/{}",
            fqdn, rrset_name, rrset_type
        );

        let body = serde_json::to_string(record)?;

        Ok(self.engine.post(&url, body).await?)
    }

    /// Overwrites a single record with {rrset_name} and {rrset_type}
    /// 
    /// PUT on <https://api.gandi.net/v5/livedns/domains/{fqdn}/records/{rrset_name}/{rrset_type}>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod, "token")?;
    /// 
    /// // create multiple A records for test.example.org
    /// let record = UpsertRecord { rrset_values: vec!["10.0.0.1".to_owned(), "10.0.0.2".to_owned()], rrset_ttl: Some(300) };
    /// api.upsert_record_by_name_and_type("example.org", "test", "A", &record).await?;
    /// ```
    pub async fn upsert_record_by_name_and_type(
        &self,
        fqdn: &str,
        rrset_name: &str,
        rrset_type: &str,
        record: &UpsertRecord,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "/livedns/domains/{}/records/{}/{}",
            fqdn, rrset_name, rrset_type
        );

        let body = serde_json::to_string(record)?;

        Ok(self.engine.put(&url, body).await?)
    }

    /// Delete record with {rrset_name} and {rrset_type}
    /// 
    /// DELETE on <https://api.gandi.net/v5/livedns/domains/{fqdn}/records/{rrset_name}/{rrset_type}>
    /// 
    /// Example:
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod, "token")?;
    /// 
    /// // delete A records for test.example.org
    /// api.delete_record_by_name_and_type("example.org", "test", "A").await?;
    /// ```
    pub async fn delete_record_by_name_and_type(
        &self,
        fqdn: &str,
        rrset_name: &str,
        rrset_type: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!(
            "/livedns/domains/{}/records/{}/{}",
            fqdn, rrset_name, rrset_type
        );

        self.engine.delete(&url).await
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use crate::{records::UpsertRecord, Api};

    #[tokio::test]
    async fn records_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();

        let api = Api::build(crate::Endpoint::Sandbox, &pat);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.records("pygoscelis-sandbox.org").await;

        assert!(res.is_err());

        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records)");
    }

    #[tokio::test]
    async fn record_by_name_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();

        let api = Api::build(crate::Endpoint::Sandbox, &pat);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.records_by_name("pygoscelis-sandbox.org", "test").await;

        assert!(res.is_err());

        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test)");
    }

    #[tokio::test]
    async fn record_by_name_and_type_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();

        let api = Api::build(crate::Endpoint::Sandbox, &pat);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.record_by_name_and_type("pygoscelis-sandbox.org", "test", "A").await;

        assert!(res.is_err());

        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn create_record_by_name_and_type_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();

        let api = Api::build(crate::Endpoint::Sandbox, &pat);

        assert!(api.is_ok());

        let api = api.unwrap();

        let record = UpsertRecord { rrset_values: vec!["127.0.01".to_owned()], rrset_ttl: Some(300) };
        let res = api.create_record_by_name_and_type("pygoscelis-sandbox.org", "test", "A", &record).await;

        assert!(res.is_err());

        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn upsert_record_by_name_and_type_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();

        let api = Api::build(crate::Endpoint::Sandbox, &pat);

        assert!(api.is_ok());

        let api = api.unwrap();

        let record = UpsertRecord { rrset_values: vec!["127.0.01".to_owned()], rrset_ttl: Some(300) };
        let res = api.upsert_record_by_name_and_type("pygoscelis-sandbox.org", "test", "A", &record).await;

        assert!(res.is_err());

        assert_eq!(res.unwrap_err().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn delete_record_by_name_and_type_404() {
        let pat = env::var("GANDI_V5_SANDBOX_PAT").unwrap();

        let api = Api::build(crate::Endpoint::Sandbox, &pat);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.delete_record_by_name_and_type("pygoscelis-sandbox.org", "test", "A").await;

        assert!(res.is_err());

        assert_eq!(res.unwrap_err().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }
}
