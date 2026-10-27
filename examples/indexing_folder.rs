use std::{path::Path, time::Instant};

use image_indexing_engine::{
    db,
    index::index_folder,
    paths::{DB_REL_PATH, prepare_dot_folder},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);

    let start = Instant::now();

    prepare_dot_folder(root, false, true)?;
    let indexed_images = index_folder(root)?;

    let db_path = root.join(DB_REL_PATH);
    db::write(&db_path, &indexed_images).await?;

    println!("Take {} seconds", start.elapsed().as_secs_f32());

    Ok(())
}
