use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
   pub server: Server,
   pub redis: Redis,
   pub jwt: JWT,
}

#[derive(Debug, Deserialize,Clone)]
pub struct Server {
   pub address: String,
}

#[derive(Debug, Deserialize,Clone)]
pub struct Redis {
   pub address: String,
   pub token_retrive_timeout_secound: i64,
}

#[derive(Debug, Deserialize,Clone)]
pub struct JWT {
   pub secret: String,
   pub expire: i64,
   pub maxage: i64,
}

pub fn init(file: String) -> Config {

    let result = std::fs::read_to_string(file);
    if result.is_err() {
        panic!("cannnot load config file: {}", result.as_ref().unwrap_err()  );
    }
    let result = toml::from_str(&result.as_ref().unwrap());
    if result.is_err() {
        panic!("cannnot parse config file: {}", result.as_ref().unwrap_err()  );
    }
    result.unwrap()
}