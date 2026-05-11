use std::{collections::BTreeMap, path::PathBuf};

use warcraft_slk::SlkTable;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type UnitUiFlagsDatabase = BTreeMap<String, UnitUiFlagsEntry>;

pub static UNIT_UI_FLAGS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: UnitUiFlagsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: UnitUiFlagsExtraction::process,
};

#[derive(Debug, Clone, Default)]
pub struct UnitUiFlagsEntry {
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
}

impl UnitUiFlagsEntry {
    pub fn is_campaign(&self) -> bool {
        self.is_campaign
    }

    pub fn is_in_editor(&self) -> bool {
        self.is_in_editor
    }

    pub fn is_hidden_in_editor(&self) -> bool {
        self.is_hidden_in_editor
    }

    pub fn is_special(&self) -> bool {
        self.is_special
    }
}

struct UnitUiFlagsExtraction;

impl UnitUiFlagsExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path);
        path.starts_with("war3.w3mod:units") && filename.ends_with("unitui.slk")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let table = SlkTable::from(text);
        let database = Self::process_table(table);
        Ok(ExtractResult::UnitUiFlags(database))
    }

    fn process_table(table: SlkTable) -> UnitUiFlagsDatabase {
        let mut database = UnitUiFlagsDatabase::new();
        for row in table.into_iter() {
            let unit_id_raw = row.get("unitUIID").unwrap_or("").trim();
            if unit_id_raw.is_empty() {
                continue;
            }
            let entry = UnitUiFlagsEntry {
                is_campaign: Self::is_truthy(row.get("campaign")),
                is_in_editor: Self::is_truthy(row.get("inEditor")),
                is_hidden_in_editor: Self::is_truthy(row.get("hiddenInEditor")),
                is_special: Self::is_truthy(row.get("special")),
            };
            database.insert(unit_id_raw.to_string(), entry);
        }
        database
    }

    fn is_truthy(value: Option<&str>) -> bool {
        let Some(raw) = value else {
            return false;
        };
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed == "-" || trimmed == "_" {
            return false;
        }
        let lowered = trimmed.to_ascii_lowercase();
        matches!(lowered.as_str(), "1" | "true" | "yes")
    }
}
