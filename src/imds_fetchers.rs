use azure_svc_imds::models::Instance;
pub(crate) use std::collections::HashMap;

trait ImdsField {
    fn get(&self, i: &Instance) -> String;
}

struct AzEnvironment;
impl ImdsField for AzEnvironment {
    fn get(&self, i: &Instance) -> String {
	i.compute.as_ref().unwrap()
	    .az_environment
	    .as_ref()
	    .expect("metadata extraction").to_string()
    }
}

struct AzLocation;
impl ImdsField for AzLocation {
    fn get(&self, i: &Instance) -> String {
	i.compute.as_ref().unwrap()
	    .location
	    .as_ref()
	    .expect("metadata extraction").to_string()
    }
}

struct Name;
impl ImdsField for Name {
    fn get(&self, i: &Instance) -> String {
	i.compute.as_ref().unwrap()
	    .name
	    .as_ref()
	    .expect("metadata extraction").to_string()
    }
}

struct Id;
impl ImdsField for Id {
    fn get(&self, i: &Instance) -> String {
	i.compute.as_ref().unwrap().vm_id.as_ref().expect("metadata extraction").to_string()
    }
}

struct Size;
impl ImdsField for Size {
    fn get(&self, i: &Instance) -> String {
	i.compute.as_ref().unwrap().vm_size.as_ref().expect("metadata extraction").to_string()
    }
}

struct PublicIPv4;
impl ImdsField for PublicIPv4 {
    fn get(&self, i: &Instance) -> String {
	i.network.as_ref().unwrap().interface[0].ipv4.as_ref().unwrap()
	    .ip_address[0].public_ip_address.as_ref().expect("foo").to_string()
    }
}

struct PrivateIPv4;
impl ImdsField for PrivateIPv4 {
    fn get(&self, i: &Instance) -> String {
	i.network.as_ref().unwrap().interface[0].ipv4.as_ref().unwrap()
	    .ip_address[0].private_ip_address.as_ref().expect("foo").to_string()
    }
}

pub struct ImdsClient<'a> {
    functions: HashMap<&'a str, Box<dyn ImdsField>>,
    instance: &'a Instance,
}

impl ImdsClient<'_> {
    pub fn new(i: &Instance) -> ImdsClient {
	let mut table: HashMap<&str, Box<dyn ImdsField>> = HashMap::new();
	table.insert("az-environment", Box::new(AzEnvironment));
	table.insert("az-location", Box::new(AzLocation));
	table.insert("name", Box::new(Name));
	table.insert("id", Box::new(Id));
	table.insert("size", Box::new(Size));
	table.insert("public-ipv4", Box::new(PublicIPv4));
	table.insert("private-ipv4", Box::new(PrivateIPv4));

	ImdsClient{
	    functions: table,
	    instance: i,
	}
    }

    pub fn get(&self, imds_key: &str) -> String {
	self.functions.get(imds_key).unwrap().get(self.instance)
    }
}
