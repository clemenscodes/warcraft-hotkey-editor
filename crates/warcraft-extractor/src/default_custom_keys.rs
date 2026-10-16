use std::collections::BTreeMap;

use warcraft_api::{SystemKeybind, WarcraftDatabase, WarcraftObject, WarcraftObjectMeta};

/// The three hotkey fields a `CustomKeys.txt` section can carry, in the
/// order the editor serializes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HotkeyField {
    Hotkey,
    Unhotkey,
    Researchhotkey,
}

impl HotkeyField {
    fn from_key(key: &str) -> Option<Self> {
        let lowercase_key = key.trim().to_ascii_lowercase();
        match lowercase_key.as_str() {
            "hotkey" => Some(Self::Hotkey),
            "unhotkey" => Some(Self::Unhotkey),
            "researchhotkey" => Some(Self::Researchhotkey),
            _ => None,
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Self::Hotkey => "Hotkey",
            Self::Unhotkey => "Unhotkey",
            Self::Researchhotkey => "Researchhotkey",
        }
    }
}

/// The game's own default keybinds as a complete `CustomKeys.txt`: every
/// object of the game database with its default hotkeys and grid positions.
///
/// Hotkeys are layered from text sources in the order they are overlaid —
/// the localized `*strings.txt` defaults first, then Blizzard's shipped
/// `customkeys.txt`, whose values win (an empty value there unbinds). Within
/// one source the last section of a duplicated id wins, which is how
/// Blizzard's patch 3.0.0 talent block re-declares its base abilities.
/// Passive abilities keep their hotkey: the game shows it on the button even
/// though it cannot be pressed.
/// Grid positions come from the game database, the same values the editor
/// materializes, so the file and the editor never disagree on a default cell.
#[derive(Clone, Debug, Default)]
pub struct DefaultCustomKeys {
    hotkeys_by_section: BTreeMap<String, BTreeMap<HotkeyField, String>>,
}

impl DefaultCustomKeys {
    const HEADER: &'static str = "// Warcraft III default keybinds, generated from the game data by\n\
         // warcraft-extractor's `default_customkeys` example. Do not edit by hand.\n\n";

    /// Overlays the hotkey fields of one INI-style text source; later values
    /// replace earlier ones. Inline `//` comments are dropped.
    pub fn overlay(&mut self, text: &str) {
        let mut current_section: Option<String> = None;
        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            if let Some(section_id) = line
                .strip_prefix('[')
                .and_then(|inner| inner.strip_suffix(']'))
            {
                let lowercase_section = section_id.trim().to_ascii_lowercase();
                current_section = Some(lowercase_section);
                continue;
            }
            let Some(section) = current_section.as_ref() else {
                continue;
            };
            let Some((key, raw_value)) = line.split_once('=') else {
                continue;
            };
            let Some(field) = HotkeyField::from_key(key) else {
                continue;
            };
            let value_without_comment = raw_value
                .split_once("//")
                .map_or(raw_value, |parts| parts.0);
            let value = value_without_comment.trim().to_string();
            let section_key = section.clone();
            self.hotkeys_by_section
                .entry(section_key)
                .or_default()
                .insert(field, value);
        }
    }

    /// Renders one section per database object that has at least one default
    /// hotkey or grid position, ordered by object id, followed by the system
    /// keybinds (inventory, hero selection, control groups, menus, camera)
    /// with their default key, command class and modifier.
    pub fn render(&self, database: &WarcraftDatabase, system_keybinds: &[SystemKeybind]) -> String {
        let mut output = String::from(Self::HEADER);
        for (object_id, warcraft_object) in database.iter() {
            let object_value = object_id.value();
            let lines = self.section_lines(object_value, warcraft_object);
            if lines.is_empty() {
                continue;
            }
            output.push_str(&format!("[{object_value}]\n"));
            for line in lines {
                output.push_str(&line);
                output.push('\n');
            }
            output.push('\n');
        }
        for system_keybind in system_keybinds {
            let section_id = system_keybind.section_id();
            let default_hotkey = system_keybind.default_hotkey();
            let class_field = system_keybind.class().ini_field();
            output.push_str(&format!(
                "[{section_id}]\nHotkey={default_hotkey}\n{class_field}\n"
            ));
            let modifier = system_keybind.default_modifier();
            if let Some(modifier_text) = modifier.ini_str() {
                output.push_str(&format!("Modifier={modifier_text}\n"));
            }
            output.push('\n');
        }
        output
    }

    fn section_lines(&self, object_value: &str, warcraft_object: &WarcraftObject) -> Vec<String> {
        let lowercase_id = object_value.to_ascii_lowercase();
        let empty_hotkeys = BTreeMap::new();
        let hotkeys = self
            .hotkeys_by_section
            .get(&lowercase_id)
            .unwrap_or(&empty_hotkeys);
        let hotkey_line = |field: HotkeyField| {
            let value = hotkeys.get(&field)?;
            if value.is_empty() {
                return None;
            }
            let key = field.key();
            Some(format!("{key}={value}"))
        };
        let off_position = match warcraft_object.meta() {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.off_button_position(),
            _ => None,
        };
        let button_line = warcraft_object
            .default_button_position()
            .map(|position| format!("Buttonpos={position}"));
        let unbutton_line = off_position.map(|position| format!("Unbuttonpos={position}"));
        let research_button_line = warcraft_object
            .default_research_button_position()
            .map(|position| format!("Researchbuttonpos={position}"));
        [
            hotkey_line(HotkeyField::Hotkey),
            hotkey_line(HotkeyField::Unhotkey),
            button_line,
            unbutton_line,
            hotkey_line(HotkeyField::Researchhotkey),
            research_button_line,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}
