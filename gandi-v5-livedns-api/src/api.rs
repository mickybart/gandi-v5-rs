mod domains;
pub mod records;

use crate::engine::Engine;
use std::error::Error;

pub struct Api {
    engine: Engine,
}

impl Api {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let engine = Engine::build()?;

        Ok(Api {
            engine,
        })
    }
}
