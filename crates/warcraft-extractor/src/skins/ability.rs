use std::path::PathBuf;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename,
    skins::{AbilitySkinIcons, SkinParser},
};

pub type AbilitySkins = AbilitySkinIcons;

pub static ABILITY_SKINS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: AbilitySkinsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: AbilitySkinsExtraction::process,
};

struct AbilitySkinsExtraction;

impl AbilitySkinsExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && path.ends_with("abilityskin.txt")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing ability skins with {path}");

        let mut text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?
            .to_string();

        if text.starts_with('\u{feff}') {
            text.remove(0);
        }

        let ability_skin_icons = match casc_filename(path).as_str() {
            "abilityskin.txt" => SkinParser::parse_ability_icons(&text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::AbilitySkin(ability_skin_icons))
    }
}
