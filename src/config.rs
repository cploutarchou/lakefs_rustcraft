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

struct Config<'a> {
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
}

impl<'a> Config<'a> {
    fn new(
        base_path: &'a str,
        api_key: HashMap<&'a str, &'a str>,
        api_key_prefix: HashMap<&'a str, &'a str>,
        username: Option<&'a str>,
        password: Option<&'a str>,
        access_token: Option<&'a str>,
        ssl_ca_cert: Option<&'a str>,
    ) -> Self {
        Config {
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
        }
    }
}
