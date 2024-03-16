mod domains;

use std::rc::Rc;
use crate::common::Common;

pub struct Api {
    pub domains: Domains,
}

pub struct Domains {
    common: Rc<Common>,
}

impl Api {
    pub fn build() -> Self {
        let common = Rc::new(Common::build());

        Api {
            // for future multiple usage of common, use: Rc::clone(&common)
            domains: Domains { common },
        }
    }
}
