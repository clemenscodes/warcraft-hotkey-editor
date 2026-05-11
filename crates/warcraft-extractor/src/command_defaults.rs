use std::{collections::BTreeMap, path::PathBuf};

use warcraft_api::{ColumnIndex, GridCoordinate, RowIndex};

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type CommandDefaultsDatabase = BTreeMap<String, CommandDefaultsEntry>;

pub static COMMAND_DEFAULTS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: CommandDefaultsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: CommandDefaultsExtraction::process,
};

#[derive(Debug, Clone, Default)]
pub struct CommandDefaultsEntry {
    button_position: Option<GridCoordinate>,
    art: Option<String>,
    tip: Option<String>,
    ubertip: Option<String>,
}

impl CommandDefaultsEntry {
    pub fn button_position(&self) -> Option<GridCoordinate> {
        self.button_position
    }

    pub fn art(&self) -> Option<&str> {
        self.art.as_deref()
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }

    pub fn set_button_position(&mut self, value: Option<GridCoordinate>) {
        self.button_position = value;
    }

    pub fn set_art(&mut self, value: Option<String>) {
        self.art = value;
    }

    pub fn set_tip(&mut self, value: Option<String>) {
        self.tip = value;
    }

    pub fn set_ubertip(&mut self, value: Option<String>) {
        self.ubertip = value;
    }
}

struct CommandDefaultsExtraction;

impl CommandDefaultsExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);
        if filename == "commandfunc.txt" {
            return path.starts_with("war3.w3mod:units");
        }
        if filename == "commandstrings.txt" {
            return path.contains("enus.w3mod:units");
        }
        false
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let database = Self::parse(text);
        Ok(ExtractResult::CommandDefaults(database))
    }

    fn parse(text: &str) -> CommandDefaultsDatabase {
        let mut database = CommandDefaultsDatabase::new();
        let mut current_id: Option<String> = None;
        let mut current_entry = CommandDefaultsEntry::default();

        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with(';') {
                continue;
            }
            if let Some(stripped) = line
                .strip_prefix('[')
                .and_then(|rest| rest.strip_suffix(']'))
            {
                Self::flush(&mut database, &mut current_id, &mut current_entry);
                current_id = Some(stripped.trim().to_string());
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let trimmed_key = key.trim().to_ascii_lowercase();
            let trimmed_value = value.trim();
            match trimmed_key.as_str() {
                "buttonpos" => {
                    current_entry.button_position = Self::parse_button_position(trimmed_value);
                }
                "art" if !trimmed_value.is_empty() => {
                    current_entry.art = Some(trimmed_value.to_string());
                }
                "tip" => {
                    let unquoted = Self::strip_outer_quotes(trimmed_value);
                    if !unquoted.is_empty() {
                        current_entry.tip = Some(unquoted.to_string());
                    }
                }
                "ubertip" => {
                    let unquoted = Self::strip_outer_quotes(trimmed_value);
                    if !unquoted.is_empty() {
                        current_entry.ubertip = Some(unquoted.to_string());
                    }
                }
                _ => {}
            }
        }
        Self::flush(&mut database, &mut current_id, &mut current_entry);
        database
    }

    fn strip_outer_quotes(input: &str) -> &str {
        let trimmed = input.trim();
        trimmed
            .strip_prefix('"')
            .and_then(|rest| rest.strip_suffix('"'))
            .unwrap_or(trimmed)
    }

    fn flush(
        database: &mut CommandDefaultsDatabase,
        current_id: &mut Option<String>,
        current_entry: &mut CommandDefaultsEntry,
    ) {
        let Some(id) = current_id.take() else {
            return;
        };
        let entry = std::mem::take(current_entry);
        if entry.button_position.is_none()
            && entry.art.is_none()
            && entry.tip.is_none()
            && entry.ubertip.is_none()
        {
            return;
        }
        database.entry(id).or_insert(entry);
    }

    fn parse_button_position(value: &str) -> Option<GridCoordinate> {
        let mut parts = value.splitn(2, ',');
        let column_str = parts.next()?.trim();
        let row_str = parts.next()?.trim();
        let column_raw = column_str.parse::<u8>().ok()?;
        let row_raw = row_str.parse::<u8>().ok()?;
        let column = ColumnIndex::try_from(column_raw).ok()?;
        let row = RowIndex::try_from(row_raw).ok()?;
        Some(GridCoordinate::new(column, row))
    }
}
