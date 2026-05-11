use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, casc_filename};

pub type ObjectTextDatabase = BTreeMap<String, ObjectText>;

#[derive(Debug, Clone, Default)]
pub struct ObjectText {
    tip_levels: Vec<String>,
    ubertip_levels: Vec<String>,
    research_ubertip_levels: Vec<String>,
    un_tip: Option<String>,
    un_ubertip: Option<String>,
}

impl ObjectText {
    pub fn tip_levels(&self) -> &[String] {
        &self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &[String] {
        &self.ubertip_levels
    }

    pub fn research_ubertip_levels(&self) -> &[String] {
        &self.research_ubertip_levels
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip_levels.first().map(String::as_str)
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip_levels.first().map(String::as_str)
    }

    pub fn research_ubertip(&self) -> Option<&str> {
        self.research_ubertip_levels.first().map(String::as_str)
    }

    pub fn un_tip(&self) -> Option<&str> {
        self.un_tip.as_deref()
    }

    pub fn un_ubertip(&self) -> Option<&str> {
        self.un_ubertip.as_deref()
    }

    pub fn merge(&mut self, other: ObjectText) {
        if self.tip_levels.is_empty() {
            self.tip_levels = other.tip_levels;
        }
        if self.ubertip_levels.is_empty() {
            self.ubertip_levels = other.ubertip_levels;
        }
        if self.research_ubertip_levels.is_empty() {
            self.research_ubertip_levels = other.research_ubertip_levels;
        }
        if self.un_tip.is_none() {
            self.un_tip = other.un_tip;
        }
        if self.un_ubertip.is_none() {
            self.un_ubertip = other.un_ubertip;
        }
    }
}

pub static OBJECT_TEXTS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: ObjectTextsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: ObjectTextsExtraction::process,
};

struct ObjectTextsExtraction;

impl ObjectTextsExtraction {
    fn matches(path: &str) -> bool {
        let filename = casc_filename(path).to_ascii_lowercase();
        let is_strings_file = filename.ends_with("abilitystrings.txt")
            || filename.ends_with("unitstrings.txt")
            || filename == "itemstrings.txt"
            || filename.ends_with("upgradestrings.txt")
            || filename == "commandstrings.txt";
        if !is_strings_file {
            return false;
        }
        path.contains("enus.w3mod:units")
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let database = parse_object_text_file(text);
        Ok(ExtractResult::ObjectTexts(database))
    }
}

fn parse_object_text_file(text: &str) -> ObjectTextDatabase {
    let mut database: ObjectTextDatabase = ObjectTextDatabase::new();
    let mut current_id: Option<String> = None;
    let mut current_tips: Vec<String> = Vec::new();
    let mut current_ubertips: Vec<String> = Vec::new();
    let mut current_research_ubertips: Vec<String> = Vec::new();
    let mut current_un_tip: Option<String> = None;
    let mut current_un_ubertip: Option<String> = None;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(inner) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            flush_object_text(
                &mut database,
                &mut current_id,
                &mut current_tips,
                &mut current_ubertips,
                &mut current_research_ubertips,
                &mut current_un_tip,
                &mut current_un_ubertip,
            );
            current_id = Some(inner.to_string());
            continue;
        }
        let Some((key_raw, value_raw)) = line.split_once('=') else {
            continue;
        };
        let key_lower = key_raw.trim().to_ascii_lowercase();
        let value_trimmed = value_raw.trim();
        match key_lower.as_str() {
            "tip" if current_tips.is_empty() => {
                current_tips = parse_quoted_value_list(value_trimmed);
            }
            "ubertip" if current_ubertips.is_empty() => {
                current_ubertips = parse_quoted_value_list(value_trimmed);
            }
            "researchubertip" if current_research_ubertips.is_empty() => {
                current_research_ubertips = parse_quoted_value_list(value_trimmed);
            }
            "untip" if current_un_tip.is_none() => {
                let first = parse_quoted_value_list(value_trimmed)
                    .into_iter()
                    .next()
                    .unwrap_or_default();
                if !first.is_empty() {
                    current_un_tip = Some(first);
                }
            }
            "unubertip" if current_un_ubertip.is_none() => {
                let first = parse_quoted_value_list(value_trimmed)
                    .into_iter()
                    .next()
                    .unwrap_or_default();
                if !first.is_empty() {
                    current_un_ubertip = Some(first);
                }
            }
            _ => {}
        }
    }
    flush_object_text(
        &mut database,
        &mut current_id,
        &mut current_tips,
        &mut current_ubertips,
        &mut current_research_ubertips,
        &mut current_un_tip,
        &mut current_un_ubertip,
    );
    database
}

fn flush_object_text(
    database: &mut ObjectTextDatabase,
    current_id: &mut Option<String>,
    current_tips: &mut Vec<String>,
    current_ubertips: &mut Vec<String>,
    current_research_ubertips: &mut Vec<String>,
    current_un_tip: &mut Option<String>,
    current_un_ubertip: &mut Option<String>,
) {
    let Some(id) = current_id.take() else {
        current_tips.clear();
        current_ubertips.clear();
        current_research_ubertips.clear();
        current_un_tip.take();
        current_un_ubertip.take();
        return;
    };
    let tip_levels = std::mem::take(current_tips);
    let ubertip_levels = std::mem::take(current_ubertips);
    let research_ubertip_levels = std::mem::take(current_research_ubertips);
    let un_tip = current_un_tip.take();
    let un_ubertip = current_un_ubertip.take();
    if tip_levels.is_empty()
        && ubertip_levels.is_empty()
        && research_ubertip_levels.is_empty()
        && un_tip.is_none()
        && un_ubertip.is_none()
    {
        return;
    }
    let entry = ObjectText {
        tip_levels,
        ubertip_levels,
        research_ubertip_levels,
        un_tip,
        un_ubertip,
    };
    database.entry(id).or_insert(entry);
}

fn parse_quoted_value_list(input: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for character in input.chars() {
        match character {
            '"' => {
                in_quotes = !in_quotes;
            }
            ',' if !in_quotes => {
                let trimmed = current.trim();
                if !trimmed.is_empty() {
                    output.push(trimmed.to_string());
                }
                current.clear();
            }
            _ => current.push(character),
        }
    }
    let trimmed = current.trim();
    if !trimmed.is_empty() {
        output.push(trimmed.to_string());
    }
    output
}
