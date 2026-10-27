use std::path::Path;

use anyhow::Result;
use rusqlite::Connection;

use crate::index::IndexedImage;

pub fn write(db_path: &Path, indexed_images: &Vec<IndexedImage>) -> Result<()> {
    let mut conn = Connection::open(db_path)?;

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
    )?;

    let tx = conn.transaction()?;

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
                img.size as i64,
                img.width,
                img.height,
                // system
                img.created as i64,
                img.modified as i64,
                // extend
                img.rating,
                img.description.to_string(),
                tags_json,
                // custom
                img.hash.as_str(),
                palette_json,
            ),
        )?;
    }

    tx.commit()?;

    println!("{} records written", indexed_images.len());

    Ok(())
}
