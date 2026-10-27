use std::path::PathBuf;

use image_indexing_engine::{db, index::index_folder, paths::DB_PATH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let indexed_images = index_folder(&args[1])?;

    let db_path = PathBuf::from(&args[1]).join(DB_PATH);

    db::write(db_path, indexed_images).await?;

    Ok(())
}
