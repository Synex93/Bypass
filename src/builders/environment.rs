use std::env;
use std::sync::OnceLock;

use bollard::{Docker, query_parameters::CreateImageOptionsBuilder};
use futures_util::StreamExt;

use crate::utils::docker_helper::{connect_docker, expected_builder_images};

static AUTO_PULL: OnceLock<bool> = OnceLock::new();

fn builder_auto_pull() -> bool {
    *AUTO_PULL.get_or_init(|| {
        env::var("builder_auto_pull")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false)
    })
}

pub async fn ensure_builder() -> anyhow::Result<()> {
    let docker = connect_docker().await?;
    let expected = expected_builder_images();

    let auto_pull = builder_auto_pull();

    let mut found = Vec::new();
    let mut missing = Vec::new();

    for image in expected {
        match image_exists(&image).await? {
            true => {
                tracing::debug!(%image, "builder image found");
                found.push(image);
            }
            false => {
                tracing::warn!(%image, "builder image missing");
                missing.push(image.clone());

                if auto_pull {
                    tracing::info!(%image, "auto pulling builder image");

                    if let Err(e) = pull_image(&docker, &image).await {
                        tracing::error!(%image, error = %e, "auto pull failed");
                    } else {
                        tracing::info!(%image, "auto pull success");
                    }
                }
            }
        }
    }

    super::repo::sync_builder_versions().await?;

    tracing::info!(
        found = found.len(),
        missing = missing.len(),
        auto_pull,
        "builder check finished"
    );

    Ok(())
}

pub async fn image_exists(image: &str) -> Result<bool, bollard::errors::Error> {
    let docker = connect_docker().await?;
    match docker.inspect_image(image).await {
        Ok(_) => Ok(true),
        Err(bollard::errors::Error::DockerResponseServerError {
            status_code: 404, ..
        }) => Ok(false),
        Err(e) => Err(e),
    }
}

pub async fn pull_image(docker: &Docker, image: &str) -> Result<(), bollard::errors::Error> {
    let options = CreateImageOptionsBuilder::default()
        .from_image(image)
        .build();

    let mut stream = docker.create_image(Some(options), None, None);

    while let Some(item) = stream.next().await {
        let _info = item?;
    }

    tracing::info!(image = %image, "pull image success");

    Ok(())
}
