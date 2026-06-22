mod abilities;
mod ability_defaults;
mod command_defaults;
mod data_tables;
mod db;
mod default_positions;
mod gameplay_constants;
mod heroes;
mod items;
mod object_texts;
mod skins;
mod strings;
mod system_keybinds;
mod unit_abilities;
mod unit_data;
mod unit_ui_flags;
mod units;
mod upgrade_swaps;
mod upgrades;

mod image;

mod game;

pub use abilities::*;
pub use ability_defaults::*;
pub use command_defaults::*;
pub use data_tables::*;
pub use default_positions::*;
pub use gameplay_constants::*;
pub use heroes::*;
pub use items::*;
pub use object_texts::*;
pub use skins::*;
pub use strings::*;
pub use system_keybinds::*;
pub use unit_abilities::*;
pub use unit_data::*;
pub use unit_ui_flags::*;
pub use units::*;
pub use upgrade_swaps::*;
pub use upgrades::*;

pub use game::GAME_EXTRACTION_RULE;

pub use image::{DdsDecoder, ExtractedImage};

pub use db::WarcraftDataAggregation;

use ::image::{ImageBuffer, Rgba};
use casclib::{CascError, open};
use ddsfile::DxgiFormat;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use thiserror::Error;
use warcraft_api::{GameplayConstants, Race};

#[derive(Debug)]
pub enum ExtractResult {
    IO,
    Heroes(HeroDatabase),
    Units(UnitDatabase),
    UnitAbilities(UnitAbilitiesDatabase),
    AbilityMetadata(AbilityMetadataDatabase),
    UpgradeSwaps(UpgradeSwapDatabase),
    UnitData(UnitDataDatabase),
    UnitUiFlags(UnitUiFlagsDatabase),
    CommandDefaults(CommandDefaultsDatabase),
    AbilityDefaults(AbilityDefaultsDatabase),
    DataTables(DataTablesDatabase),
    DefaultPositions(DefaultPositionsDatabase),
    ObjectTexts(ObjectTextDatabase),
    SystemKeybinds(SystemKeybindsDatabase),
    Items(ItemDatabase),
    AbilitySkin(AbilitySkins),
    ItemSkin(ItemSkins),
    UnitSkin(UnitSkins),
    HumanUpgradesArt(HumanUpgradeArtDatabase),
    HumanUpgradesName(HumanUpgradeNameDatabase),
    HumanAbilityStrings(HumanAbilityStringsDatabase),
    HumanUnitStrings(HumanUnitStringsDatabase),
    NightelfUpgradesArt(NightelfUpgradeArtDatabase),
    NightelfUpgradesName(NightelfUpgradeNameDatabase),
    NightelfAbilityStrings(NightelfAbilityStringsDatabase),
    NightelfUnitStrings(NightelfUnitStringsDatabase),
    OrcUpgradesArt(OrcUpgradeArtDatabase),
    OrcUpgradesName(OrcUpgradeNameDatabase),
    OrcAbilityStrings(OrcAbilityStringsDatabase),
    OrcUnitStrings(OrcUnitStringsDatabase),
    UndeadUpgradesArt(UndeadUpgradeArtDatabase),
    UndeadUpgradesName(UndeadUpgradeNameDatabase),
    UndeadAbilityStrings(UndeadAbilityStringsDatabase),
    UndeadUnitStrings(UndeadUnitStringsDatabase),
    NeutralAbilityStrings(NeutralAbilityStringsDatabase),
    NeutralUnitStrings(NeutralUnitStringsDatabase),
    ItemAbilityStrings(ItemAbilityStringsDatabase),
    ItemUnitStrings(ItemUnitStringsDatabase),
    CampaignUnitStrings(CampaignUnitStringsDatabase),
    CampaignAbilityStrings(CampaignAbilityStringsDatabase),
    CommonAbilityStrings(CommonAbilityStringsDatabase),
    CommonUnitStrings(CommonUnitStringsDatabase),
    GameplayConstants(Box<GameplayConstants>),
}

