use crate::settings::Settings;
use actix_web_httpauth::headers::authorization::Basic;
use std::env::current_dir;
use std::path::PathBuf;

pub const TEST_PASSWORD: &str = "password";

pub fn get_example_dir() -> PathBuf {
    current_dir().unwrap().join("example/sites")
}

pub fn get_test_settings() -> Settings {
    Settings {
        sites_root: get_example_dir(),
        traefik_service: String::from("traefik-service@docker"),
        traefik_cert_resolver: Some(String::from("le")),
        auth_password: TEST_PASSWORD.into(),
        deny_prefixes: Vec::new(),
    }
}

pub fn auth_credentials() -> Basic {
    // HACK: Coerce type of `None` correctly
    let password: Option<&str> = None;
    Basic::new(TEST_PASSWORD, password)
}
