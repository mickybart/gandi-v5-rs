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

#[derive(Debug, Serialize)]
pub struct UpsertRecord {
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
    use crate::{records::UpsertRecord, Api};

    #[tokio::test]
    async fn records_404() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.records("pygoscelis-sandbox.org").await;

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records)");
    }

    #[tokio::test]
    async fn record_by_name_404() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.records_by_name("pygoscelis-sandbox.org", "test").await;

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test)");
    }

    #[tokio::test]
    async fn record_by_name_and_type_404() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.record_by_name_and_type("pygoscelis-sandbox.org", "test", "A").await;

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn create_record_by_name_and_type_404() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let record = UpsertRecord { rrset_values: vec!["127.0.01".to_owned()], rrset_ttl: Some(300) };
        let res = api.create_record_by_name_and_type("pygoscelis-sandbox.org", "test", "A", &record).await;

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn upsert_record_by_name_and_type_404() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let record = UpsertRecord { rrset_values: vec!["127.0.01".to_owned()], rrset_ttl: Some(300) };
        let res = api.upsert_record_by_name_and_type("pygoscelis-sandbox.org", "test", "A", &record).await;

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().as_ref().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }

    #[tokio::test]
    async fn delete_record_by_name_and_type_404() {
        let api = Api::build(crate::Endpoint::Sandbox);

        assert!(api.is_ok());

        let api = api.unwrap();

        let res = api.delete_record_by_name_and_type("pygoscelis-sandbox.org", "test", "A").await;

        assert!(res.is_err());

        assert_eq!(res.err().unwrap().to_string(), "HTTP status client error (404 Not Found) for url (https://api.sandbox.gandi.net/v5/livedns/domains/pygoscelis-sandbox.org/records/test/A)");
    }
}
