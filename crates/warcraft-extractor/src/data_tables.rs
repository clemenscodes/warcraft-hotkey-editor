use std::collections::BTreeMap;
use std::path::PathBuf;

use warcraft_slk::SlkTable;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type DataTablesDatabase = BTreeMap<String, BTreeMap<String, String>>;

const ID_COLUMN_CANDIDATES: &[&str] = &[
    "alias",
    "abilCode",
    "abilcode",
    "unitID",
    "unitid",
    "unitBalanceID",
    "unitbalanceid",
    "unitWeaponID",
    "unitweaponid",
    "itemID",
    "itemid",
    "upgradeid",
    "code",
    "buffID",
    "buffid",
];

pub static DATA_TABLES_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: DataTablesExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: DataTablesExtraction::process,
};

struct DataTablesExtraction;

impl DataTablesExtraction {
    fn matches(path: &str) -> bool {
        if !path.starts_with("war3.w3mod:units") {
            return false;
        }
        let filename = casc_filename(path);
        let lower_filename = filename.to_ascii_lowercase();
        if lower_filename.starts_with("notused_") {
            return false;
        }
        matches!(
            lower_filename.as_str(),
            "abilitydata.slk"
                | "unitbalance.slk"
                | "unitweapons.slk"
                | "unitdata.slk"
                | "unitabilities.slk"
                | "upgradedata.slk"
                | "itemdata.slk"
                | "abilitybuffdata.slk"
        )
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let table = SlkTable::from(text);

        let mut id_column_indices: Vec<warcraft_slk::ColumnKey> = Vec::new();
        for candidate in ID_COLUMN_CANDIDATES {
            if let Some(column_key) = table.column_key(candidate) {
                id_column_indices.push(column_key);
            }
        }

        let mut database: DataTablesDatabase = DataTablesDatabase::new();
        for row_view in &table {
            let mut row_id_option: Option<String> = None;
            for column_key in &id_column_indices {
                let Some(raw_value) = row_view.get_by_index(*column_key) else {
                    continue;
                };
                let trimmed_value = raw_value.trim();
                if trimmed_value.is_empty() {
                    continue;
                }
                row_id_option = Some(trimmed_value.to_string());
                break;
            }
            let Some(row_id) = row_id_option else {
                continue;
            };
            let entry = database.entry(row_id).or_default();
            for (column_key, column_name) in table.columns() {
                if column_name.is_empty() {
                    continue;
                }
                let Some(value) = row_view.get_by_index(*column_key) else {
                    continue;
                };
                let trimmed_value = value.trim();
                if trimmed_value.is_empty() {
                    continue;
                }
                entry.insert(column_name.clone(), trimmed_value.to_string());
            }
        }

        Ok(ExtractResult::DataTables(database))
    }
}
