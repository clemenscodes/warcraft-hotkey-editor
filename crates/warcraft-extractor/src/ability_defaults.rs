use std::collections::BTreeMap;
use std::path::PathBuf;

use warcraft_api::ButtonPosition;
use warcraft_keybinds::CustomKeysFile;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type AbilityDefaultsDatabase = BTreeMap<String, AbilityDefaultsEntry>;

#[derive(Debug, Clone, Default)]
pub struct AbilityDefaultsEntry {
    button_position: Option<ButtonPosition>,
    research_button_position: Option<ButtonPosition>,
    off_button_position: Option<ButtonPosition>,
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
    pub fn button_position(&self) -> Option<ButtonPosition> {
        self.button_position
    }

    pub fn research_button_position(&self) -> Option<ButtonPosition> {
        self.research_button_position
    }

    pub fn off_button_position(&self) -> Option<ButtonPosition> {
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
        let parsed = CustomKeysFile::from(text);
        let requires_map = Self::extract_requires(text);

        let mut database = AbilityDefaultsDatabase::new();
        for entry in parsed.bindings_in_order() {
            let id = entry.id();
            let binding = entry.binding();
            let regular_position = binding
                .button_position()
                .map(|position| ButtonPosition::new(position.column(), position.row()));
            let research_position = binding
                .research_button_position()
                .map(|position| ButtonPosition::new(position.column(), position.row()));
            let off_position = binding
                .unbutton_position()
                .map(|position| ButtonPosition::new(position.column(), position.row()));
            let ubertip = binding.ubertip().map(str::to_owned);
            let research_ubertip = binding.research_ubertip().map(str::to_owned);
            let off_ubertip = binding.un_ubertip().map(str::to_owned);
            let off_tip = binding.un_tip().map(str::to_owned);
            let off_icon = binding.un_icon().map(str::to_owned);
            let requires = requires_map.get(id).cloned();

            if regular_position.is_none()
                && research_position.is_none()
                && off_position.is_none()
                && ubertip.is_none()
                && research_ubertip.is_none()
                && off_ubertip.is_none()
                && off_tip.is_none()
                && off_icon.is_none()
                && requires.is_none()
            {
                continue;
            }

            let entry_data = AbilityDefaultsEntry {
                button_position: regular_position,
                research_button_position: research_position,
                off_button_position: off_position,
                ubertip,
                research_ubertip,
                off_ubertip,
                off_tip,
                off_icon,
                requires,
            };
            database.insert(id.to_string(), entry_data);
        }
        Ok(ExtractResult::AbilityDefaults(database))
    }

    /// Scan the raw func file text for `Requires=` entries per ability section.
    /// The `CustomKeysFile` parser doesn't expose this field, so we scan directly.
    fn extract_requires(text: &str) -> std::collections::HashMap<String, String> {
        let mut result = std::collections::HashMap::new();
        let mut current_id: Option<&str> = None;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                current_id = Some(&line[1..line.len() - 1]);
            } else if let Some(id) = current_id {
                let lower = line.to_ascii_lowercase();
                if let Some(rest) = lower.strip_prefix("requires=") {
                    let value = rest.trim();
                    if !value.is_empty() {
                        result.insert(id.to_string(), value.to_string());
                    }
                }
            }
        }
        result
    }
}
