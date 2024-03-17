use std::process::ExitCode;
use serde::Serialize;

pub fn handler_yaml<T>(result : Result<T, String>) -> ExitCode
where
    T: Serialize
{
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let yaml = match serde_yaml::to_string(&result) {
        Ok(yaml) => yaml,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    println!("{}", yaml);

    ExitCode::SUCCESS
}