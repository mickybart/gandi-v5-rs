#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

//! Gandi LiveDNS Api
//!
//! Provides an abstration on top of Gandi LiveDNS RESTful Api.
//!
//! A [personal access token](https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html#personal-access-tokens) is required
//!
//! # Examples
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

pub use api::domains;
pub use api::records;
pub use api::Api;
pub use engine::Endpoint;
