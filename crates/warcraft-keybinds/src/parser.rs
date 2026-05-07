use std::collections::BTreeMap;

use warcraft_api::WarcraftObjectKind;
use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};

use crate::file::CustomKeysFile;
use crate::model::{SectionAccumulator, SectionKind, WarcraftKeybinding};

impl SectionKind {
    fn for_section_id(id: &str) -> Option<Self> {
        let lowercase = id.to_ascii_lowercase();
        if let Some(object) = WARCRAFT_DATABASE
            .iter()
            .find(|(key, _)| key.value().to_ascii_lowercase() == lowercase)
            .map(|(_, object)| object)
        {
            return match object.kind() {
                WarcraftObjectKind::Command => Some(SectionKind::Command),
                _ => Some(SectionKind::Ability),
            };
        }
        if let Some(entry) = WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .find(|entry| entry.section_id().to_ascii_lowercase() == lowercase)
        {
            return Some(SectionKind::System(entry.class()));
        }
        None
    }
}

impl Default for CustomKeysFile {
    fn default() -> Self {
        Self::from("")
    }
}

impl From<&str> for CustomKeysFile {
    fn from(text: &str) -> Self {
        let mut entries: BTreeMap<String, WarcraftKeybinding> = BTreeMap::new();

        let mut current_key: Option<String> = None;
        let mut accumulator: Option<SectionAccumulator> = None;

        let flush = |current_key: &mut Option<String>,
                     accumulator: &mut Option<SectionAccumulator>,
                     entries: &mut BTreeMap<String, WarcraftKeybinding>| {
            let maybe_key = current_key.take();
            let maybe_accumulated = accumulator.take();
            if let Some(key) = maybe_key {
                if let Some(accumulated) = maybe_accumulated {
                    let binding = WarcraftKeybinding::from(accumulated);
                    entries.insert(key, binding);
                }
            }
        };

        for line in text.lines() {
            let trimmed = line.trim();
            let is_blank = trimmed.is_empty();
            let is_comment = trimmed.starts_with("//") || trimmed.starts_with(';');

            let header = if is_blank || is_comment {
                None
            } else {
                SectionAccumulator::section_id_from(trimmed)
            };

            if let Some(original_id) = header {
                flush(&mut current_key, &mut accumulator, &mut entries);

                let key = original_id.to_lowercase();
                if entries.contains_key(&key) {
                    current_key = None;
                    accumulator = None;
                } else if let Some(section_kind) = SectionKind::for_section_id(&original_id) {
                    let section_accumulator = SectionAccumulator::new(section_kind);
                    current_key = Some(key);
                    accumulator = Some(section_accumulator);
                } else {
                    current_key = None;
                    accumulator = None;
                }
            } else if !is_blank
                && !is_comment
                && let Some((key, value)) = trimmed.split_once('=')
                && let Some(section_accumulator) = accumulator.as_mut()
            {
                section_accumulator.apply(key.trim(), value);
            }
        }

        flush(&mut current_key, &mut accumulator, &mut entries);

        CustomKeysFile::from_parts(entries)
    }
}

impl From<String> for CustomKeysFile {
    fn from(text: String) -> Self {
        Self::from(text.as_str())
    }
}
