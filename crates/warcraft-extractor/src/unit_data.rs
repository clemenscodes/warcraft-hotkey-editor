use std::{collections::BTreeMap, path::PathBuf};

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type UnitDataDatabase = BTreeMap<String, UnitDataEntry>;

pub static UNIT_DATA_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UnitDataExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UnitDataExtraction::process,
};

#[derive(Debug, Clone, Default)]
pub struct UnitDataEntry {
    builds: Vec<String>,
    trains: Vec<String>,
    researches: Vec<String>,
    upgrades: Vec<String>,
    sell_items: Vec<String>,
    sell_units: Vec<String>,
    make_items: Vec<String>,
}

impl UnitDataEntry {
    pub fn builds(&self) -> &[String] {
        &self.builds
    }

    pub fn trains(&self) -> &[String] {
        &self.trains
    }

    pub fn researches(&self) -> &[String] {
        &self.researches
    }

    pub fn upgrades(&self) -> &[String] {
        &self.upgrades
    }

    pub fn sell_items(&self) -> &[String] {
        &self.sell_items
    }

    pub fn sell_units(&self) -> &[String] {
        &self.sell_units
    }

    pub fn make_items(&self) -> &[String] {
        &self.make_items
    }

    pub fn is_worker(&self) -> bool {
        !self.builds.is_empty()
    }

    /// Union every id list with `other`'s ids. Balance overlays publish the
    /// same `unitfunc.txt` sections as the base but routinely *drop* lines
    /// (e.g. `htow` in `_balance/custom_v0` is missing the `Researches=Rhpm`
    /// the base file lists). A "first wins" merge would discard Rhpm if
    /// custom_v0 happened to be processed before the base; this method
    /// keeps every id from every variant. Case-insensitive so we don't
    /// double-list the same id with two casings.
    pub fn merge_additive(&mut self, other: &UnitDataEntry) {
        Self::append_missing(&mut self.builds, &other.builds);
        Self::append_missing(&mut self.trains, &other.trains);
        Self::append_missing(&mut self.researches, &other.researches);
        Self::append_missing(&mut self.upgrades, &other.upgrades);
        Self::append_missing(&mut self.sell_items, &other.sell_items);
        Self::append_missing(&mut self.sell_units, &other.sell_units);
        Self::append_missing(&mut self.make_items, &other.make_items);
    }

    fn append_missing(destination: &mut Vec<String>, source: &[String]) {
        for id in source {
            let already_present = destination
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(id));
            if !already_present {
                destination.push(id.clone());
            }
        }
    }
}

struct UnitDataExtraction;

impl UnitDataExtraction {
    fn matches(path: &str) -> bool {
        if !path.starts_with("war3.w3mod:units") {
            return false;
        }
        let filename = casc_filename(path);
        filename.ends_with("unitfunc.txt")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let database = Self::parse(text);
        Ok(ExtractResult::UnitData(database))
    }

    fn parse(text: &str) -> UnitDataDatabase {
        let mut database = UnitDataDatabase::new();
        let mut current_id: Option<String> = None;
        let mut current_entry = UnitDataEntry::default();

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
                "builds" => {
                    current_entry.builds = Self::split_id_list(trimmed_value);
                }
                "trains" => {
                    current_entry.trains = Self::split_id_list(trimmed_value);
                }
                "researches" => {
                    current_entry.researches = Self::split_id_list(trimmed_value);
                }
                "upgrade" => {
                    current_entry.upgrades = Self::split_id_list(trimmed_value);
                }
                "sellitems" => {
                    current_entry.sell_items = Self::split_id_list(trimmed_value);
                }
                "sellunits" => {
                    current_entry.sell_units = Self::split_id_list(trimmed_value);
                }
                "makeitems" => {
                    current_entry.make_items = Self::split_id_list(trimmed_value);
                }
                _ => {}
            }
        }
        Self::flush(&mut database, &mut current_id, &mut current_entry);
        database
    }

    fn flush(
        database: &mut UnitDataDatabase,
        current_id: &mut Option<String>,
        current_entry: &mut UnitDataEntry,
    ) {
        let Some(id) = current_id.take() else {
            return;
        };
        let entry = std::mem::take(current_entry);
        if entry.builds.is_empty()
            && entry.trains.is_empty()
            && entry.researches.is_empty()
            && entry.upgrades.is_empty()
            && entry.sell_items.is_empty()
            && entry.sell_units.is_empty()
            && entry.make_items.is_empty()
        {
            return;
        }
        database.entry(id).or_insert(entry);
    }

    fn split_id_list(raw: &str) -> Vec<String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed == "_" || trimmed == "-" {
            return Vec::new();
        }
        trimmed
            .split(',')
            .map(|piece| piece.trim().to_string())
            .filter(|piece| !piece.is_empty() && piece != "_" && piece != "-")
            .collect()
    }
}
