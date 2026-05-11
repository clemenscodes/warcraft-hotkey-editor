use std::{collections::BTreeMap, path::PathBuf};

use warcraft_api::ItemClass;
use warcraft_slk::SlkTable;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type ItemDatabase = BTreeMap<ItemClass, BTreeMap<String, ItemDefinition>>;

pub static ITEMS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: ItemsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: ItemsExtraction::process,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemDefinition {
    level: u32,
    ability_list: Vec<String>,
    cooldown_id: Option<String>,
}

impl ItemDefinition {
    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn ability_list(&self) -> &[String] {
        &self.ability_list
    }

    pub fn cooldown_id(&self) -> Option<&str> {
        self.cooldown_id.as_deref()
    }
}

struct ItemsExtraction;

impl ItemsExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);

        path.starts_with("war3.w3mod:units") && filename.ends_with("itemdata.slk")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing items with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;

        let table = SlkTable::from(text);

        let item_database = match casc_filename(path).as_str() {
            "itemdata.slk" => Self::process_data(table),
            _ => return Err(ExtractError::Items),
        };

        Ok(ExtractResult::Items(item_database))
    }

    fn parse_ability_list(raw: &str) -> Vec<String> {
        raw.split(',')
            .map(|ability| ability.trim())
            .filter(|ability| !ability.is_empty() && *ability != "-" && *ability != "_")
            .map(|ability| ability.to_string())
            .collect()
    }

    fn process_data(table: SlkTable) -> ItemDatabase {
        let mut item_database: ItemDatabase = ItemDatabase::new();

        for row in table.into_iter() {
            let id = row.get("itemID").unwrap_or("").trim();
            let class = row.get("class").unwrap_or("").trim();
            let level = row.get("Level").unwrap_or("0").trim();
            let ability_list = row.get("abilList").unwrap_or("").trim();
            let cooldown_id = row
                .get("cooldownID")
                .map(|cooldown_id| cooldown_id.to_string());

            if id.is_empty() {
                continue;
            }

            let Ok(item_class) = ItemClass::try_from(class) else {
                continue;
            };

            let level: u32 = level.parse().unwrap_or(0);
            let ability_list = Self::parse_ability_list(ability_list);

            let item = ItemDefinition {
                level,
                ability_list,
                cooldown_id,
            };

            item_database
                .entry(item_class)
                .or_default()
                .insert(id.to_string(), item);
        }

        item_database
    }
}
