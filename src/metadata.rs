use std::path::Path;

use anyhow::Result;
use xmpkit::{XmpFile, XmpOptions, XmpValue, core::namespace::ns};

#[derive(Debug, Default)]
pub struct Metadata {
    pub rating: u8,
    pub tags: Vec<String>,
}

pub enum MetadataWriteAction {
    Rating(u8),
    Tagging(String),
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
        if let Some(tags) = xmp.get_property(ns::DC, "subject")
            && let XmpValue::Array(tags) = tags
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
pub fn write(path: &Path, action: MetadataWriteAction) -> Result<()> {
    let mut file = XmpFile::new();
    file.open_with(path, XmpOptions::default().for_update())?;

    let mut xmp = file.get_xmp().cloned().unwrap_or_default();

    match action {
        MetadataWriteAction::Rating(rating) => {
            xmp.set_property(ns::XMP, "Rating", i64::from(rating).into())?;
        }
        MetadataWriteAction::Tagging(tag) => {
            if let Some(tags) = xmp.get_property(ns::DC, "subject")
                && let XmpValue::Array(mut tags) = tags
            {
                if let Some(pos) = tags.iter().position(|t| t.to_string() == tag) {
                    tags.swap_remove(pos);
                } else {
                    tags.push(tag.into());
                }
                xmp.set_property(ns::DC, "subject", XmpValue::Array(tags))?;
            }
        }
    }

    file.put_xmp(xmp);
    file.save(path)?;

    Ok(())
}
