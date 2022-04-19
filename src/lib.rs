use reqwest;
use serde::Deserialize;
use std::error::Error;

use async_std_resolver::proto::rr::RecordType;
use async_std_resolver::proto::xfer::DnsRequestOptions;
use async_std_resolver::{config, resolver};

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub check_enabled: bool,
    pub prometheus_exporter_enabled: bool,
    pub pull_servers: Vec<String>,
    pub tcp_timeout_seconds: u32,
    pub broken_stations_never_working_timeout_seconds: u32,
    pub broken_stations_timeout_seconds: u32,
    pub checks_timeout_seconds: u32,
    pub click_valid_timeout_seconds: u32,
    pub clicks_timeout_seconds: u32,
    pub mirror_pull_interval_seconds: u32,
    pub update_caches_interval_seconds: u32,
    pub server_name: String,
    pub server_location: String,
    pub server_country_code: String,
    pub check_retries: u32,
    pub check_batchsize: u32,
    pub check_pause_seconds: u32,
    pub api_threads: u32,
    pub cache_type: String,
    pub cache_ttl: u32,
    pub language_replace_filepath: String,
    pub language_to_code_filepath: String,
}

pub async fn get_server_config<P: AsRef<str>>(server: P) -> Result<ApiConfig, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("https://{}/json/config", server.as_ref()))
        .send()
        .await?
        .json::<ApiConfig>()
        .await?;
    Ok(res)
}

pub async fn get_servers() -> Result<Vec<String>, Box<dyn Error>> {
    // Construct a new Resolver with default configuration options
    let resolver = resolver(
        config::ResolverConfig::default(),
        config::ResolverOpts::default(),
    )
    .await?;

    // Lookup the IP addresses associated with a name.
    // This returns a future that will lookup the IP addresses, it must be run in the Core to
    //  to get the actual result.
    let response = resolver
        .lookup(
            "_api._tcp.radio-browser.info",
            RecordType::SRV,
            DnsRequestOptions::default(),
        )
        .await?;

    // There can be many addresses associated with the name,
    //  this can return IPv4 and/or IPv6 addresses
    let list = response
        .iter()
        .filter_map(|entry| entry.as_srv())
        .map(|entry| entry.target().to_string().trim_matches('.').to_string())
        .collect();
    Ok(list)
}
