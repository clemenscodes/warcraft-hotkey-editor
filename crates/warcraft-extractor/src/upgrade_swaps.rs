use std::{collections::BTreeSet, path::PathBuf};

use warcraft_slk::{RowView, SlkTable};

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename, is_war3_units_path,
};

pub type UpgradeSwapDatabase = BTreeSet<UnitUpgradeSwapEntry>;

pub static UPGRADE_SWAPS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UpgradeSwapExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UpgradeSwapExtraction::process,
};

/// One upgrade-swap parsed from `units/upgradedata.slk`: an upgrade whose
/// effect replaces a trained unit with another. Ordered by `from_unit_id`
/// first so the emitted table is deterministic across regenerations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UnitUpgradeSwapEntry {
    from_unit_id: String,
    to_unit_id: String,
}

impl UnitUpgradeSwapEntry {
    pub fn new(from_unit_id: String, to_unit_id: String) -> Self {
        Self {
            from_unit_id,
            to_unit_id,
        }
    }

    pub fn from_unit_id(&self) -> &str {
        &self.from_unit_id
    }

    pub fn to_unit_id(&self) -> &str {
        &self.to_unit_id
    }
}

struct UpgradeSwapExtraction;

impl UpgradeSwapExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);
        is_war3_units_path(path) && filename.ends_with("upgradedata.slk")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let table = SlkTable::from(text);
        let database = Self::process_table(table);
        Ok(ExtractResult::UpgradeSwaps(database))
    }

    fn process_table(table: SlkTable) -> UpgradeSwapDatabase {
        let mut database = UpgradeSwapDatabase::new();
        for row in table.into_iter() {
            if let Some(entry) = Self::swap_from_row(&row) {
                database.insert(entry);
            }
        }
        database
    }

    /// A unit-replace upgrade has `effect{N} == "rtma"` in any of its four
    /// effect slots; `code1` is the replaced (from) unit id and `code2` is the
    /// replacement (to) unit id. Rows whose from/to are not unit-id-shaped are
    /// skipped.
    fn swap_from_row(row: &RowView) -> Option<UnitUpgradeSwapEntry> {
        let mut has_replace_effect = false;
        let mut effect_index = 1;
        while effect_index <= 4 {
            let column_name = format!("effect{effect_index}");
            let raw_effect = row.get(&column_name).unwrap_or("").trim();
            if raw_effect == "rtma" {
                has_replace_effect = true;
            }
            effect_index += 1;
        }
        if !has_replace_effect {
            return None;
        }
        let raw_from = row.get("code1").unwrap_or("").trim();
        let raw_to = row.get("code2").unwrap_or("").trim();
        if !Self::looks_like_unit_id(raw_from) || !Self::looks_like_unit_id(raw_to) {
            return None;
        }
        let from_unit_id = raw_from.to_string();
        let to_unit_id = raw_to.to_string();
        let entry = UnitUpgradeSwapEntry::new(from_unit_id, to_unit_id);
        Some(entry)
    }

    /// Unit ids are the four-character lowercase-or-digit shape Warcraft uses
    /// (e.g. `ohun`, `otbk`). Anything else in `code1`/`code2` is not a unit.
    fn looks_like_unit_id(value: &str) -> bool {
        if value.len() != 4 {
            return false;
        }
        value
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
    }
}
