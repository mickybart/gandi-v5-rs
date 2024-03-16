use std::process::ExitCode;
use gandi_v5_livedns_api::Api;

pub async fn list(api: &Api) -> ExitCode {
    let domains = match api.domains.list().await {
        Ok(domains) => domains,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE
        }
    };

    for domain in domains {
        println!("{}", domain.fqdn);
    }

    ExitCode::SUCCESS
}

pub async fn information(api: &Api, fqdn: &str) -> ExitCode {
    let domain_info = match api.domains.information(fqdn).await {
        Ok(domain_info) => domain_info,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE
        }
    };

    println!("{:?}", domain_info);

    ExitCode::SUCCESS
}