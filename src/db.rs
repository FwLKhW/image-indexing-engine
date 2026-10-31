use std::path::Path;

use anyhow::Result;
use rusqlite::{Connection, params};

use crate::{color::{dec2rgb, rgb2dec}, index::IndexedImage};

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
            created     INTEGER,
            modified    INTEGER,
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
            let colors_int: Vec<i64> = img.colors.iter().map(rgb2dec).collect();

            stmt.execute(params![
                id.cast_signed(),
                img.path,
                img.name,
                img.format,
                img.size.cast_signed(),
                img.width.cast_signed(),
                img.height.cast_signed(),
                img.created.to_string(),
                img.modified.to_string(),
                img.rating.cast_signed(),
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

/// # Errors
///
/// Returns an error if the database connection fails, the query fails, or row mapping fails.
#[allow(clippy::as_conversions)]
pub fn read(db_path: &Path) -> Result<Vec<IndexedImage>> {
    let conn = Connection::open(db_path)?;

    let mut stmt: rusqlite::Statement<'_> = conn.prepare(
        "SELECT path, name, format, size, width, height, created, modified, rating, tags, hash, color_1, color_2, color_3, color_4 FROM images"
    )?;

    let image_iter = stmt.query_map([], |row| {
        let path = row.get::<_, String>(0)?;
        let name = row.get::<_, String>(1)?;
        let format = row.get::<_, String>(2)?;
        let size = row.get::<_, i64>(3)? as u64;
        let width = row.get::<_, i64>(4)? as u32;
        let height = row.get::<_, i64>(5)? as u32;

        let created = row.get::<_, i64>(6)? as u64;
        let modified = row.get::<_, i64>(7)? as u64;

        let rating = row.get::<_, i64>(8)? as u8;

        let tags_str = row.get::<_, String>(9)?;
        let tags: Vec<String> = if tags_str.is_empty() {
            Vec::new()
        } else {
            tags_str.split('|').map(String::from).collect()
        };

        let hash = row.get::<_, String>(10)?;

        let color_1: Option<i64> = row.get(11)?;
        let color_2: Option<i64> = row.get(12)?;
        let color_3: Option<i64> = row.get(13)?;
        let color_4: Option<i64> = row.get(14)?;

        let colors = [color_1, color_2, color_3, color_4]
            .into_iter()
            .flatten()
            .map(dec2rgb)
            .collect();

        Ok(IndexedImage {
            path,
            name,
            format,
            size,
            width,
            height,
            created,
            modified,
            rating,
            tags,
            hash,
            colors,
        })
    })?;

    let indexed_images = image_iter.filter_map(Result::ok).collect();

    Ok(indexed_images)
}