use std::ops::RangeInclusive;

use clap::{Parser, Subcommand};

/// Control Gandi services
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ApiCommands,
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
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
    /// Overwrite one or many resources.
    Apply {
        #[command(subcommand)]
        command: LiveDnsApplyCommands,
    },
    /// Create one or many resources.
    Create {
        #[command(subcommand)]
        command: LiveDnsCreateCommands,
    },
    /// Delete one or many resources.
    Delete {
        #[command(subcommand)]
        command: LiveDnsDeleteCommands,
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

#[derive(Subcommand)]
pub enum LiveDnsApplyCommands {
    /// Overwrites a single record with {rrset_name} and {rrset_type}
    Record {
        /// Domain name
        fqdn: String,
        /// Name of the record
        rrset_name: String,
        /// Type of the record
        rrset_type: String,
        /// A list of values for this record (comma delimiter)
        #[arg(long,required=true,num_args=1..,value_delimiter=',')]
        rrset_values: Vec<String>,
        /// The time in seconds that DNS resolvers should cache this record (300 to 2592000)
        #[arg(long,value_parser = rrset_ttl_in_range)]
        rrset_ttl: Option<u32>,
    },
}

#[derive(Subcommand)]
pub enum LiveDnsCreateCommands {
    /// Create a new record for {rrset_name} and {rrset_type}
    Record {
        /// Domain name
        fqdn: String,
        /// Name of the record
        rrset_name: String,
        /// Type of the record
        rrset_type: String,
        /// A list of values for this record (comma delimiter)
        #[arg(long,required=true,num_args=1..,value_delimiter=',')]
        rrset_values: Vec<String>,
        /// The time in seconds that DNS resolvers should cache this record (300 to 2592000)
        #[arg(long,value_parser = rrset_ttl_in_range)]
        rrset_ttl: Option<u32>,
    },
}

#[derive(Subcommand)]
pub enum LiveDnsDeleteCommands {
    /// Delete a single record with {rrset_name} and {rrset_type}
    Record {
        /// Domain name
        fqdn: String,
        /// Name of the record
        rrset_name: String,
        /// Type of the record
        rrset_type: String,
    },
}

const RRSET_TTL_RANGE: RangeInclusive<usize> = 300..=2592000;

fn rrset_ttl_in_range(rrset_ttl: &str) -> Result<u32, String> {
    let rrset_ttl: usize = rrset_ttl
        .parse()
        .map_err(|_| format!("'{rrset_ttl} isn't a ttl number'"))?;

    if RRSET_TTL_RANGE.contains(&rrset_ttl) {
        Ok(rrset_ttl as u32)
    } else {
        Err(format!(
            "rrset_ttl not in range {}-{}",
            RRSET_TTL_RANGE.start(),
            RRSET_TTL_RANGE.end()
        ))
    }
}
