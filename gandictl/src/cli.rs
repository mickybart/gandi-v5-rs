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
    /// Get resources relative to domains.
    Domains {},
    /// Get resources realtive to one domain name.
    Domain {
        /// Domain name
        fqdn: String,
    },
    /// Get resources relative to domain records.
    Records {
        /// Domain name
        fqdn: String,
    },
    /// Get resources relative to on domain record.
    Record {
        /// Domain name
        fqdn: String,
        /// Record name
        rrset_name: String,
        /// Record type
        #[arg(short, long)]
        rrset_type: Option<String>,
    }
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}
