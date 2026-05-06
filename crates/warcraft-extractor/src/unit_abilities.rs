use std::{collections::BTreeMap, path::PathBuf};

use warcraft_slk::SlkTable;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type UnitAbilitiesDatabase = BTreeMap<String, UnitAbilitiesEntry>;

pub static UNIT_ABILITIES_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UnitAbilitiesExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UnitAbilitiesExtraction::process,
};

#[derive(Debug, Clone, Default)]
pub struct UnitAbilitiesEntry {
    abilities: Vec<String>,
    hero_abilities: Vec<String>,
}

impl UnitAbilitiesEntry {
    pub fn abilities(&self) -> &[String] {
        &self.abilities
    }

    pub fn hero_abilities(&self) -> &[String] {
        &self.hero_abilities
    }
}

struct UnitAbilitiesExtraction;

impl UnitAbilitiesExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);
        path.starts_with("war3.w3mod:units") && filename.ends_with("unitabilities.slk")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;

        let table = SlkTable::from(text);
        let database = Self::process_table(table);
        Ok(ExtractResult::UnitAbilities(database))
    }

    fn process_table(table: SlkTable) -> UnitAbilitiesDatabase {
        let mut database = UnitAbilitiesDatabase::new();
        for row in table.into_iter() {
            let unit_id = row.get("unitAbilID").unwrap_or("").trim();
            if unit_id.is_empty() {
                continue;
            }
            let raw_abilities = row.get("abilList").unwrap_or("");
            let raw_hero_abilities = row.get("heroAbilList").unwrap_or("");

            let abilities = Self::split_ability_list(raw_abilities);
            let hero_abilities = Self::split_ability_list(raw_hero_abilities);

            if abilities.is_empty() && hero_abilities.is_empty() {
                continue;
            }

            let entry = UnitAbilitiesEntry {
                abilities,
                hero_abilities,
            };
            database.insert(unit_id.to_string(), entry);
        }
        database
    }

    fn split_ability_list(raw: &str) -> Vec<String> {
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
