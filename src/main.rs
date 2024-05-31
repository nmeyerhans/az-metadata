use azure_core::TransportOptions;
use azure_svc_imds::models::{IdentityTokenResponse, Instance};
use azure_svc_imds::package_2023_07_01;
use azure_svc_imds::package_2023_07_01::models::Versions;
use futures::executor::block_on;
use once_cell::sync::Lazy;
use reqwest;
use reqwest::header;
use std::sync::Arc;
mod empty_credential;
mod imds_fetchers;

use crate::imds_fetchers::ImdsClient;

async fn get_versions(c: &package_2023_07_01::Client) -> Result<Versions, azure_core::Error> {
    match c.get_versions().await {
	Ok(t) => Ok(t),
	Err(e) => {
	    println!("got err {}", e);
	    Err(e)
	}
    }
}

async fn get_token_info(c: &package_2023_07_01::Client) -> String {
    match c.identity_client().get_info("true").await {
	Ok(t) => t.tenant_id.expect("get_token_info()"),
	Err(e) => format!("Error: {}", e),
    }
}

async fn get_token(c: &package_2023_07_01::Client) -> Result<IdentityTokenResponse, azure_core::Error> {
    match c.identity_client().get_token("true", "https%3A%2F%2Fstorage.azure.com%2F").await {
	Ok(t) => Ok(t),
	Err(e) => {
	    println!("get_token: error: {}", e);
	    Err(e)
	}
    }
}

async fn _get_metadata() -> Instance {
    let transport_options = build_transport_options();
    let c = match azure_svc_imds::ClientBuilder::new(empty_credential::create_empty_credential())
        .endpoint(package_2023_07_01::DEFAULT_ENDPOINT.clone())
        .transport(transport_options)
        .build() {
	    Ok(t) => t,
	    Err(e) => panic!("unable to build client: {}", e),
	};
    match c.instances_client().get_metadata("true").await {
	Ok(r) => r,
	Err(e) => panic!("Unable to retrieve metadata from IMDS endpoint: {}", e),
    }
}

fn build_transport_options() -> TransportOptions {
    let mut headers = header::HeaderMap::new();
    headers.insert("Metadata", header::HeaderValue::from_static("true"));
    let client = match reqwest::Client::builder()
	.default_headers(headers)
	.build() {
	    Ok(t) => t,
	    Err(e) => panic!("bad result from reqweest::Client::builder(): {}", e),
	};
    TransportOptions::new(Arc::new(client))
}

static METADATA: Lazy<Instance> = Lazy::new(|| {
    println!("Retrieving and caching data");
    block_on(_get_metadata())
});

#[tokio::main]
async fn main() {

    // println!("metadata: {:#?}", METADATA.compute);

    let c = ImdsClient::new(&METADATA);
    println!("got environment {}", c.get("az-environment"));
    println!("VM name: {}", c.get("name"));
    println!("VM ID: {}", c.get("id"));
    println!("VM Size: {}", c.get("size"));
    println!("VM location: {}", c.get("az-location"));
    println!("IPv4: {}", c.get("public-ipv4"));
    println!("Private IPv4: {}", c.get("private-ipv4"));
}
