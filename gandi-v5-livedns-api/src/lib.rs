//! Gandi LiveDNS Api
//! 
//! Provides an abstration on top of Gandi LiveDNS RESTful Api.
//! 
//! # Examples
//! 
//! You need to create your own personal access token (pat) in the Gandi.net console.
//! 
//! ```
//! use std:env;
//! use gandi_v5_livedns_api::{Api, Endpoint};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let personal_access_token = env::var("GANDI_V5_PAT")?;
//! 
//!     let api = Api::build(Endpoint::Prod, &personal_access_token)?;
//! }
//! ```

mod api;
mod engine;

pub use api::records;
pub use api::Api;
pub use engine::Endpoint;
