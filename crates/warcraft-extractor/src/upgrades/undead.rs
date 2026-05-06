use std::path::PathBuf;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, RaceUpgradeArtDatabase,
    RaceUpgradeNameDatabase, casc_filename,
    upgrades::{RaceUpgradeDatabase, UpgradeFileParser},
};

pub type UndeadUpgradeArtDatabase = RaceUpgradeArtDatabase;
pub type UndeadUpgradeNameDatabase = RaceUpgradeNameDatabase;
pub type UndeadUpgradeDatabase = RaceUpgradeDatabase;

pub static UNDEAD_UPGRADES_ART_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UndeadUpgradesArtExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UndeadUpgradesArtExtraction::process,
};

struct UndeadUpgradesArtExtraction;

impl UndeadUpgradesArtExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && path.ends_with("undeadupgradefunc.txt")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing undead upgrades art with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_art_database = match casc_filename(path).as_str() {
            "undeadupgradefunc.txt" => UpgradeFileParser::parse_art_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::UndeadUpgradesArt(upgrade_art_database))
    }
}

pub static UNDEAD_UPGRADES_NAME_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UndeadUpgradesNameExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UndeadUpgradesNameExtraction::process,
};

struct UndeadUpgradesNameExtraction;

impl UndeadUpgradesNameExtraction {
    fn matches(path: &str) -> bool {
        path.ends_with("undeadupgradestrings.txt") && path.contains("enus.w3mod:units")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing undead upgrades name with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_name_database = match casc_filename(path).as_str() {
            "undeadupgradestrings.txt" => UpgradeFileParser::parse_name_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::UndeadUpgradesName(upgrade_name_database))
    }
}
