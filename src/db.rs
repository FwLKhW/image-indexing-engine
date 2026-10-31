use std::path::Path;

use anyhow::Result;
use rusqlite::{Connection, params};

use crate::{color::rgb2hex_oct, index::IndexedImage};

/// # Errors
///
/// Returns an error if the database connection fails, table creation fails, or execution fails.
pub fn write(db_path: &Path, indexed_images: &[IndexedImage]) -> Result<()> {
    let mut conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS images (
            id          INTEGER PRIMARY KEY,
            path        TEXT,
            name        TEXT,
            format      TEXT,
            size        INTEGER,
            width       INTEGER,
            height      INTEGER,
            created     TEXT,
            modified    TEXT,
            rating      INTEGER,
            tags        TEXT,
            hash        TEXT,
            color_1     INTEGER,
            color_2     INTEGER,
            color_3     INTEGER,
            color_4     INTEGER
        )",
        [],
    )?;

    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO images (
                id, path, name, format, size, width, height, created, modified, rating,
                tags, hash, color_1, color_2, color_3, color_4
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )?;

        for (id, img) in indexed_images.iter().enumerate() {
            let colors_int: Vec<i64> = img.colors.iter().map(rgb2hex_oct).collect();

            stmt.execute(params![
                id as i64,
                img.path,
                img.name,
                img.format,
                img.size as i64,
                img.width as i64,
                img.height as i64,
                img.created.to_string(),
                img.modified.to_string(),
                img.rating as i64,
                img.tags.join("|"),
                img.hash,
                colors_int.first(),
                colors_int.get(1),
                colors_int.get(2),
                colors_int.get(3),
            ])?;
        }
    }
    tx.commit()?;

    Ok(())
}
