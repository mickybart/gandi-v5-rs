# Projects around Gandi v5

The main goal of this project is to provide:
- libraries on top of [Gandi V5 API](https://api.gandi.net/docs/reference/)
    - [LiveDNS Api](gandi-v5-livedns-api/README.md)
- command line tool to control the gandi.net management console
    - [gandictl](gandictl/README.md)
- a service to update DNS entry for DynamicDNS support
    - [Custom Dynamic DNS service](custom-ddns/README.md)

Currently, there is no target to cover all APIs provided by Gandi V5. This project is mainly build to share a working base and finally provides a DDNS service usable with some TP-Link routers and any custom DDNS compatible routers.

Contribution or submitted issues will be helpful to determine the global interest around Gandi V5 to improve API coverage.

## Libraries

- [gandi-v5-livedns-api](gandi-v5-livedns-api/README.md)

## Tools

- [gandictl](gandictl/README.md) 
- [Custom Dynamic DNS service](custom-ddns/README.md)

## Development

The project is using rust exclusively.

### build

```bash
# dev
cargo build

# release
cargo build --release
```

### coverage

A Gandi [sandbox](https://api.sandbox.gandi.net/docs/sandbox/) account is required.

Tarpaulin [installation](https://github.com/xd009642/tarpaulin?tab=readme-ov-file#installation) is required.

Generate an html report `tarpaulin-report.html`:

```bash
export GANDI_V5_SANDBOX_PAT="YOUR_SANDBOX_PERSONAL_ACCESS_TOKEN"

cargo tarpaulin --engine llvm --out Html --skip-clean --target-dir target/coverage
```

### documentation

```bash
cargo doc
```

## Packaging

- [archlinux](packaging/README.md)
