use std::collections::BTreeSet;
use std::env;
use std::sync::OnceLock;

use bollard::Docker;

use crate::builders::BUILDER_LANGUAGES;

static BUILDER_REGISTRY_IMAGE: OnceLock<String> = OnceLock::new();

fn get_builder_registry_image() -> &'static str {
    BUILDER_REGISTRY_IMAGE.get_or_init(|| {
        env::var("builder_registry_image")
            .unwrap_or_else(|_| "ghcr.io/synex93/bypass-builder".to_string())
    })
}

pub fn expected_builder_images() -> BTreeSet<String> {
    let mut expected = BTreeSet::new();

    for builder in BUILDER_LANGUAGES {
        for version in builder.versions {
            let image = image_name(builder.language, version);
            expected.insert(image);
        }
    }

    expected
}

pub fn image_name(language: &str, version: &str) -> String {
    let base = get_builder_registry_image();
    format!("{base}:{language}-{version}")
}

pub async fn connect_docker() -> Result<Docker, bollard::errors::Error> {
    match env::var("docker_api_addr").ok() {
        Some(url) => Docker::connect_with_http(&url, 120, bollard::API_DEFAULT_VERSION),
        _ => Docker::connect_with_socket_defaults(),
    }
}
