use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use std::{env, error::Error};

pub struct Engine {
    client: Client,
    endpoint: String,
}

impl Engine {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        // Bearer with Personal Access Token
        let pat = match env::var("GANDI_V5_PAT") {
            Ok(pat) => pat,
            Err(_) => return Err("environment variable 'GANDI_V5_PAT' not found".into()),
        };
        let bearer_pat = "Bearer ".to_owned() + &pat;

        // Headers
        // Content-Type: application/json
        // Authorization: Bearer PERSONAL_ACCESS_TOKEN
        let mut headers = header::HeaderMap::new();
        // headers.insert(
        //     "content-type",
        //     header::HeaderValue::from_static("application/json"),
        // );

        let mut auth_value = header::HeaderValue::from_str(&bearer_pat)?;
        auth_value.set_sensitive(true);
        headers.insert("authorization", auth_value);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Engine {
            client,
            endpoint: "https://api.gandi.net/v5".to_owned(),
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
}
