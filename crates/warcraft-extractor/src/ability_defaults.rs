use std::collections::BTreeMap;
use std::path::PathBuf;

use warcraft_api::GridCoordinate;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type AbilityDefaultsDatabase = BTreeMap<String, AbilityDefaultsEntry>;

#[derive(Debug, Clone, Default)]
pub struct AbilityDefaultsEntry {
    button_position: Option<GridCoordinate>,
    research_button_position: Option<GridCoordinate>,
    off_button_position: Option<GridCoordinate>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    off_ubertip: Option<String>,
    off_tip: Option<String>,
    off_icon: Option<String>,
    /// Research ID that must be completed before this ability becomes active
    /// (`Requires=` field from the abilityfunc files).
    requires: Option<String>,
}

impl AbilityDefaultsEntry {
    pub fn button_position(&self) -> Option<GridCoordinate> {
        self.button_position
    }

    pub fn research_button_position(&self) -> Option<GridCoordinate> {
        self.research_button_position
    }

    pub fn off_button_position(&self) -> Option<GridCoordinate> {
        self.off_button_position
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }

    pub fn research_ubertip(&self) -> Option<&str> {
        self.research_ubertip.as_deref()
    }

    pub fn off_ubertip(&self) -> Option<&str> {
        self.off_ubertip.as_deref()
    }

    pub fn off_tip(&self) -> Option<&str> {
        self.off_tip.as_deref()
    }

    pub fn off_icon(&self) -> Option<&str> {
        self.off_icon.as_deref()
    }

    pub fn requires(&self) -> Option<&str> {
        self.requires.as_deref()
    }

    pub fn clear_button_position(&mut self) {
        self.button_position = None;
    }

    pub fn clear_research_button_position(&mut self) {
        self.research_button_position = None;
    }

    /// Fill in fields from `other` that this entry left empty. Balance
    /// overlays publish the same `[Axxx]` sections as the base but
    /// sometimes drop individual lines (`Researchbuttonpos`, `Ubertip`,
    /// off-state art, etc.). A "first wins" merge would lose those
    /// drops; this preserves whichever variant published the field.
    pub fn merge_additive(&mut self, other: &AbilityDefaultsEntry) {
        if self.button_position.is_none() {
            self.button_position = other.button_position;
        }
        if self.research_button_position.is_none() {
            self.research_button_position = other.research_button_position;
        }
        if self.off_button_position.is_none() {
            self.off_button_position = other.off_button_position;
        }
        if self.ubertip.is_none() {
            self.ubertip = other.ubertip.clone();
        }
        if self.research_ubertip.is_none() {
            self.research_ubertip = other.research_ubertip.clone();
        }
        if self.off_ubertip.is_none() {
            self.off_ubertip = other.off_ubertip.clone();
        }
        if self.off_tip.is_none() {
            self.off_tip = other.off_tip.clone();
        }
        if self.off_icon.is_none() {
            self.off_icon = other.off_icon.clone();
        }
        if self.requires.is_none() {
            self.requires = other.requires.clone();
        }
    }
}

pub static ABILITY_DEFAULTS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: AbilityDefaultsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: AbilityDefaultsExtraction::process,
};

struct AbilityDefaultsExtraction;

impl AbilityDefaultsExtraction {
    fn matches(path: &str) -> bool {
        if !path.starts_with("war3.w3mod:units") {
            return false;
        }
        let filename = casc_filename(path);
        filename.ends_with("abilityfunc.txt")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let database = Self::parse_all_sections(text);
        Ok(ExtractResult::AbilityDefaults(database))
    }

    /// Parse every `[SectionId]` block in the func file directly.
    ///
    /// The previous implementation routed through `CustomKeys::from`, which
    /// silently drops sections whose ID is not in the compiled game database.
    /// That caused synthetic test IDs and any ability added after the last
    /// database regeneration to be skipped.  This parser accepts every
    /// section header without filtering.
    fn parse_all_sections(text: &str) -> AbilityDefaultsDatabase {
        let mut database = AbilityDefaultsDatabase::new();
        let mut current_id: Option<String> = None;
        let mut pending = AbilityDefaultsEntry::default();

        for raw_line in text.lines() {
            let line = raw_line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                if let Some(finished_id) = current_id.take() {
                    Self::commit_section(&mut database, finished_id, pending);
                    pending = AbilityDefaultsEntry::default();
                }
                let section_id = line[1..line.len() - 1].trim();
                if !section_id.is_empty() {
                    current_id = Some(section_id.to_string());
                }
                continue;
            }

            if current_id.is_none() {
                continue;
            }

            let Some(equals_pos) = line.find('=') else {
                continue;
            };
            let lowercase_key = line[..equals_pos].trim().to_ascii_lowercase();
            let value = line[equals_pos + 1..].trim();
            Self::apply_section_field(&lowercase_key, value, &mut pending);
        }

        if let Some(last_id) = current_id {
            Self::commit_section(&mut database, last_id, pending);
        }

        database
    }

    fn commit_section(
        database: &mut AbilityDefaultsDatabase,
        section_id: String,
        entry: AbilityDefaultsEntry,
    ) {
        let has_data = entry.button_position.is_some()
            || entry.research_button_position.is_some()
            || entry.off_button_position.is_some()
            || entry.ubertip.is_some()
            || entry.research_ubertip.is_some()
            || entry.off_ubertip.is_some()
            || entry.off_tip.is_some()
            || entry.off_icon.is_some()
            || entry.requires.is_some();
        if has_data {
            database.insert(section_id, entry);
        }
    }

    fn apply_section_field(lowercase_key: &str, value: &str, entry: &mut AbilityDefaultsEntry) {
        match lowercase_key {
            "buttonpos" if entry.button_position.is_none() => {
                entry.button_position = GridCoordinate::try_from(value).ok();
            }
            "unbuttonpos" if entry.off_button_position.is_none() => {
                entry.off_button_position = GridCoordinate::try_from(value).ok();
            }
            "researchbuttonpos" if entry.research_button_position.is_none() => {
                entry.research_button_position = GridCoordinate::try_from(value).ok();
            }
            "ubertip" if entry.ubertip.is_none() && !value.is_empty() => {
                entry.ubertip = Some(value.to_string());
            }
            "researchubertip" if entry.research_ubertip.is_none() && !value.is_empty() => {
                entry.research_ubertip = Some(value.to_string());
            }
            "unubertip" if entry.off_ubertip.is_none() && !value.is_empty() => {
                entry.off_ubertip = Some(value.to_string());
            }
            "untip" if entry.off_tip.is_none() && !value.is_empty() => {
                entry.off_tip = Some(value.to_string());
            }
            "unart" | "unicon" if entry.off_icon.is_none() && !value.is_empty() => {
                entry.off_icon = Some(value.to_string());
            }
            "requires" if entry.requires.is_none() && !value.is_empty() => {
                entry.requires = Some(value.to_string());
            }
            _ => {}
        }
    }
}
