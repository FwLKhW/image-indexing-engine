use std::path::Path;

use duckdb::{Connection, Result, params};

use crate::index::IndexedImage;

pub fn write<P: AsRef<Path>>(db_path: P, indexed_images: Vec<IndexedImage>) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        r"DROP TABLE IF EXISTS images;
        CREATE TABLE IF NOT EXISTS images (
            hash        VARCHAR     PRIMARY KEY,            -- 1.  blake3 hash of pixels + format + size
            path        VARCHAR     NOT NULL,               -- 2.  path/to/image.png
            name        VARCHAR     NOT NULL,               -- 3.  image
            format      VARCHAR     NOT NULL,               -- 4.  PNG, JPG, WEBP
            size        UBIGINT     NOT NULL,               -- 5.  in bytes
            width       UINTEGER    NOT NULL,               -- 6.  3840
            height      UINTEGER    NOT NULL,               -- 7.  2160
            created     BIGINT      NOT NULL,               -- 8.  1792552976580
            modified    BIGINT      NOT NULL,               -- 9.  1792552976580
            rating      INTEGER     NOT NULL DEFAULT 0,     -- 10.
            description VARCHAR     NOT NULL DEFAULT '',    -- 11.
            tags        VARCHAR[]   NOT NULL DEFAULT [],    -- 12.
            palette     FLOAT[3][]                          -- 13. Array of Oklab
        );",
    )?;

    indexed_images.iter().for_each(|img| {
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
                CAST(?12 AS VARCHAR[]),
                CAST(?13 AS FLOAT[3][])
            )
            "#,
            params![
                img.hash,
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
                palette_json
            ],
        )
        .expect("Failed to insert image");
    });

    Ok(())
}
