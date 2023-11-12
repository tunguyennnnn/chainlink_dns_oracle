pub mod DnsResolver {
    use crate::services::dns_resolvers::DnsResolver;
    use crate::utils::domain_utils;
    use actix_web::{web, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use trust_dns_resolver::config::*;
    use trust_dns_resolver::Resolver;

    #[derive(Deserialize)]
    pub struct ChainLinkRequest {
        data: serde_json::Value,
        metadata: Option<serde_json::Value>,
        responseURL: Option<String>,
        id: Option<String>,
    }

    #[derive(Serialize)]
    pub struct ChainLinkResponse {
        data: serde_json::Value,
    }

    pub async fn resolve(body: web::Json<ChainLinkRequest>) -> impl Responder {
        let request_data = body.into_inner();
        let data = request_data.data;

        let domain = match data.get("domain").and_then(|v| v.as_str()) {
            Some(domain_str) => domain_str,
            None => "",
        };

        let empty_response: ChainLinkResponse = ChainLinkResponse { data: json!({}) };
        if domain.is_empty() || !domain_utils::is_valid_domain(domain) {
            HttpResponse::Ok().json(empty_response)
        } else {
            let resolver =
                Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

            let a_records = match resolver.lookup_ip(domain) {
                Ok(lookup) => lookup
                    .iter()
                    .map(|ip| ip.to_string())
                    .collect::<Vec<String>>(),
                Err(_) => vec![],
            };

            let ens_address_record = match DnsResolver::resolve_ens_address_record(domain) {
                Ok(record) => record,
                Err(_) => "".to_string(),
            };

            let response: ChainLinkResponse = ChainLinkResponse {
                data: json!({
                    "a_records": a_records,
                    "ens_address_record": ens_address_record,
                }),
            };

            // Return the response
            HttpResponse::Ok().json(response)
        }
    }
}
