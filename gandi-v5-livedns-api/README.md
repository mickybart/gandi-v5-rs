# Gandi V5 LiveDNS API Library

Provides an abstration on top of [Gandi LiveDNS RESTful Api](https://api.gandi.net/docs/livedns/).

A [personal access token](https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html#personal-access-tokens) is required

This library is asynchronous.

## Examples

The Api is the main entrypoint to communicate with Gandi LiveDNS Api.

```
use std:env;
use gandi_v5_livedns_api::{Api, Endpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let personal_access_token = env::var("GANDI_V5_PAT")?;

    let api = Api::build(Endpoint::Prod, &personal_access_token)?;
}
```

## Calls Supported

- [ ] TSIG keys
- [ ] Manage TSIG keys
- [ ] Software configuration information
- [ ] List accepted record types
- [ ] Domains
    - [x] List of domains handled by LiveDNS
    - [ ] Add a new domain to LiveDNS
- [ ] Domain information
    - [x] Show domain's properties
    - [ ] Update domain's properties
- [ ] Zone transfer slaves
- [ ] Zone slaves'IP
- [ ] TSIG keys associated with a domain
- [ ] Manage TSIG key association
- [ ] DNSSEC keys
- [ ] Manipulate a specific DNSSEC key
- [ ] Domain's nameserver information
- [ ] Domain's records
    - [x] List records associated with a domain
    - [ ] Creates a new record
    - [ ] Replace the whole zone with new records
    - [ ] Delete all records
- [ ] Domain's records, by name
    - [x] List records named {rrset_name} associated with this domain
    - [ ] Create a new record whose name is defined by the path
    - [ ] Replace all records named {rrset_name}
    - [ ] Delete all records named {rrset_name}
- [x] Single domain's record, by name and type
    - [x] Get a single single record with its name and type
    - [x] Create a new record whose name and type are defined by the path
    - [x] Overwrites a single record with {rrset_name} and {rrset_type}
    - [x] Delete record with {rrset_name} and {rrset_type}
- [ ] Snapshots of a domain
- [ ] Snapshots operations
- [ ] Generic nameservers
