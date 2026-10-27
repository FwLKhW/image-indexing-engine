use std::path::Path;

use anyhow::Result;

use crate::index::IndexedImage;

pub async fn write(db_path: &Path, indexed_images: &Vec<IndexedImage>) -> Result<()> {
    let path_str = db_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("invalid DB path"))?;
    let db = turso::Builder::new_local(path_str).build().await?;

    let mut conn = db.connect()?;

    conn.execute_batch(
        r"DROP TABLE IF EXISTS images;
        CREATE TABLE IF NOT EXISTS images (
            path        TEXT        PRIMARY KEY,
            name        TEXT        NOT NULL,
            format      TEXT        NOT NULL,
            size        INT         NOT NULL,
            width       INT         NOT NULL,
            height      INT         NOT NULL,
            created     INT         NOT NULL,
            modified    INT         NOT NULL,
            rating      INT         DEFAULT 0,
            description TEXT        DEFAULT '',
            tags        TEXT        DEFAULT '[]',
            hash        TEXT        DEFAULT '',
            palette     TEXT        DEFAULT '[]'
        );",
    )
    .await?;

    let tx = conn.transaction().await?;

    println!("Writing database...");

    for img in indexed_images {
        let tags_json = serde_json::to_string(&img.tags).expect("Failed to serialize tags");

        let palette_json =
            serde_json::to_string(&img.palette).expect("Failed to serialize palette");

        tx.execute(
            r#"
            INSERT INTO images
            VALUES (
                ?1,
                ?2, ?3, ?4, ?5, ?6,
                ?7, ?8,
                ?9, ?10, ?11,
                ?12, ?13
            )
            "#,
            (
                // id
                img.path.to_string(),
                // basic
                img.name.to_string(),
                img.format.to_string(),
                img.size,
                img.width,
                img.height,
                // system
                img.created,
                img.modified,
                // extend
                img.rating,
                img.description.to_string(),
                tags_json,
                // custom
                img.hash.as_str(),
                palette_json,
            ),
        )
        .await?;
    }

    tx.commit().await?;

    println!("{} records written", indexed_images.len());

    Ok(())
}
