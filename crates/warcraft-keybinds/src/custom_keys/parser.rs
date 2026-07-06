use super::CustomKeys;
use crate::model::{SectionAccumulator, SectionResolution, WarcraftKeybinding};
use std::collections::BTreeMap;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, Debug, Default)]
pub(crate) struct CustomKeysParser {
    entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>,
    current_id: Option<WarcraftObjectId>,
    accumulator: Option<SectionAccumulator>,
}

impl CustomKeysParser {
    pub(crate) fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            current_id: None,
            accumulator: None,
        }
    }

    fn flush_pending_section(&mut self) {
        let maybe_id = self.current_id.take();
        let maybe_accumulator = self.accumulator.take();
        if let Some(object_id) = maybe_id
            && let Some(accumulated) = maybe_accumulator
        {
            let binding = WarcraftKeybinding::from(accumulated);
            self.entries.insert(object_id, binding);
        }
    }

    fn extract_section_id(trimmed_line: &str) -> Option<String> {
        let without_brackets = trimmed_line.strip_prefix('[')?.strip_suffix(']')?;
        let section_id = without_brackets.trim();
        if section_id.is_empty() {
            None
        } else {
            Some(section_id.to_string())
        }
    }

    pub(crate) fn process_line(&mut self, line: &str) {
        let trimmed = line.trim();
        let is_blank = trimmed.is_empty();
        let is_comment = trimmed.starts_with("//") || trimmed.starts_with(';');
        if is_blank || is_comment {
            return;
        }
        if let Some(section_id) = Self::extract_section_id(trimmed) {
            self.flush_pending_section();
            if let Some(resolution) = SectionResolution::from_section_id(&section_id) {
                let already_present = self.entries.contains_key(resolution.canonical_id().value());
                if already_present {
                    self.current_id = None;
                    self.accumulator = None;
                } else {
                    let section_accumulator = SectionAccumulator::new(resolution.kind());
                    self.current_id = Some(resolution.canonical_id());
                    self.accumulator = Some(section_accumulator);
                }
            } else {
                self.current_id = None;
                self.accumulator = None;
            }
        } else if let Some((key, value)) = trimmed.split_once('=')
            && let Some(section_accumulator) = self.accumulator.as_mut()
        {
            section_accumulator.apply(key.trim(), value);
        }
    }

    pub(crate) fn finish(mut self) -> CustomKeys {
        self.flush_pending_section();
        CustomKeys::from(self.entries)
    }
}
