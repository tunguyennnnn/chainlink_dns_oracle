pub mod DnsResolver {
    use trust_dns_resolver::config::*;
    use trust_dns_resolver::error::ResolveError;
    use trust_dns_resolver::Resolver;

    pub fn resolve_ens_address_record(domain: &str) -> Result<String, ResolveError> {
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
        let subdomain = format!("_ens.{}", domain);

        let txt_records = resolver.txt_lookup(subdomain)?;

        let record = txt_records
            .iter()
            .flat_map(|txt| {
                txt.txt_data()
                    .iter()
                    .map(|b| String::from_utf8_lossy(b).into_owned())
            })
            .find(|s| s.starts_with("a=0x"));

        match record {
            Some(record) => Ok(record),
            None => Ok("0x".to_string()),
        }
    }
}
