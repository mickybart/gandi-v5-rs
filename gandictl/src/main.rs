mod cli;
mod livedns;

use cli::{ApiCommands, Cli, LiveDnsCommands, LiveDnsGetCommands};
use gandi_v5_livedns_api::Api;

#[tokio::main]
async fn main() {
    let cli = Cli::init();

    let api = Api::build();

    match cli.command {
        ApiCommands::LiveDNS { command } => {
            match command {
                LiveDnsCommands::Get { command } => {
                    match command {
                        LiveDnsGetCommands::Domains { fqdn } => {
                            if let Some(fqdn) = fqdn {
                                livedns::domains::information(&api, &fqdn).await;
                            } else {
                                livedns::domains::list(&api).await;
                            }
                        },
                    }
                },
            }
        },
    }
}
