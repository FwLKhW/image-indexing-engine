use std::path::Path;

use anyhow::Result;
use xmpkit::{XmpFile, XmpOptions, XmpValue, core::namespace::ns};

#[derive(Debug, Default)]
pub struct Metadata {
    pub rating: u8,
    pub description: String,
    pub tags: Vec<String>,
}

/// # Errors
///
/// Fail if path does not exists.
pub fn read(path: &Path) -> Result<Metadata> {
    let mut file = XmpFile::new();
    file.open(path)?;

    let mut meta = Metadata::default();

    if let Some(xmp) = file.get_xmp() {
        if let Some(rating) = xmp.get_property(ns::XMP, "Rating") {
            meta.rating = u8::try_from(rating.as_int().unwrap_or_default()).unwrap_or_default();
        }
        if let Some(description) = xmp.get_property(ns::DC, "description") {
            meta.description = description.as_str().unwrap_or_default().to_string();
        }
        if let Some(tags) = xmp.get_property(ns::DC, "subject")
            && let xmpkit::XmpValue::Array(tags) = tags
        {
            meta.tags = tags
                .iter()
                .filter_map(|t| t.as_str())
                .map(ToString::to_string)
                .collect();
        }
    }

    Ok(meta)
}

/// # Errors
///
/// Fail if path does not exists or write XMP failed.
pub fn write(path: &Path, meta: &Metadata) -> Result<()> {
    let mut file = XmpFile::new();
    file.open_with(path, XmpOptions::default().for_update())?;

    let mut xmp = file.get_xmp().cloned().unwrap_or_default();

    xmp.set_property(ns::XMP, "Rating", i64::from(meta.rating).into())?;
    xmp.set_property(ns::DC, "description", meta.description.clone().into())?;
    xmp.set_property(
        ns::DC,
        "subject",
        XmpValue::Array(meta.tags.clone().into_iter().map(Into::into).collect()),
    )?;

    file.put_xmp(xmp);
    file.save(path)?;

    Ok(())
}
