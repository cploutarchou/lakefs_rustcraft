use std::collections::HashMap;
use crate::configuration::Configuration;

struct RestClient {
    configuration: Configuration<'static>,
    pool_threads: u32,
    rest_client: RESTClientObject,
    default_headers: HashMap<String, String>,
    cookie: Option<String>,
    user_agent: String,
    client_side_validation: bool,
}

impl RestClient {
    fn new(
        configuration: Option<Configuration<'static>>,
        header_name: Option<String>,
        header_value: Option<String>,
        cookie: Option<String>,
        pool_threads: u32,
    ) -> Self {
        let configuration = configuration.unwrap_or_else(Configuration::get_default);
        let mut default_headers = HashMap::new();
        if let (Some(header_name), Some(header_value)) = (header_name, header_value) {
            default_headers.insert(header_name, header_value);
        }

        Self {
            configuration,
            pool_threads,
            rest_client: RESTClientObject::new(&configuration),
            default_headers,
            cookie,
            user_agent: String::from("lakefs-rust-sdk/0.1.0-SNAPSHOT"),
            client_side_validation: *&configuration.client_side_validation,
        }
    }
}


struct RESTClientObject;

impl RESTClientObject {
    fn new(_configuration: &Configuration) -> Self {
        // Your RESTClientObject implementation here
        RESTClientObject
    }
}

