use std::{collections::BTreeMap, path::PathBuf};

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule};

pub type DefaultHotkeysDatabase = BTreeMap<String, DefaultHotkeysEntry>;

pub static DEFAULT_HOTKEYS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: DefaultHotkeysExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: DefaultHotkeysExtraction::process,
};

/// The hotkeys one section of a game text file assigns, verbatim: either
/// Blizzard's shipped `customkeys.txt` or a localized `*strings.txt`. An empty value is kept as `Some("")` (deliberately unbound);
/// `None` means the section does not mention the field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DefaultHotkeysEntry {
    hotkey: Option<String>,
    unhotkey: Option<String>,
    research_hotkey: Option<String>,
}

impl DefaultHotkeysEntry {
    /// Overlays `other` on `self`: every field `other` mentions wins.
    pub fn overlay(&mut self, other: &Self) {
        if other.hotkey.is_some() {
            self.hotkey.clone_from(&other.hotkey);
        }
        if other.unhotkey.is_some() {
            self.unhotkey.clone_from(&other.unhotkey);
        }
        if other.research_hotkey.is_some() {
            self.research_hotkey.clone_from(&other.research_hotkey);
        }
    }

    pub fn hotkey(&self) -> Option<&str> {
        self.hotkey.as_deref()
    }

    pub fn unhotkey(&self) -> Option<&str> {
        self.unhotkey.as_deref()
    }

    pub fn research_hotkey(&self) -> Option<&str> {
        self.research_hotkey.as_deref()
    }
}

struct DefaultHotkeysExtraction;

impl DefaultHotkeysExtraction {
    const SHIPPED_CUSTOM_KEYS: &'static str = "war3.w3mod:customkeys.txt";
    const LOCALIZED_STRINGS_PREFIX: &'static str = "war3.w3mod:_locales/enus.w3mod:units/";

    fn matches(path: &str) -> bool {
        let lowered = path.to_ascii_lowercase().replace('\\', "/");
        let is_localized_strings =
            lowered.starts_with(Self::LOCALIZED_STRINGS_PREFIX) && lowered.ends_with("strings.txt");
        lowered == Self::SHIPPED_CUSTOM_KEYS || is_localized_strings
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let database = Self::parse(text);
        let lowered = path.to_ascii_lowercase();
        if lowered == Self::SHIPPED_CUSTOM_KEYS {
            Ok(ExtractResult::ShippedDefaultHotkeys(database))
        } else {
            Ok(ExtractResult::LocalizedDefaultHotkeys(database))
        }
    }

    /// Reads every section's `Hotkey`, `Unhotkey` and `Researchhotkey` (keys
    /// matched case-insensitively, inline `//` remarks dropped). A section
    /// declared twice is merged field by field with the later value winning,
    /// which is how patch 3.0.0's talent block re-declares its base abilities.
    fn parse(text: &str) -> DefaultHotkeysDatabase {
        let mut database = DefaultHotkeysDatabase::new();
        let mut current_section: Option<String> = None;
        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            if let Some(after_bracket) = line.strip_prefix('[') {
                current_section = after_bracket
                    .split_once(']')
                    .map(|parts| parts.0.trim().to_string());
                continue;
            }
            let Some(section) = current_section.as_ref() else {
                continue;
            };
            let Some((raw_key, raw_value)) = line.split_once('=') else {
                continue;
            };
            let value_without_remark = raw_value
                .split_once("//")
                .map_or(raw_value, |parts| parts.0);
            let value = value_without_remark.trim().to_string();
            let lowercase_key = raw_key.trim().to_ascii_lowercase();
            let section_key = section.clone();
            let entry = database.entry(section_key).or_default();
            match lowercase_key.as_str() {
                "hotkey" => entry.hotkey = Some(value),
                "unhotkey" => entry.unhotkey = Some(value),
                "researchhotkey" => entry.research_hotkey = Some(value),
                _ => {}
            }
        }
        database.retain(|_, entry| {
            entry.hotkey.is_some() || entry.unhotkey.is_some() || entry.research_hotkey.is_some()
        });
        database
    }
}
