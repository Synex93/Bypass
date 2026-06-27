use crate::{builders::BUILDER_LANGUAGES, database::get_pool};

pub async fn sync_builder_versions() -> Result<(), sqlx::Error> {
    for lang in BUILDER_LANGUAGES {
        for version in lang.versions {
            sqlx::query!(
                r#"
                INSERT INTO builder_versions (language, version)
                VALUES ($1, $2)
                ON CONFLICT (language, version)
                DO NOTHING
                "#,
                lang.language,
                version
            )
            .execute(get_pool()?)
            .await?;
        }
    }

    Ok(())
}
