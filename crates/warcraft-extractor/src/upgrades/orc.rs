use std::path::PathBuf;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, RaceUpgradeArtDatabase,
    RaceUpgradeDatabase, RaceUpgradeNameDatabase, casc_filename, upgrades::UpgradeFileParser,
};

pub type OrcUpgradeArtDatabase = RaceUpgradeArtDatabase;
pub type OrcUpgradeNameDatabase = RaceUpgradeNameDatabase;
pub type OrcUpgradeDatabase = RaceUpgradeDatabase;

pub static ORC_UPGRADES_ART_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: OrcUpgradesArtExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: OrcUpgradesArtExtraction::process,
};

struct OrcUpgradesArtExtraction;

impl OrcUpgradesArtExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && path.ends_with("orcupgradefunc.txt")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing orc upgrades art with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_art_database = match casc_filename(path).as_str() {
            "orcupgradefunc.txt" => UpgradeFileParser::parse_art_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::OrcUpgradesArt(upgrade_art_database))
    }
}

pub static ORC_UPGRADES_NAME_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: OrcUpgradesNameExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: OrcUpgradesNameExtraction::process,
};

struct OrcUpgradesNameExtraction;

impl OrcUpgradesNameExtraction {
    fn matches(path: &str) -> bool {
        path.ends_with("orcupgradestrings.txt") && path.contains("enus.w3mod:units")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing orc upgrades name with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_name_database = match casc_filename(path).as_str() {
            "orcupgradestrings.txt" => UpgradeFileParser::parse_name_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::OrcUpgradesName(upgrade_name_database))
    }
}
