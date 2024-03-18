use clap::{Parser, Subcommand};

/// Control Gandi services
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ApiCommands,
}

#[derive(Subcommand)]
pub enum ApiCommands {
    /// LiveDNS API (https://api.gandi.net/docs/livedns/)
    LiveDNS {
        #[command(subcommand)]
        command: LiveDnsCommands,
    },
}

#[derive(Subcommand)]
pub enum LiveDnsCommands {
    /// Display one or many resources.
    Get {
        #[command(subcommand)]
        command: LiveDnsGetCommands,
    },
}

#[derive(Subcommand)]
pub enum LiveDnsGetCommands {
    /// List of domains handled by LiveDNS
    Domains {},
    /// Show domain's properties
    Domain {
        /// Domain name
        fqdn: String,
    },
    /// List records or named records associated with a domain
    Records {
        /// Domain name
        fqdn: String,
        /// Name of the record
        #[arg(short, long)]
        rrset_name: Option<String>,
    },
    /// Get a single record with its name and type
    Record {
        /// Domain name
        fqdn: String,
        /// Name of the record
        rrset_name: String,
        /// Type of the record
        rrset_type: String,
    },
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}
