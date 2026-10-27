use std::path::Path;

use image_indexing_engine::{db, index::index_folder, paths::DB_PATH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let root = Path::new(&args[1]);
    let indexed_images = index_folder(root)?;

    let db_path = root.join(DB_PATH);
    db::write(db_path, indexed_images).await?;

    Ok(())
}
