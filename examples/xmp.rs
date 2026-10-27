use anyhow::Result;
use xmpkit::{XmpFile, XmpOptions, XmpValue, core::namespace::ns};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut file = XmpFile::new();
    file.open_with(&args[1], XmpOptions::default().for_update())?;

    let mut meta = file.get_xmp().cloned().unwrap_or_default();

    meta.set_property(ns::XMP, "Rating", 3.into())?;
    meta.set_property(ns::DC, "description", "A random beach picture".into())?;
    meta.set_property(
        ns::DC,
        "subject",
        XmpValue::Array(
            vec!["Sunset", "Beach"]
                .into_iter()
                .map(Into::into)
                .collect(),
        ),
    )?;

    file.put_xmp(meta);
    file.save(&args[1])?;
    file.close();

    let mut file = XmpFile::new();
    file.open(&args[1])?;

    if let Some(meta) = file.get_xmp() {
        println!("{:#?}", meta);

        let rating = meta.get_property(ns::XMP, "Rating");
        println!("Rating: {:?}", rating);

        let description = meta.get_property(ns::DC, "description");
        println!("Description: {:?}", description);

        let tags = meta.get_property(ns::DC, "subject");
        println!("Tags: {:?}", tags);
    } else {
        println!("No XMP metadata found in this image.");
    }

    Ok(())
}
