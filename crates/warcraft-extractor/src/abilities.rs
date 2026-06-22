use std::{collections::BTreeMap, path::PathBuf};

use warcraft_slk::{RowView, SlkTable};

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename, is_war3_units_path,
};

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
    evasion_chance_per_level: [f32; 4],
    /// For a leveled summon ability, the distinct unit summoned at each level
    /// (`UnitID1..UnitIDN`) in level order — e.g. `[osw1, osw2, osw3]`. These are
    /// the tiers of one summoned unit; empty unless the ability summons two or
    /// more distinct units across its levels.
    tiered_summon_units: Vec<String>,
}

impl AbilityMetadataEntry {
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Per-level chance to evade an attack (0.0..=1.0). `[0.0; 4]` unless the
    /// ability is an evasion ability. Read from the real numeric data field
    /// (`DataA`/`DataD`), never from the tooltip text.
    pub fn evasion_chance_per_level(&self) -> [f32; 4] {
        self.evasion_chance_per_level
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

    pub fn tiered_summon_units(&self) -> &[String] {
        &self.tiered_summon_units
    }
}

struct AbilityMetadataExtraction;

impl AbilityMetadataExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);
        is_war3_units_path(path) && filename.ends_with("abilitydata.slk")
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
            let raw_levels = row.get("levels").unwrap_or("").trim();
            let level_count = raw_levels.parse::<usize>().unwrap_or(0);
            let evasion_chance_per_level = Self::evasion_chances(raw_code, &row, level_count);

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

            let tiered_summon_units = Self::tiered_summon_units(&row, level_count);

            let has_evasion = evasion_chance_per_level.iter().any(|chance| *chance > 0.0);
            if code.is_none()
                && morph_target_unit.is_none()
                && transform_from_unit.is_none()
                && transform_to_unit.is_none()
                && !has_evasion
                && tiered_summon_units.is_empty()
            {
                continue;
            }

            let entry = AbilityMetadataEntry {
                code,
                morph_target_unit,
                transform_from_unit,
                transform_to_unit,
                evasion_chance_per_level,
                tiered_summon_units,
            };
            database.insert(alias.to_string(), entry);
        }
        database
    }

    /// Per-level chance to evade an attack, read from the real numeric data
    /// field — never parsed out of the tooltip. The column that holds the
    /// dodge chance depends on the ability's base mechanic `code`:
    ///
    /// - `AEev` (Evasion — Demon Hunter, creep, Talisman of Evasion): `DataA`
    /// - `ANdb` (Drunken Brawler — Brewmaster, Chen): `DataD` (here `DataA`
    ///   and `DataB` hold the critical-strike chance and multiplier).
    ///
    /// Only the first `level_count` levels are real; `DataA4`/`DataD4` carry
    /// filler values past a 3-level ability, so anything beyond stays `0.0`.
    fn evasion_chances(code: &str, row: &RowView, level_count: usize) -> [f32; 4] {
        let column_prefix = match code {
            "AEev" => "DataA",
            "ANdb" => "DataD",
            _ => return [0.0; 4],
        };
        let mut chances: [f32; 4] = [0.0; 4];
        let usable_levels = level_count.min(chances.len());
        let mut level_index = 0;
        while level_index < usable_levels {
            let column_number = level_index + 1;
            let column_name = format!("{column_prefix}{column_number}");
            let raw_cell = row.get(&column_name).unwrap_or("").trim();
            let parsed_chance = raw_cell.parse::<f32>().unwrap_or(0.0);
            chances[level_index] = parsed_chance;
            level_index += 1;
        }
        chances
    }

    /// The distinct unit summoned at each level of a leveled summon ability,
    /// read from `UnitID1..UnitID{level_count}` in level order. Returns the
    /// ordered distinct unit ids only when two or more distinct units appear
    /// (i.e. the summon has real tiers, like `osw1`/`osw2`/`osw3`); otherwise
    /// empty. Transform/morph abilities carry a single `UnitID1`, so they yield
    /// fewer than two and are excluded.
    fn tiered_summon_units(row: &RowView, level_count: usize) -> Vec<String> {
        let mut ordered_distinct: Vec<String> = Vec::new();
        let mut level_index = 0;
        while level_index < level_count {
            let column_number = level_index + 1;
            let column_name = format!("UnitID{column_number}");
            let raw_value = row.get(&column_name).unwrap_or("").trim();
            if Self::looks_like_unit_id(raw_value) {
                let value = raw_value.to_string();
                if !ordered_distinct.contains(&value) {
                    ordered_distinct.push(value);
                }
            }
            level_index += 1;
        }
        if ordered_distinct.len() >= 2 {
            ordered_distinct
        } else {
            Vec::new()
        }
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
