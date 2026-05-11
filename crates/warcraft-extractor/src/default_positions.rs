use std::collections::BTreeMap;
use std::path::PathBuf;

use warcraft_api::{ColumnIndex, GridCoordinate, RowIndex};

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type DefaultPositionsDatabase = BTreeMap<String, DefaultPositionEntry>;

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultPositionEntry {
    button_position: Option<GridCoordinate>,
    research_button_position: Option<GridCoordinate>,
}

impl DefaultPositionEntry {
    pub fn new(
        button_position: Option<GridCoordinate>,
        research_button_position: Option<GridCoordinate>,
    ) -> Self {
        Self {
            button_position,
            research_button_position,
        }
    }

    pub fn button_position(&self) -> Option<GridCoordinate> {
        self.button_position
    }

    pub fn research_button_position(&self) -> Option<GridCoordinate> {
        self.research_button_position
    }

    pub fn merge(&mut self, other: DefaultPositionEntry) {
        if self.button_position.is_none() {
            self.button_position = other.button_position;
        }
        if self.research_button_position.is_none() {
            self.research_button_position = other.research_button_position;
        }
    }
}

pub static DEFAULT_POSITIONS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: DefaultPositionsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: DefaultPositionsExtraction::process,
};

struct DefaultPositionsExtraction;

impl DefaultPositionsExtraction {
    fn matches(path: &str) -> bool {
        if !path.starts_with("war3.w3mod:units") {
            return false;
        }
        let filename = casc_filename(path).to_ascii_lowercase();
        filename.ends_with("unitfunc.txt")
            || filename == "itemfunc.txt"
            || filename.ends_with("upgradefunc.txt")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let database = parse(text);
        Ok(ExtractResult::DefaultPositions(database))
    }
}

fn parse(text: &str) -> DefaultPositionsDatabase {
    let mut database: DefaultPositionsDatabase = DefaultPositionsDatabase::new();
    let mut current_id: Option<String> = None;
    let mut current_button: Option<GridCoordinate> = None;
    let mut current_research: Option<GridCoordinate> = None;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with(';') {
            continue;
        }
        if let Some(inner) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            flush(
                &mut database,
                &mut current_id,
                &mut current_button,
                &mut current_research,
            );
            current_id = Some(inner.to_string());
            current_button = None;
            current_research = None;
            continue;
        }
        let Some((key_raw, value_raw)) = line.split_once('=') else {
            continue;
        };
        let key_lower = key_raw.trim().to_ascii_lowercase();
        let value_trimmed = value_raw.trim();
        match key_lower.as_str() {
            "buttonpos" if current_button.is_none() => {
                current_button = parse_button_position(value_trimmed);
            }
            "researchbuttonpos" if current_research.is_none() => {
                current_research = parse_button_position(value_trimmed);
            }
            _ => {}
        }
    }
    flush(
        &mut database,
        &mut current_id,
        &mut current_button,
        &mut current_research,
    );
    database
}

fn flush(
    database: &mut DefaultPositionsDatabase,
    current_id: &mut Option<String>,
    current_button: &mut Option<GridCoordinate>,
    current_research: &mut Option<GridCoordinate>,
) {
    let Some(id) = current_id.take() else {
        current_button.take();
        current_research.take();
        return;
    };
    let button = current_button.take();
    let research = current_research.take();
    if button.is_none() && research.is_none() {
        return;
    }
    let entry = DefaultPositionEntry {
        button_position: button,
        research_button_position: research,
    };
    let existing = database.entry(id).or_default();
    existing.merge(entry);
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
