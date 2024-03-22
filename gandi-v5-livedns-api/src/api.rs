//! Types for working with Gandi LiveDNS Api.

mod domains;
pub mod records;

use crate::engine::{Endpoint, Engine};
use std::error::Error;

/// The Api entrypoint
///  
/// # Examples
/// 
/// Create an Api using Gandi prod Api:
/// 
/// ```no_run
/// let api = Api::build(Endpoint::Prod)?;
/// ```
/// 
/// Create an Api using Gandi sandbox Api:
/// 
/// ```no_run
/// let api = Api::build(Endpoint::Sandbox)?;
/// ```
/// 
/// Create an Api using a custom fqdn in front of Gandi Api:
/// 
/// ```no_run
/// let api = Api::build(Endpoint::Custom{"https://api.example.org".to_owned()})?;
/// ```
/// 
/// Query all domains available:
/// 
/// ```no_run
/// let api = Api::build(Endpoint::Prod)?;
/// let domains = api.domains().await?;
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
    /// An internal `Engine` will be created with the `Endpoint` provided.
    /// A Personal Access Token (pat) will be used to query Gandi Api.
    /// 
    /// ```no_run
    /// let api = Api::build(Endpoint::Prod)?;
    /// let api = Api::build(Endpoint::Sandbox)?;
    /// let api = Api::build(Endpoint::Custom {"https://localhost".to_owned()})?;
    /// ```
    pub fn build(endpoint: Endpoint) -> Result<Self, Box<dyn Error>> {
        let engine = Engine::build(endpoint)?;

        Ok(Api { engine })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::Api;

    #[test]
    fn build_api_pat_unset() {
        let backup_pat = env::var("GANDI_V5_PAT").unwrap();
        env::remove_var("GANDI_V5_PAT");
        let api = Api::build(crate::Endpoint::Prod);
        env::set_var("GANDI_V5_PAT", backup_pat);

        assert!(api.is_err());
        assert_eq!(api.err().unwrap().as_ref().to_string(), "environment variable 'GANDI_V5_PAT' not found");
    }

    #[test]
    fn build_api_pat_set() {
        let api = Api::build(crate::Endpoint::Prod);

        assert!(api.is_ok());
    }
}
