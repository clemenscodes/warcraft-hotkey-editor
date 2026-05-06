use std::path::PathBuf;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, RaceUpgradeArtDatabase,
    RaceUpgradeDatabase, RaceUpgradeNameDatabase, casc_filename, upgrades::UpgradeFileParser,
};

pub type HumanUpgradeArtDatabase = RaceUpgradeArtDatabase;
pub type HumanUpgradeNameDatabase = RaceUpgradeNameDatabase;
pub type HumanUpgradeDatabase = RaceUpgradeDatabase;

pub static HUMAN_UPGRADES_ART_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: HumanUpgradesArtExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: HumanUpgradesArtExtraction::process,
};

struct HumanUpgradesArtExtraction;

impl HumanUpgradesArtExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && path.ends_with("humanupgradefunc.txt")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing human upgrades art with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_art_database = match casc_filename(path).as_str() {
            "humanupgradefunc.txt" => UpgradeFileParser::parse_art_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::HumanUpgradesArt(upgrade_art_database))
    }
}

pub static HUMAN_UPGRADES_NAME_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: HumanUpgradesNameExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: HumanUpgradesNameExtraction::process,
};

struct HumanUpgradesNameExtraction;

impl HumanUpgradesNameExtraction {
    fn matches(path: &str) -> bool {
        path.ends_with("humanupgradestrings.txt") && path.contains("enus.w3mod:units")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing human upgrades name with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

        let upgrade_name_database = match casc_filename(path).as_str() {
            "humanupgradestrings.txt" => UpgradeFileParser::parse_name_database(text),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::HumanUpgradesName(upgrade_name_database))
    }
}
