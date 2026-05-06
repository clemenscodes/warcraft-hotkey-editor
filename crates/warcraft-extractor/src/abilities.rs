use std::{collections::BTreeMap, path::PathBuf};

use warcraft_slk::SlkTable;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type AbilityMetadataDatabase = BTreeMap<String, AbilityMetadataEntry>;

pub static ABILITY_METADATA_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: AbilityMetadataExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: AbilityMetadataExtraction::process,
};

/// Per-ability data extracted from `units/abilitydata.slk`.
///
/// `code` carries the four-character mechanic class (e.g. `Apit` for
/// "Purchase Item", `Aave` for "Avenger Form") which is what determines
/// runtime behavior independent of the alias used in a unit's `abilList`.
/// `morph_target_unit` is the destination unit id parsed from the SLK's
/// `UnitID1` column for one-way morph abilities — used by consumers to
/// suppress the morph trigger on the unit it morphs *into*.
///
/// `transform_from_unit` / `transform_to_unit` are unit-ID-shaped values from
/// `DataA1` / `DataB1`. For "Call to Arms"-style abilities (e.g. `Amil`) they
/// encode the base unit (`DataA1 = hpea`) and the transformed unit
/// (`DataB1 = hmil`). Used to filter inherited source-form abilities from the
/// transform-target unit's ability list.
#[derive(Debug, Clone, Default)]
pub struct AbilityMetadataEntry {
    code: Option<String>,
    morph_target_unit: Option<String>,
    transform_from_unit: Option<String>,
    transform_to_unit: Option<String>,
}

impl AbilityMetadataEntry {
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    pub fn morph_target_unit(&self) -> Option<&str> {
        self.morph_target_unit.as_deref()
    }

    pub fn transform_from_unit(&self) -> Option<&str> {
        self.transform_from_unit.as_deref()
    }

    pub fn transform_to_unit(&self) -> Option<&str> {
        self.transform_to_unit.as_deref()
    }
}

struct AbilityMetadataExtraction;

impl AbilityMetadataExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);
        path.starts_with("war3.w3mod:units") && filename.ends_with("abilitydata.slk")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let table = SlkTable::from(text);
        let database = Self::process_table(table);
        Ok(ExtractResult::AbilityMetadata(database))
    }

    fn process_table(table: SlkTable) -> AbilityMetadataDatabase {
        let mut database = AbilityMetadataDatabase::new();
        for row in table.into_iter() {
            let alias = row.get("alias").unwrap_or("").trim();
            if alias.is_empty() {
                continue;
            }
            let raw_code = row.get("code").unwrap_or("").trim();
            let raw_morph_target = row.get("UnitID1").unwrap_or("").trim();
            let raw_data_a1 = row.get("DataA1").unwrap_or("").trim();
            let raw_data_b1 = row.get("DataB1").unwrap_or("").trim();

            let code = if raw_code.is_empty() {
                None
            } else {
                Some(raw_code.to_string())
            };
            let morph_target_unit = if Self::looks_like_unit_id(raw_morph_target) {
                Some(raw_morph_target.to_string())
            } else {
                None
            };
            let transform_from_unit = if Self::looks_like_unit_id(raw_data_a1) {
                Some(raw_data_a1.to_string())
            } else {
                None
            };
            let transform_to_unit = if Self::looks_like_unit_id(raw_data_b1) {
                Some(raw_data_b1.to_string())
            } else {
                None
            };

            if code.is_none()
                && morph_target_unit.is_none()
                && transform_from_unit.is_none()
                && transform_to_unit.is_none()
            {
                continue;
            }

            let entry = AbilityMetadataEntry {
                code,
                morph_target_unit,
                transform_from_unit,
                transform_to_unit,
            };
            database.insert(alias.to_string(), entry);
        }
        database
    }

    /// `UnitID1` is overloaded — for non-morph abilities it can hold area
    /// values, durations, or be empty / `_` / `-`. Treat it as a unit id
    /// only when the cell is the four-character lowercase shape Warcraft
    /// uses for unit ids (e.g. `ubsp`, `eden`).
    fn looks_like_unit_id(value: &str) -> bool {
        if value.len() != 4 {
            return false;
        }
        value
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
    }
}
