# Custom Dynamic DNS Service provider

Most router lists common Dynamic DNS service providers (DDNS). Gandi is not one of them but some routers can add a custom DDNS entry.

Custom Dynamic DNS Service provider is built for this use case by providing a microservice with an endpoint that can be added to a custom DDNS entry.

## Routers
### custom DDNS entry

In your router, add a custom DDNS entry. This entry will be composed of an url and some parameters replaced by the router.

| custom | description | example |
|--------|-------------|---------|
| {scheme} | http or https | https |
| {ip}   | ip where the custom ddns service is running | 192.168.0.10 |
| {rrset_name} | the dns record to update (without the domain) | home |
| {rrset_type} | A or AAAA type if you respectively want to update an ipv4 or ipv6 address | A |
| {rrset_ttl} | the ttl to use | 300 |

#### TP-Link

Possible update URL:
- {scheme}://[USERNAME]:[PASSWORD]@{ip}/gandi/[DOMAIN]/{rrset_name}/{rrset_type}/[IP]?rrset_ttl={rrset_ttl}
- {scheme}://[USERNAME]:[PASSWORD]@{ip}/gandi/[DOMAIN]/{rrset_name}/{rrset_type}/[IP]

Example:

```yaml
# The service will listen to 192.168.0.10:3000

# TLS, ipv4 and ttl set
https://[USERNAME]:[PASSWORD]@192.168.0.10:3000/gandi/[DOMAIN]/home/A/[IP]?rrset_ttl=300

# TLS, ipv4, default ttl will be used
https://[USERNAME]:[PASSWORD]@192.168.0.10:3000/gandi/[DOMAIN]/home/A/[IP]

# Without TLS (not recommanded)
http://[USERNAME]:[PASSWORD]@192.168.0.10:3000/gandi/[DOMAIN]/home/A/[IP]
```

## Custom DDNS Service
### Configuration

A config file is required and need to be created under `config/prod.yaml`.

```yaml
listen: ''                    # IP:PORT
default_rrset_ttl:            # value in second 
whitelist:
  DOMAIN:                     # a domain to manage; eg: example.org
    personal_access_token: '' # Gandi personal access token <https://docs.gandi.net/en/managing_an_organization/organizations/personal_access_token.html#personal-access-tokens>
    records:                  # a list of record that can be updated
    - rrset_name: ''          # eg: 'test' (test.example.org)
      rrset_type: ''          # 'A' for ipv4, 'AAAA' for ipv6
      rrset_ttl_max:          # max ttl that can be set (in second)
    authorizations:           # a list of "who" is authorized to update this domain and relative records
    - ''                      # eg: 'Basic dGVzdDoxMjM0' where dGVzdDoxMjM0 is equal to [USERNAME]:[PASSWORD] encoded in base64
```

#### authorizations

Routers are using basic auth `https://[USERNAME]:[PASSWORD]@...`.

To determine values to put on the authorizations list, you can encode it with this bash command:

```bash
# replace [USERNAME] with the value set in your router (custom DDNS entry)
# replace [PASSWORD] with the value set in your router (custom DDNS entry)
echo "Basic $(echo -n [USERNAME]:[PASSWORD] | base64)"

# example with username test and password 1234
echo "Basic $(echo -n test:1234 | base64)"

# ouput to set on your authorization list will be: Basic dGVzdDoxMjM0
```

#### Reasons about authorizations and records whitelist

Custom Dynamic DNS Service uses a strong separation between the router and the DNS provider. This is mandatory to limit the attack surface if a malicious person is able to hack the router. Due to some lack of support or routers firmware upgrade not applied by customer, by using an explicit whitelist, we can protect all records of your zone except those whitelisted from an attacker.

### Build image

*build from root folder (cd ..)*

```bash
docker build -t cddns:latest -f custom-ddns/Dockerfile .
```

### Run image

Create `config` folder and add `prod.yaml` file.

```bash
# example with local.yaml provided in the project
docker run -it --rm -v $(pwd)/custom-ddns/config:/config:ro -p 3000:3000 cddns
```
