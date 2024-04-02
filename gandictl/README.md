# gandictl

gandictl controls the gandi.net management console

## cli

```bash
gandictl -h

gandictl controls the gandi.net management console

Usage: gandictl [OPTIONS] <COMMAND>

Commands:
  live-dns  LiveDNS API (<https://api.gandi.net/docs/livedns/>)
  help      Print this message or the help of the given subcommand(s)

Options:
  -s             Gandi Sandbox Api
  -h, --help     Print help
  -V, --version  Print version
```

### Personal Access Token

A [Personal Access Token](https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html#personal-access-tokens) is required.

You need to export it with an environment variable before using `gandictl`.

```bash
export GANDI_V5_PAT="YOU_PERSONAL_ACCESS_TOKEN"
```

### Sandbox

By default, `gandictl` will use Gandi production endpoint.

To use the Gandi Sandbox Api, use the flag `-s`.

```bash
gandictl -s -h
```

### Use cases
#### Using as a DynamicDNS solution

You can use a cronjob to update your public IP with a small script like:

```bash
set -e

FQDN=example.org
NAME=test

export GANDI_V5_PAT="YOU_PERSONAL_ACCESS_TOKEN"

# ipv4
gandictl live-dns apply record $FQDN $NAME A --rrset-ttl 300 --rrset-values $(curl -s https://ipv4.seeip.org/)

# ipv6
gandictl live-dns apply record $FQDN $NAME AAAA --rrset-ttl 300 --rrset-values $(curl -s https://ipv6.seeip.org/)
```

In [packaging](../packaging), an alternative with systemd, by using a service and a timer, is available.

### Commands
#### LiveDNS

```bash
gandictl live-dns -h

LiveDNS API (<https://api.gandi.net/docs/livedns/>)

Usage: gandictl live-dns <COMMAND>

Commands:
  get     Display one or many resources
  apply   Overwrite one or many resources
  create  Create one or many resources
  delete  Delete one or many resources
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

## Packaging

see [packaging](../packaging/README.md)
