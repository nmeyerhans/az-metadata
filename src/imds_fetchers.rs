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

	ImdsClient{
	    functions: table,
	    instance: i,
	}
    }

    pub fn get(&self, imds_key: &str) -> String {
	self.functions.get(imds_key).unwrap().get(self.instance)
    }
}
