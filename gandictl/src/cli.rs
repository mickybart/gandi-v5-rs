use clap::{Parser, Subcommand};

/// Control Gandi services
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ApiCommands,
}

#[derive(Subcommand)]
pub enum ApiCommands {
    /// LiveDNS API (https://api.gandi.net/docs/livedns/)
    LiveDNS {
        #[command(subcommand)]
        command: LiveDnsCommands
    },
}

#[derive(Subcommand)]
pub enum LiveDnsCommands {
    /// Display one or many resources.
    Get {
        #[command(subcommand)]
        command: LiveDnsGetCommands
    },
}

#[derive(Subcommand)]
pub enum LiveDnsGetCommands {
    /// Get resources relative to domains.
    Domains {
        /// Domain name
        fqdn: Option<String>
    },
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}