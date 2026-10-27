use std::path::Path;

use anyhow::Result;

use crate::index::IndexedImage;

pub async fn write<P: AsRef<Path>>(db_path: P, indexed_images: Vec<IndexedImage>) -> Result<()> {
    let path_str = db_path
        .as_ref()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("invalid DB path"))?;
    let db = turso::Builder::new_local(path_str).build().await?;
    let conn = db.connect()?;

    conn.execute_batch(
        r"DROP TABLE IF EXISTS images;
        CREATE TABLE IF NOT EXISTS images (
            hash        TEXT        PRIMARY KEY,            -- 1.  blake3 hash of pixels + format + size
            path        TEXT        NOT NULL,               -- 2.  path/to/image.png
            name        TEXT        NOT NULL,               -- 3.  image
            format      TEXT        NOT NULL,               -- 4.  PNG, JPG, WEBP
            size        INT         NOT NULL,               -- 5.  in bytes
            width       INT         NOT NULL,               -- 6.  3840
            height      INT         NOT NULL,               -- 7.  2160
            created     INT         NOT NULL,               -- 8.  1792552976580
            modified    INT         NOT NULL,               -- 9.  1792552976580
            rating      INT         DEFAULT 0,              -- 10.
            description TEXT        DEFAULT '',             -- 11.
            tags        TEXT        DEFAULT '[]',             -- 12.
            palette     TEXT        DEFAULT '[]'              -- 13. Array of Oklab
        ) STRICT;",
    ).await?;

    for img in indexed_images {
        let tags_json = serde_json::to_string(&img.tags).expect("Failed to serialize tags");

        let palette_json =
            serde_json::to_string(&img.palette).expect("Failed to serialize palette");

        conn.execute(
            r#"
            INSERT INTO images
            VALUES (
                ?1, ?2, ?3, ?4,
                ?5, ?6, ?7, ?8, ?9,
                ?10, ?11,
                ?12,
                ?13
            )
            "#,
            (
                img.hash.as_str(),
                img.path.to_string(),
                img.name.to_string(),
                img.format.to_string(),
                //
                img.size,
                img.width,
                img.height,
                img.created,
                img.modified,
                //
                img.rating,
                img.description.to_string(),
                //
                tags_json,
                palette_json,
            ),
        )
        .await?;
    }

    Ok(())
}
