mod domains;

use crate::engine::Engine;
use std::rc::Rc;

pub struct Api {
    pub domains: Domains,
}

pub struct Domains {
    engine: Rc<Engine>,
}

impl Api {
    pub fn build() -> Result<Self, String> {
        let engine = Rc::new(Engine::build()?);

        Ok(Api {
            // for future multiple usage of common, use: Rc::clone(&common)
            domains: Domains { engine },
        })
    }
}