impl ExtractResult {
    #[must_use]
    pub fn is_io(&self) -> bool {
        matches!(self, Self::IO)
    }
}

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("failed to open CASC storage")]
    OpenFailed,

    #[error("file not found in CASC: {0}")]
    NotFound(String),

    #[error("unsupported DDS format: {0:?}")]
    UnsupportedFormat(Option<DxgiFormat>),

    #[error(transparent)]
    Casc(#[from] CascError),

    #[error(transparent)]
    Dds(#[from] ddsfile::Error),

    #[error(transparent)]
    Image(#[from] ::image::ImageError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Failed to extract heroes from CASC")]
    Heroes,

    #[error("Failed to extract units from CASC")]
    Units,

    #[error("Failed to extract items from CASC")]
    Items,

    #[error("Missing output path")]
    IO,
}

#[derive(Debug, Clone, Copy)]
pub enum ExtractTarget {
    Raw,
    Image,
    /// Like Image, but skips writing if the output file already exists.
    /// Used for fallback CASC paths so the primary path always wins.
    ImageFallback,
    Text,
}

#[derive(Debug, Copy, Clone)]
pub struct ExtractionRule {
    matcher: fn(&str) -> bool,
    target: ExtractTarget,
    output_path: fn(&str, &Path) -> PathBuf,
    processor: fn(&str, &[u8]) -> Result<ExtractResult, ExtractError>,
}

impl ExtractionRule {
    pub const fn new(
        matcher: fn(&str) -> bool,
        target: ExtractTarget,
        output_path: fn(&str, &Path) -> PathBuf,
        processor: fn(&str, &[u8]) -> Result<ExtractResult, ExtractError>,
    ) -> Self {
        Self {
            matcher,
            target,
            output_path,
            processor,
        }
    }

    pub fn matches(&self, path: &str) -> bool {
        (self.matcher)(path)
    }

    pub fn target(&self) -> ExtractTarget {
        self.target
    }

    pub fn process(&self, path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        (self.processor)(path, bytes)
    }
}

pub struct ExtractionPipeline;

impl ExtractionPipeline {
    pub fn run(
        casc_root: &str,
        output_dir: Option<&Path>,
        rules: &[ExtractionRule],
    ) -> Result<Vec<ExtractResult>, ExtractError> {
        let storage = open(casc_root).map_err(|_| ExtractError::OpenFailed)?;
        let mut results = Vec::new();
        let mut extracted_count: u64 = 0;

        for r in storage.files::<String>() {
            let entry = r?;
            let normalized = normalize(entry.get_name());

            let matching_rules: Vec<&ExtractionRule> = rules
                .iter()
                .filter(|extraction_rule| (extraction_rule.matcher)(&normalized))
                .collect();
            if matching_rules.is_empty() {
                continue;
            }

            extracted_count += 1;
            tracing::info!(file = %normalized, count = extracted_count, "extracting");

            let mut bytes = Vec::new();

            match entry
                .open()
                .map_err(|_| ExtractError::OpenFailed)?
                .extract(&mut bytes)
            {
                Ok(_) => (),
                Err(error) => match error {
                    CascError::Io(io_error) => {
                        if !io_error.to_string().contains("code = 1007") {
                            return Err(ExtractError::Casc(casclib::CascError::Io(io_error)));
                        }
                    }
                    _ => return Err(ExtractError::Casc(error)),
                },
            }

            for rule in &matching_rules {
                match rule.target {
                    ExtractTarget::Text => {
                        let result = (rule.processor)(&normalized, &bytes)?;
                        results.push(result);
                    }

                    ExtractTarget::Raw => {
                        let Some(output_dir) = output_dir else {
                            return Err(ExtractError::IO);
                        };
                        let out_path = (rule.output_path)(&normalized, output_dir);
                        if let Some(parent) = out_path.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        std::fs::write(&out_path, &bytes)?;
                    }

                    ExtractTarget::Image | ExtractTarget::ImageFallback => {
                        let Some(output_dir) = output_dir else {
                            return Err(ExtractError::IO);
                        };
                        let base_out_path = (rule.output_path)(&normalized, output_dir);
                        let out_path = base_out_path.with_extension("png");

                        if matches!(rule.target, ExtractTarget::ImageFallback) && out_path.exists()
                        {
                            continue;
                        }

                        let Ok(decoded_image) = DdsDecoder::decode(&bytes) else {
                            tracing::warn!("Failed to decode image");
                            continue;
                        };
                        let invalid_rgba_error =
                            std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid RGBA");
                        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
                            decoded_image.width(),
                            decoded_image.height(),
                            decoded_image.rgba(),
                        )
                        .ok_or(invalid_rgba_error)?;

                        if let Some(parent) = out_path.parent() {
                            std::fs::create_dir_all(parent)?;
                        }

                        buffer.save(out_path)?;
                    }
                }
            }
        }

        tracing::info!(total = extracted_count, "extraction complete");
        Ok(results)
    }
}

pub fn casc_filename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|filename| filename.to_str())
        .unwrap_or(path)
        .to_string()
}

