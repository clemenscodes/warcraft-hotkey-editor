use std::path::PathBuf;

use warcraft_api::{SystemKeybindClass, SystemKeybindModifier};

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule};

pub type SystemKeybindsDatabase = Vec<SystemKeybindEntry>;

#[derive(Debug, Clone)]
pub struct SystemKeybindEntry {
    section_id: String,
    comment: String,
    default_hotkey: u32,
    default_modifier: SystemKeybindModifier,
    class: SystemKeybindClass,
}

impl SystemKeybindEntry {
    pub fn section_id(&self) -> &str {
        &self.section_id
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn default_hotkey(&self) -> u32 {
        self.default_hotkey
    }

    pub fn default_modifier(&self) -> SystemKeybindModifier {
        self.default_modifier
    }

    pub fn class(&self) -> SystemKeybindClass {
        self.class
    }
}

pub static SYSTEM_KEYBINDS_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: SystemKeybindsExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: SystemKeybindsExtraction::process,
};

struct SystemKeybindsExtraction;

impl SystemKeybindsExtraction {
    fn matches(path: &str) -> bool {
        let lowered = path.to_ascii_lowercase();
        lowered == "war3.w3mod:customkeys.txt"
    }

    fn process(_: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;
        let database = parse_template(text);
        Ok(ExtractResult::SystemKeybinds(database))
    }
}

fn parse_template(text: &str) -> SystemKeybindsDatabase {
    let mut database: SystemKeybindsDatabase = Vec::new();
    let mut last_comment: Option<String> = None;
    let mut current_section_id: Option<String> = None;
    let mut current_comment: String = String::new();
    let mut current_hotkey: Option<u32> = None;
    let mut current_modifier: SystemKeybindModifier = SystemKeybindModifier::None;
    let mut current_class: Option<SystemKeybindClass> = None;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(comment_body) = line.strip_prefix("//") {
            let trimmed_body = comment_body.trim();
            if !trimmed_body.is_empty() && !trimmed_body.starts_with('/') {
                last_comment = Some(trimmed_body.to_string());
            }
            continue;
        }
        if let Some(inner) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            flush_entry(
                &mut database,
                &mut current_section_id,
                &mut current_comment,
                &mut current_hotkey,
                &mut current_modifier,
                &mut current_class,
            );
            current_section_id = Some(inner.to_string());
            current_comment = last_comment.take().unwrap_or_default();
            current_modifier = SystemKeybindModifier::None;
            current_class = None;
            current_hotkey = None;
            continue;
        }
        let Some((key_raw, value_raw)) = line.split_once('=') else {
            continue;
        };
        let key_lower = key_raw.trim().to_ascii_lowercase();
        let value_trimmed = value_raw.trim();
        match key_lower.as_str() {
            "hotkey" => {
                if let Ok(parsed) = value_trimmed.parse::<u32>() {
                    current_hotkey = Some(parsed);
                }
            }
            "modifier" => {
                current_modifier = parse_modifier(value_trimmed);
            }
            "menucommand" => current_class = Some(SystemKeybindClass::Menu),
            "ctrlgroupcommand" => current_class = Some(SystemKeybindClass::ControlGroup),
            "gamecommand" => current_class = Some(SystemKeybindClass::Game),
            "cameracommand" => current_class = Some(SystemKeybindClass::Camera),
            "observercommand" => current_class = Some(SystemKeybindClass::Observer),
            "replaycommand" => current_class = Some(SystemKeybindClass::Replay),
            _ => {}
        }
    }
    flush_entry(
        &mut database,
        &mut current_section_id,
        &mut current_comment,
        &mut current_hotkey,
        &mut current_modifier,
        &mut current_class,
    );
    database
}

fn flush_entry(
    database: &mut SystemKeybindsDatabase,
    current_section_id: &mut Option<String>,
    current_comment: &mut String,
    current_hotkey: &mut Option<u32>,
    current_modifier: &mut SystemKeybindModifier,
    current_class: &mut Option<SystemKeybindClass>,
) {
    let Some(section_id) = current_section_id.take() else {
        return;
    };
    let Some(class) = current_class.take() else {
        return;
    };
    let Some(default_hotkey) = current_hotkey.take() else {
        return;
    };
    let comment = std::mem::take(current_comment);
    let modifier = std::mem::replace(current_modifier, SystemKeybindModifier::None);
    let entry = SystemKeybindEntry {
        section_id,
        comment,
        default_hotkey,
        default_modifier: modifier,
        class,
    };
    database.push(entry);
}

fn parse_modifier(value: &str) -> SystemKeybindModifier {
    match value.to_ascii_lowercase().as_str() {
        "alt" => SystemKeybindModifier::Alt,
        "ctrl" => SystemKeybindModifier::Ctrl,
        "ctrl_or_alt" => SystemKeybindModifier::CtrlOrAlt,
        "shift" => SystemKeybindModifier::Shift,
        _ => SystemKeybindModifier::None,
    }
}
