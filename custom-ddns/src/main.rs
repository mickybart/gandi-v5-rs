#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

//! Custom Dynamic DNS

mod app;
mod config;

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let terminated = app::run_app().await;

    match terminated {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e.as_ref());

            ExitCode::FAILURE
        }
    }
}
