use reqwest::{header, Client};
use serde::de::DeserializeOwned;
use std::env;

pub struct Common {
    client: Client,
    endpoint: String,
}

impl Common {
    pub fn build() -> Self {
        // Bearer with Personal Access Token
        let bearer_pat = "Bearer ".to_owned() + &env::var("GANDI_V5_PAT").unwrap();

        // Headers
        // Content-Type: application/json
        // Authorization: Bearer PERSONAL_ACCESS_TOKEN
        let mut headers = header::HeaderMap::new();
        // headers.insert(
        //     "content-type",
        //     header::HeaderValue::from_static("application/json"),
        // );

        let mut auth_value = header::HeaderValue::from_str(&bearer_pat)
            .expect("Personal Access Token header is invalid !");
        auth_value.set_sensitive(true);
        headers.insert("authorization", auth_value);

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create the reqwest client !");

        Common {
            client,
            endpoint: "https://api.gandi.net/v5".to_owned(),
        }
    }

    pub async fn get<T>(&self, url: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(format!("{}{}", self.endpoint, url))
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(_) => return Err("Network issue !".to_owned()),
        };

        if response.status().is_success() {
            match response.json::<T>().await {
                Ok(t) => Ok(t),
                Err(_) => Err("Payload can't be decoded !".to_owned()),
            }
        } else {
            Err(format!("Server returned {} !", response.status()))
        }
    }
}
