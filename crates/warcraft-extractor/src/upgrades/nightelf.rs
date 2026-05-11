use std::path::PathBuf;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, RaceUpgradeArtDatabase,
    RaceUpgradeDatabase, RaceUpgradeNameDatabase, casc_filename, upgrades::UpgradeFileParser,
};

pub type NightelfUpgradeArtDatabase = RaceUpgradeArtDatabase;
pub type NightelfUpgradeNameDatabase = RaceUpgradeNameDatabase;
pub type NightelfUpgradeDatabase = RaceUpgradeDatabase;

pub static NIGHTELF_UPGRADES_ART_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: NightelfUpgradesArtExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: NightelfUpgradesArtExtraction::process,
};

struct NightelfUpgradesArtExtraction;

impl NightelfUpgradesArtExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && path.ends_with("nightelfupgradefunc.txt")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing nightelf upgrades art with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_art_database = match casc_filename(path).as_str() {
            "nightelfupgradefunc.txt" => UpgradeFileParser::parse_art_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::NightelfUpgradesArt(upgrade_art_database))
    }
}

pub static NIGHTELF_UPGRADES_NAME_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: NightelfUpgradesNameExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: NightelfUpgradesNameExtraction::process,
};

struct NightelfUpgradesNameExtraction;

impl NightelfUpgradesNameExtraction {
    fn matches(path: &str) -> bool {
        path.ends_with("nightelfupgradestrings.txt") && path.contains("enus.w3mod:units")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing nightelf upgrades name with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_name_database = match casc_filename(path).as_str() {
            "nightelfupgradestrings.txt" => UpgradeFileParser::parse_name_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::NightelfUpgradesName(upgrade_name_database))
    }
}
