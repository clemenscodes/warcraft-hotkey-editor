use std::path::PathBuf;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename,
    skins::{SkinDatabase, SkinParser},
};

pub type UnitSkins = SkinDatabase;

pub static UNIT_SKINS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UnitSkinsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UnitSkinsExtraction::process,
};

struct UnitSkinsExtraction;

impl UnitSkinsExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && path.ends_with("unitskin.txt")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing unit skins with {path}");

        let mut text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?
            .to_string();

        if text.starts_with('\u{feff}') {
            text.remove(0);
        }

        let unit_skin_db = match casc_filename(path).as_str() {
            "unitskin.txt" => SkinParser::parse(&text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::UnitSkin(unit_skin_db))
    }
}
