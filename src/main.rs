use azure_core::TransportOptions;
use azure_svc_imds::package_2023_07_01;
use azure_svc_imds::package_2023_07_01::models::Versions;
use futures::executor::block_on;
use reqwest;
use reqwest::header;
use std::sync::Arc;
mod empty_credential;

async fn get_metadata(c: &package_2023_07_01::Client) -> Result<Versions, azure_core::Error> {
    match c.get_versions().await {
	Ok(t) => Ok(t),
	Err(e) => {
	    println!("got err {}", e);
	    Err(e)
	}
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
    let md = match block_on(get_metadata(&c)) {
	Ok(t) => t,
	Err(e) => {
	    println!("got an error from get_metadata(): {}", e);
	    panic!("bad");
	}
    };
    for ver in md.api_versions.iter() {
	println!("Got version {}", ver);
    }
}
