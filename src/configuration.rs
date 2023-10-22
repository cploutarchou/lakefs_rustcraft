use std::collections::HashMap;

const JSON_SCHEMA_VALIDATION_KEYWORDS: [&str; 10] = [
    "multipleOf",
    "maximum",
    "exclusiveMaximum",
    "minimum",
    "exclusiveMinimum",
    "maxLength",
    "minLength",
    "pattern",
    "maxItems",
    "minItems",
];

pub struct Configuration<'a> {
    base_path: &'a str,
    server_index: i32,
    server_operation_index: HashMap<&'a str, i32>,
    server_variables: HashMap<&'a str, &'a str>,
    server_operation_variables: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    api_key: HashMap<&'a str, &'a str>,
    api_key_prefix: HashMap<&'a str, &'a str>,
    username: Option<&'a str>,
    password: Option<&'a str>,
    access_token: Option<&'a str>,
    ssl_ca_cert: Option<&'a str>,
    pub(crate) client_side_validation: bool,
}

impl<'a> Configuration<'a> {
    fn new(
        base_path: &'a str,
        api_key: HashMap<&'a str, &'a str>,
        api_key_prefix: HashMap<&'a str, &'a str>,
        username: Option<&'a str>,
        password: Option<&'a str>,
        access_token: Option<&'a str>,
        ssl_ca_cert: Option<&'a str>,
    ) -> Self {
        Configuration {
            base_path,
            server_index: 0,
            server_operation_index: HashMap::new(),
            server_variables: HashMap::new(),
            server_operation_variables: HashMap::new(),
            api_key,
            api_key_prefix,
            username,
            password,
            access_token,
            ssl_ca_cert,
            client_side_validation: true,
        }
    }

    pub(crate) fn get_default() -> Self {
        Configuration {
            base_path: "",
            server_index: 0,
            server_operation_index: HashMap::new(),
            server_variables: HashMap::new(),
            server_operation_variables: HashMap::new(),
            api_key: HashMap::new(),
            api_key_prefix: HashMap::new(),
            username: None,
            password: None,
            access_token: None,
            ssl_ca_cert: None,
            client_side_validation: true,
        }
    }
}
