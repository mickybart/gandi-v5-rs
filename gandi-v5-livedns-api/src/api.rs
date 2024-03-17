mod domains;

use crate::engine::Engine;
use std::{error::Error, rc::Rc};

pub struct Api {
    pub domains: Domains,
}

pub struct Domains {
    engine: Rc<Engine>,
}

impl Api {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let engine = Rc::new(Engine::build()?);

        Ok(Api {
            // for future multiple usage of engin, use: Rc::clone(&engine)
            domains: Domains { engine },
        })
    }
}
