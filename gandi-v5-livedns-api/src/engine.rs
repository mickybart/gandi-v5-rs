//! Types for working with RESTful Api.

use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use std::{env, error::Error};

/// The engine ables to auth and query Gandi Api.
pub struct Engine {
    /// A `Client` with Personal Access Token set to authenticate against Gandi Api.
    client: Client,
    /// The prod, sandbox or custom endpoint of the Gandi Api.
    endpoint: String,
}

/// Used to select the endpoint required.
pub enum Endpoint {
    Prod,
    Sandbox,
    Custom(String),
}

impl Engine {
    /// Returns a new [`Engine`] object.
    /// 
    /// This function will build a new `Engine` object that will be used
    /// internally by `Api` object to authenticate and query Gandi RESTful Api.
    /// 
    /// ```no_run
    /// let engine = Engine::build(Endpoint::Prod)?;
    /// let engine = Engine::build(Endpoint::Sandbox)?;
    /// let engine = Engine::build(Endpoint::Custom {"https://localhost".to_owned()})?;
    /// ```
    pub fn build(endpoint: Endpoint) -> Result<Self, Box<dyn Error>> {
        // Bearer with Personal Access Token
        let pat = match env::var("GANDI_V5_PAT") {
            Ok(pat) => pat,
            Err(_) => return Err("environment variable 'GANDI_V5_PAT' not found".into()),
        };
        let bearer_pat = "Bearer ".to_owned() + &pat;

        // Headers
        // Authorization: Bearer PERSONAL_ACCESS_TOKEN
        let mut headers = header::HeaderMap::new();

        let mut auth_value = header::HeaderValue::from_str(&bearer_pat)?;
        auth_value.set_sensitive(true);
        headers.insert("authorization", auth_value);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Engine {
            client,
            endpoint: match endpoint {
                Endpoint::Prod => "https://api.gandi.net/v5".to_owned(),
                Endpoint::Sandbox => "https://api.sandbox.gandi.net/v5".to_owned(),
                Endpoint::Custom(endpoint) => endpoint,
            }
        })
    }

    pub async fn get<T>(&self, url: &str) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(format!("{}{}", self.endpoint, url))
            .send()
            .await?;

        Ok(response.error_for_status()?.json::<T>().await?)
    }

    pub async fn post(&self, url: &str, body: String) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .post(format!("{}{}", self.endpoint, url))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }

    pub async fn put(&self, url: &str, body: String) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .put(format!("{}{}", self.endpoint, url))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }

    pub async fn delete(&self, url: &str) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .delete(format!("{}{}", self.endpoint, url))
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use serde::Deserialize;
    use serde_json::Map;

    use super::*;

    #[test]
    fn build_engine_pat_unset() {
        env::remove_var("GANDI_V5_PAT");
        let engine = Engine::build(Endpoint::Prod);

        assert_eq!(engine.is_err(), true);
        assert_eq!(engine.err().unwrap().as_ref().to_string(), "environment variable 'GANDI_V5_PAT' not found");
    }

    #[test]
    fn build_engine_prod() {
        env::set_var("GANDI_V5_PAT", "secret");

        let engine = Engine::build(Endpoint::Prod);

        assert_eq!(engine.is_ok(), true);

        let engine = engine.unwrap();

        assert_eq!(engine.endpoint, "https://api.gandi.net/v5");
    }
    
    #[test]
    fn build_engine_sandbox() {
        let engine = Engine::build(Endpoint::Sandbox);

        assert_eq!(engine.is_ok(), true);

        let engine = engine.unwrap();

        assert_eq!(engine.endpoint, "https://api.sandbox.gandi.net/v5");
    }

    #[test]
    fn build_engine_custom() {
        let engine = Engine::build(Endpoint::Custom("https://api.local".to_owned()));

        assert_eq!(engine.is_ok(), true);

        let engine = engine.unwrap();

        assert_eq!(engine.endpoint, "https://api.local");
    }

    #[tokio::test]
    async fn check_bearer_header() {
        let backup_pat = env::var("GANDI_V5_PAT").unwrap();
        env::set_var("GANDI_V5_PAT", "secret");

        let engine = Engine::build(Endpoint::Custom("https://httpbin.org".to_owned())).unwrap();

        #[derive(Deserialize)]
        struct Headers {
            headers: Map<String, serde_json::Value>,
        }

        let response: Result<Headers, Box<dyn Error>> = engine.get("/headers").await;

        env::set_var("GANDI_V5_PAT", backup_pat);

        assert_eq!(response.is_ok(), true);

        let response = response.unwrap();

        assert!(response.headers.contains_key("Authorization"));

        let bearer = response.headers.get("Authorization").unwrap().as_str().unwrap();

        assert_eq!(bearer, "Bearer secret");
    }
}