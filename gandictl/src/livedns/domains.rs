use gandi_v5_livedns_api::Api;

pub async fn list(api: &Api) {
    let domains = api.domains.list().await;

    for domain in domains {
        println!("{}", domain.fqdn);
    }
}

pub async fn information(api: &Api, fqdn: &str) {
    let domain_info = api.domains.information(fqdn).await;

    println!("{:?}", domain_info);
}