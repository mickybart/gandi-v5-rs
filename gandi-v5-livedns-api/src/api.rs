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
    pub fn build() -> Result<Self, ()> {
        let engine = match Engine::build() {
            Ok(engine) => Rc::new(engine),
            Err(_) => return Err(()),
        };

        Ok(Api {
            // for future multiple usage of common, use: Rc::clone(&common)
            domains: Domains { engine },
        })
    }
}