/// True for any CASC path whose `units/` directory lives under `war3.w3mod:`.
///
/// Covers both the base `war3.w3mod:units/...` and the balance overlays at
/// `war3.w3mod:_balance/<variant>.w3mod:units/...`. All overlays carry the
/// same set of structural files (`unitabilities.slk`, `abilityfunc.txt`,
/// etc.) — they're variant data for Custom and Melee balance modes that
/// supplement the base files with additional ability rows.
///
/// Excludes `notused_*.slk` siblings — balance overlays ship abandoned
/// pre-rebalance SLK tables alongside the live ones with different column
/// layouts (e.g. `notused_unitui.slk`'s `inEditor` is column 10, the live
/// `unitui.slk`'s is column 9). Reading them clobbers good rows with stale
/// data once the extractor's "first wins" merge picks the wrong one.
pub fn is_war3_units_path(path: &str) -> bool {
    if !path.starts_with("war3.w3mod:") {
        return false;
    }
    if !path.contains("w3mod:units/") {
        return false;
    }
    let filename = casc_filename(path);
    let filename_lower = filename.to_ascii_lowercase();
    !filename_lower.starts_with("notused_")
}

fn normalize(path: &str) -> String {
    path.replace('\\', "/")
}

#[derive(Debug, Clone)]
pub struct ParsedEntry {
    values: Vec<String>,
}

impl ParsedEntry {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }
}

pub struct SectionedListParser;

impl SectionedListParser {
    pub fn parse(text: &str, value_prefix: &str) -> BTreeMap<String, ParsedEntry> {
        let mut map: BTreeMap<String, ParsedEntry> = BTreeMap::new();

        let mut current_id: Option<String> = None;
        let mut current_values: Vec<String> = Vec::new();

        let flush = |map: &mut BTreeMap<String, ParsedEntry>,
                     id: &mut Option<String>,
                     values: &mut Vec<String>| {
            if let Some(id) = id.take()
                && !values.is_empty()
            {
                map.insert(id.clone(), ParsedEntry::new(values.clone()));
                values.clear();
            }
        };

        for line in text.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some(id) = line
                .strip_prefix('[')
                .and_then(|line_inner| line_inner.strip_suffix(']'))
            {
                flush(&mut map, &mut current_id, &mut current_values);
                current_id = Some(id.to_string());
                continue;
            }

            if let Some(values) = line.strip_prefix(value_prefix) {
                current_values = QuotedListParser::split(values)
                    .into_iter()
                    .map(|path_segment| path_segment.replace('\\', "/"))
                    .collect();
            }
        }

        flush(&mut map, &mut current_id, &mut current_values);

        map
    }
}

struct QuotedListParser;

