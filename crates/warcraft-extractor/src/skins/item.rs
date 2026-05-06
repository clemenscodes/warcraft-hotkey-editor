use std::path::PathBuf;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename,
    skins::{SkinDatabase, SkinParser},
};

pub type ItemSkins = SkinDatabase;

pub static ITEM_SKINS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: ItemSkinsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: ItemSkinsExtraction::process,
};

struct ItemSkinsExtraction;

impl ItemSkinsExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && path.ends_with("itemfunc.txt")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing item skins with {path}");

        let mut text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?
            .to_string();

        if text.starts_with('\u{feff}') {
            text.remove(0);
        }

        let item_skin_db = match casc_filename(path).as_str() {
            "itemfunc.txt" => SkinParser::parse(&text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::ItemSkin(item_skin_db))
    }
}
