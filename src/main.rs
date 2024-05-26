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

fn get_name() -> &'static String {
    METADATA.compute.as_ref().unwrap().name.as_ref().expect("metadata extraction failed")
}

fn get_id() -> &'static String {
    METADATA.compute.as_ref().unwrap().vm_id.as_ref().expect("metadata extraction failed")
}

fn get_az_environment() -> &'static String {
    METADATA.compute.as_ref().unwrap().az_environment.as_ref().expect("metadata extraction failed")
}

fn get_vm_size() -> &'static String {
    METADATA.compute.as_ref().unwrap().vm_size.as_ref().expect("metadata extraction failed")
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
    println!("Hello, world!");
    let transport_options = build_transport_options();
    let c = match azure_svc_imds::ClientBuilder::new(empty_credential::create_empty_credential())
        .endpoint(package_2023_07_01::DEFAULT_ENDPOINT.clone())
        .transport(transport_options)
        .build() {
	    Ok(t) => t,
	    Err(e) => panic!("unable to build client: {}", e),
	};
    let md = match block_on(get_versions(&c)) {
	Ok(t) => t,
	Err(e) => {
	    println!("got an error from get_versions(): {}", e);
	    panic!("bad");
	}
    };
    for ver in md.api_versions.iter() {
	println!("Got version {}", ver);
    }
    let tokeninfo = get_token_info(&c).await;
    println!("token: {}", tokeninfo);
    let token = get_token(&c).await;
    println!("Got a token with lifetime {}", token.unwrap().expires_in.expect("token"));

    println!("metadata: {:#?}", METADATA.compute);
    println!("VM is named {}", get_name());
    println!("VM ID is {}", get_id());
    println!("AZ environment: {}", get_az_environment());
    println!("VM size: {}", get_vm_size());
}
