use std::path::Path;

use anyhow::Result;
use index::metadata::{self, Metadata};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let default_img_path = String::from("fixtures/imgs/train.webp");
    let img_path = args.get(1).unwrap_or(&default_img_path);
    let img_path = Path::new(img_path);

    metadata::write(
        img_path,
        &Metadata {
            rating: 5,
            description: "A random description.".to_string(),
            tags: vec![
                "something".to_string(),
                "not".to_string(),
                "right".to_string(),
            ],
        },
    )?;

    Ok(())
}
