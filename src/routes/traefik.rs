use crate::config::{Config, RETRY_COUNT};
use crate::site::Site;
use actix_web::{web, HttpResponse};
use serde_json::{json, Value};
use std::collections::HashMap;

const DEFAULT_MIDDLEWARE_NAME: &str = "tp-default";

fn get_router_name(site: &Site) -> String {
    format!("router-{}", site.get_hostname().replace(".", "-"))
}

/// Routers look funny, so no point defining as a struct
fn serialize_router(site: &Site, config: &Config) -> Value {
    let mut router = json!({
        "rule": format!("Host(`{}`)", site.get_hostname()),
        "service": &config.traefik_service,
        "middlewares": [DEFAULT_MIDDLEWARE_NAME]
    });
    if let Some(cert_resolver) = &config.traefik_cert_resolver {
        router.as_object_mut().unwrap().insert(
            String::from("tls"),
            json!({ "certResolver": cert_resolver }),
        );
    }
    router
}

fn get_middleware() -> Value {
    json!({
        DEFAULT_MIDDLEWARE_NAME: {
            "chain": {
                "middlewares": [
                    "tp-compress",
                    "tp-retry"
                ]
            }
        },
        "tp-compress": {
            "compress": {}
        },
        "tp-retry": {
            "retry": {
                "attempts": RETRY_COUNT
            }
        }
    })
}

pub async fn traefik_provider(config: web::Data<Config>) -> HttpResponse {
    let sites = match Site::discover_all(&config.sites_root).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let routers: HashMap<String, Value> = sites
        .iter()
        .map(|s| (get_router_name(s), serialize_router(s, &config)))
        .collect();

    HttpResponse::Ok().json(json!({
        "http": {
            "routers": routers,
            "middlewares": get_middleware()
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_middleware() {
        let middleware = get_middleware();
        let default_middleware = &middleware[DEFAULT_MIDDLEWARE_NAME];
        let chain_middlewares = default_middleware["chain"]["middlewares"]
            .as_array()
            .unwrap();
        assert_eq!(chain_middlewares.len(), 2);
        for m in chain_middlewares.iter() {
            assert!(middleware.get(m.as_str().unwrap()).is_some());
        }
    }
}