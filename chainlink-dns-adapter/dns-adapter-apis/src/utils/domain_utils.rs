use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([a-zA-Z0-9]([-a-zA-Z0-9])*\.)+([a-zA-Z0-9])+$").unwrap();
}

pub fn is_valid_domain(domain: &str) -> bool {
    RE.is_match(&domain)
}
