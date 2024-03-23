use serde::Serialize;
use std::error::Error;

pub(crate) fn handler_yaml<T>(data: T) -> Result<(), Box<dyn Error>>
// only handle serde_yaml::Error but simplifying caller code by using dyn Error
where
    T: Serialize,
{
    let yaml = serde_yaml::to_string(&data)?;

    println!("{}", yaml);

    Ok(())
}
