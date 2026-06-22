use std::{collections::BTreeMap, path::PathBuf};

use warcraft_api::{Race, UnitKind};
use warcraft_slk::SlkTable;

use crate::{
    ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename, race_from_unit_id,
};

pub type UnitDatabase = BTreeMap<Race, BTreeMap<UnitKind, BTreeMap<String, UnitDefinition>>>;

pub static UNITS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UnitsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UnitsExtraction::process,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitDefinition {
    build_time: u32,
    level: u32,
    gold_cost: u32,
}

impl UnitDefinition {
    pub fn new(build_time: u32, level: u32, gold_cost: u32) -> Self {
        Self {
            build_time,
            level,
            gold_cost,
        }
    }

    pub fn build_time(&self) -> u32 {
        self.build_time
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn gold_cost(&self) -> u32 {
        self.gold_cost
    }
}

fn kind_from_row(def_type: &str, isbldg: &str) -> UnitKind {
    if def_type == "hero" {
        UnitKind::Hero
    } else if isbldg == "1" {
        UnitKind::Building
    } else {
        UnitKind::Worker
    }
}

struct UnitsExtraction;

impl UnitsExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);

        path.starts_with("war3.w3mod:units") && filename.ends_with("unitbalance.slk")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing units with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;

        let table = SlkTable::from(text);

        let unit_database = match casc_filename(path).as_str() {
            "unitbalance.slk" => Self::process_data(table),
            _ => return Err(ExtractError::Units),
        };

        Ok(ExtractResult::Units(unit_database))
    }

    fn process_data(table: SlkTable) -> UnitDatabase {
        let mut unit_database = UnitDatabase::new();

        for row in table.into_iter() {
            let id = row.get("unitBalanceID").unwrap_or("").trim();
            let def_type = row.get("defType").unwrap_or("");
            let isbldg = row.get("isbldg").unwrap_or("");
            let build_time: u32 = row.get("bldtm").unwrap_or("0").trim().parse().unwrap_or(0);
            let level: u32 = row.get("level").unwrap_or("0").trim().parse().unwrap_or(0);
            let gold_cost: u32 = row
                .get("goldcost")
                .unwrap_or("0")
                .trim()
                .parse()
                .unwrap_or(0);

            if id.is_empty() {
                continue;
            }

            let Some(race) = race_from_unit_id(id) else {
                continue;
            };

            let kind = kind_from_row(def_type, isbldg);

            unit_database
                .entry(race)
                .or_default()
                .entry(kind)
                .or_default()
                .insert(
                    id.to_string(),
                    UnitDefinition::new(build_time, level, gold_cost),
                );
        }

        unit_database
    }
}
