//! Types for working with Gandi LiveDNS Api.

pub mod domains;
pub mod records;

use crate::engine::{Endpoint, Engine};
use std::error::Error;

/// The Api entrypoint
///
/// A [personal access token](https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html#personal-access-tokens) is required.
///
/// # Examples
///
/// Create an Api using Gandi prod endpoint:
///
/// ```no_run
/// let api = Api::build(Endpoint::Prod, "token")?;
/// ```
///
/// Create an Api using Gandi sandbox endpoint:
///
/// ```no_run
/// let api = Api::build(Endpoint::Sandbox, "token")?;
/// ```
///
/// Create an Api using a custom fqdn in front of a Gandi endpoint:
///
/// ```no_run
/// let api = Api::build(Endpoint::Custom{"https://api.example.org".to_owned()}, "token")?;
/// ```
///
/// Query all domains available:
///
/// ```no_run
/// let api = Api::build(Endpoint::Prod, "token")?;
/// let domains = api.domains().await?;
/// println!("{:?}", domains);
/// ```
pub struct Api {
    engine: Engine,
}

impl Api {
    /// Returns a new [`Api`] object.
    ///
    /// This function will build a new `Api` object that you can use to
    /// query Gandi LiveDNS Api.
    ///
    /// An internal `Engine` will be created with the [`Endpoint`] provided.
    ///
    /// A [Personal Access Token](https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html#personal-access-tokens) is required.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let personal_access_token = "token".to_owned();
    ///
    /// let api = Api::build(Endpoint::Prod, &personal_access_token)?;
    /// let api = Api::build(Endpoint::Sandbox, &personal_access_token)?;
    /// let api = Api::build(Endpoint::Custom {"https://localhost".to_owned()}, &personal_access_token)?;
    /// ```
    pub fn build(endpoint: Endpoint, personal_access_token: &str) -> Result<Self, Box<dyn Error>> {
        let engine = Engine::build(endpoint, personal_access_token)?;

        Ok(Api { engine })
    }
}

#[cfg(test)]
mod tests {
    use crate::Api;

    #[test]
    fn build_api() {
        let api = Api::build(crate::Endpoint::Prod, "token");

        assert!(api.is_ok());
    }
}