impl QuotedListParser {
    fn split(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for c in input.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                }
                ',' if !in_quotes => {
                    let trimmed = current.trim();
                    if !trimmed.is_empty() {
                        values.push(trimmed.to_string());
                    }
                    current.clear();
                }
                _ => current.push(c),
            }
        }

        let trimmed = current.trim();
        if !trimmed.is_empty() {
            values.push(trimmed.to_string());
        }

        values
    }
}

pub fn race_from_unit_id(id: &str) -> Option<Race> {
    let first_char = id.chars().next()?;
    race_from_char(first_char)
}

pub fn race_from_char(race: char) -> Option<Race> {
    match race {
        'h' | 'H' => Some(Race::Human),
        'e' | 'E' => Some(Race::Nightelf),
        'o' | 'O' => Some(Race::Orc),
        'u' | 'U' => Some(Race::Undead),
        'n' | 'N' => Some(Race::Neutral),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casc_filename_extracts_unix_basename() {
        assert_eq!(casc_filename("path/to/file.slk"), "file.slk");
    }

    #[test]
    fn casc_filename_with_no_separator_returns_input() {
        assert_eq!(casc_filename("file.slk"), "file.slk");
    }

    #[test]
    fn casc_filename_of_empty_string_is_empty() {
        assert_eq!(casc_filename(""), "");
    }

    #[test]
    fn race_from_char_handles_lowercase() {
        assert_eq!(race_from_char('h'), Some(Race::Human));
        assert_eq!(race_from_char('e'), Some(Race::Nightelf));
        assert_eq!(race_from_char('o'), Some(Race::Orc));
        assert_eq!(race_from_char('u'), Some(Race::Undead));
        assert_eq!(race_from_char('n'), Some(Race::Neutral));
    }

    #[test]
    fn race_from_char_handles_uppercase() {
        assert_eq!(race_from_char('H'), Some(Race::Human));
        assert_eq!(race_from_char('E'), Some(Race::Nightelf));
        assert_eq!(race_from_char('O'), Some(Race::Orc));
        assert_eq!(race_from_char('U'), Some(Race::Undead));
        assert_eq!(race_from_char('N'), Some(Race::Neutral));
    }

    #[test]
    fn race_from_char_returns_none_for_unknown() {
        assert_eq!(race_from_char('x'), None);
        assert_eq!(race_from_char('Z'), None);
        assert_eq!(race_from_char('1'), None);
    }

    #[test]
    fn race_from_unit_id_uses_first_character() {
        assert_eq!(race_from_unit_id("hpea"), Some(Race::Human));
        assert_eq!(race_from_unit_id("egrm"), Some(Race::Nightelf));
        assert_eq!(race_from_unit_id("ogru"), Some(Race::Orc));
        assert_eq!(race_from_unit_id("uaco"), Some(Race::Undead));
        assert_eq!(race_from_unit_id("nvl0"), Some(Race::Neutral));
    }

    #[test]
    fn race_from_unit_id_returns_none_for_empty() {
        assert_eq!(race_from_unit_id(""), None);
    }

    #[test]
    fn race_from_unit_id_returns_none_for_unknown_first_char() {
        assert_eq!(race_from_unit_id("xyz"), None);
    }

    #[test]
    fn normalize_converts_backslashes_to_forward_slashes() {
        assert_eq!(normalize(r"path\to\file.slk"), "path/to/file.slk");
    }

    #[test]
    fn normalize_leaves_forward_slashes_alone() {
        assert_eq!(normalize("path/to/file.slk"), "path/to/file.slk");
    }

    #[test]
    fn normalize_handles_mixed_separators() {
        assert_eq!(normalize(r"a/b\c/d"), "a/b/c/d");
    }

    #[test]
    fn parsed_entry_exposes_values() {
        let values = vec!["a".to_string(), "b".to_string()];
        let entry = ParsedEntry::new(values.clone());
        assert_eq!(entry.values(), values.as_slice());
    }

    #[test]
    fn sectioned_list_parser_extracts_single_section() {
        let input = "[foo]\nArt = \"a.blp\",\"b.blp\"\n";
        let parsed_sections = SectionedListParser::parse(input, "Art = ");
        assert_eq!(parsed_sections.len(), 1);
        let foo_entry = parsed_sections.get("foo").unwrap();
        assert_eq!(
            foo_entry.values(),
            vec!["a.blp".to_string(), "b.blp".to_string()].as_slice()
        );
    }

    #[test]
    fn sectioned_list_parser_normalizes_backslashes_in_values() {
        let input = "[foo]\nArt = \"icons\\custom\\a.blp\"\n";
        let parsed_sections = SectionedListParser::parse(input, "Art = ");
        let foo_entry = parsed_sections.get("foo").unwrap();
        assert_eq!(
            foo_entry.values(),
            vec!["icons/custom/a.blp".to_string()].as_slice()
        );
    }

    #[test]
    fn sectioned_list_parser_handles_multiple_sections() {
        let input = "[alpha]\nArt = \"one.blp\"\n[beta]\nArt = \"two.blp\",\"three.blp\"\n";
        let parsed_sections = SectionedListParser::parse(input, "Art = ");
        assert_eq!(parsed_sections.len(), 2);
        let alpha_entry = parsed_sections.get("alpha").unwrap();
        let beta_entry = parsed_sections.get("beta").unwrap();
        assert_eq!(alpha_entry.values(), vec!["one.blp".to_string()].as_slice());
        assert_eq!(
            beta_entry.values(),
            vec!["two.blp".to_string(), "three.blp".to_string()].as_slice()
        );
    }

    #[test]
    fn sectioned_list_parser_ignores_sections_without_matching_prefix() {
        let input = "[foo]\nUnrelated = something\n";
        let parsed_sections = SectionedListParser::parse(input, "Art = ");
        assert_eq!(parsed_sections.len(), 0);
    }

    #[test]
    fn sectioned_list_parser_handles_last_section_without_trailing_newline() {
        let input = "[foo]\nArt = \"only.blp\"";
        let parsed_sections = SectionedListParser::parse(input, "Art = ");
        let foo_entry = parsed_sections.get("foo").unwrap();
        assert_eq!(foo_entry.values(), vec!["only.blp".to_string()].as_slice());
    }

    #[test]
    fn sectioned_list_parser_drops_sections_with_empty_value_lists() {
        let input = "[foo]\nArt = \n[bar]\nArt = \"kept.blp\"\n";
        let parsed_sections = SectionedListParser::parse(input, "Art = ");
        assert!(!parsed_sections.contains_key("foo"));
        assert!(parsed_sections.contains_key("bar"));
    }

    #[test]
    fn quoted_list_parser_splits_plain_csv() {
        let split = QuotedListParser::split("a,b,c");
        assert_eq!(
            split,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn quoted_list_parser_trims_whitespace_around_values() {
        let split = QuotedListParser::split("  a  ,  b  ");
        assert_eq!(split, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn quoted_list_parser_ignores_commas_inside_quotes() {
        let split = QuotedListParser::split(r#""a,b",c"#);
        assert_eq!(split, vec!["a,b".to_string(), "c".to_string()]);
    }

    #[test]
    fn quoted_list_parser_strips_quotes_from_values() {
        let split = QuotedListParser::split(r#""a","b""#);
        assert_eq!(split, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn quoted_list_parser_skips_empty_values() {
        let split = QuotedListParser::split("a,,b");
        assert_eq!(split, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn quoted_list_parser_handles_empty_input() {
        let split = QuotedListParser::split("");
        assert!(split.is_empty());
    }

    #[test]
    fn quoted_list_parser_handles_single_value() {
        let split = QuotedListParser::split("only");
        assert_eq!(split, vec!["only".to_string()]);
    }

    #[test]
    fn extract_result_is_io_detects_io_variant() {
        assert!(ExtractResult::IO.is_io());
    }
}
