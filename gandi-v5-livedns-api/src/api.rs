mod domains;
pub mod records;

use crate::engine::{Endpoint, Engine};
use std::error::Error;

pub struct Api {
    engine: Engine,
}

impl Api {
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
