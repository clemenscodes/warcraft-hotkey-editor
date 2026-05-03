use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonPosition {
    column: u8,
    row: u8,
}

impl ButtonPosition {
    pub const fn new(column: u8, row: u8) -> Self {
        Self { column, row }
    }

    pub fn column(&self) -> u8 {
        self.column
    }

    pub fn row(&self) -> u8 {
        self.row
    }
}

impl TryFrom<&str> for ButtonPosition {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut parts = text.splitn(2, ',');
        let column_str = parts.next().ok_or(())?;
        let row_str = parts.next().ok_or(())?;
        let column = column_str.trim().parse::<u8>().map_err(|_| ())?;
        let row = row_str.trim().parse::<u8>().map_err(|_| ())?;
        let button_position = ButtonPosition { column, row };
        Ok(button_position)
    }
}

#[derive(Default, Debug, Clone)]
pub struct AbilityBinding {
    hotkey: Option<String>,
    unhotkey: Option<String>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    research_hotkey: Option<String>,
    research_button_position: Option<ButtonPosition>,
    tip: Option<String>,
    research_tip: Option<String>,
    un_tip: Option<String>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    un_ubertip: Option<String>,
    icon: Option<String>,
    un_icon: Option<String>,
    modifier: Option<String>,
    dirty: bool,
}

impl AbilityBinding {
    pub fn hotkey(&self) -> Option<&str> {
        self.hotkey.as_deref()
    }

    pub fn unhotkey(&self) -> Option<&str> {
        self.unhotkey.as_deref()
    }

    pub fn button_position(&self) -> Option<&ButtonPosition> {
        self.button_position.as_ref()
    }

    pub fn unbutton_position(&self) -> Option<&ButtonPosition> {
        self.unbutton_position.as_ref()
    }

    pub fn research_hotkey(&self) -> Option<&str> {
        self.research_hotkey.as_deref()
    }

    pub fn research_button_position(&self) -> Option<&ButtonPosition> {
        self.research_button_position.as_ref()
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn research_tip(&self) -> Option<&str> {
        self.research_tip.as_deref()
    }

    pub fn un_tip(&self) -> Option<&str> {
        self.un_tip.as_deref()
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }

    pub fn research_ubertip(&self) -> Option<&str> {
        self.research_ubertip.as_deref()
    }

    pub fn un_ubertip(&self) -> Option<&str> {
        self.un_ubertip.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub fn un_icon(&self) -> Option<&str> {
        self.un_icon.as_deref()
    }

    pub fn modifier(&self) -> Option<&str> {
        self.modifier.as_deref()
    }

    pub fn set_modifier(&mut self, value: Option<String>) {
        if self.modifier != value {
            self.modifier = value;
            self.dirty = true;
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn set_hotkey(&mut self, value: Option<String>) {
        if self.hotkey != value {
            self.hotkey = value;
            self.dirty = true;
        }
    }

    pub fn set_button_position(&mut self, value: Option<ButtonPosition>) {
        if self.button_position != value {
            self.button_position = value;
            self.dirty = true;
        }
    }

    pub fn set_unbutton_position(&mut self, value: Option<ButtonPosition>) {
        if self.unbutton_position != value {
            self.unbutton_position = value;
            self.dirty = true;
        }
    }

    pub fn set_unhotkey(&mut self, value: Option<String>) {
        if self.unhotkey != value {
            self.unhotkey = value;
            self.dirty = true;
        }
    }

    pub fn set_research_hotkey(&mut self, value: Option<String>) {
        if self.research_hotkey != value {
            self.research_hotkey = value;
            self.dirty = true;
        }
    }

    pub fn set_research_button_position(&mut self, value: Option<ButtonPosition>) {
        if self.research_button_position != value {
            self.research_button_position = value;
            self.dirty = true;
        }
    }

    pub fn set_tip(&mut self, value: Option<String>) {
        if self.tip != value {
            self.tip = value;
            self.dirty = true;
        }
    }

    pub fn set_research_tip(&mut self, value: Option<String>) {
        if self.research_tip != value {
            self.research_tip = value;
            self.dirty = true;
        }
    }

    pub fn set_un_tip(&mut self, value: Option<String>) {
        if self.un_tip != value {
            self.un_tip = value;
            self.dirty = true;
        }
    }

    pub fn set_ubertip(&mut self, value: Option<String>) {
        if self.ubertip != value {
            self.ubertip = value;
            self.dirty = true;
        }
    }

    pub fn set_research_ubertip(&mut self, value: Option<String>) {
        if self.research_ubertip != value {
            self.research_ubertip = value;
            self.dirty = true;
        }
    }

    pub fn set_un_ubertip(&mut self, value: Option<String>) {
        if self.un_ubertip != value {
            self.un_ubertip = value;
            self.dirty = true;
        }
    }

    pub fn set_un_icon(&mut self, value: Option<String>) {
        if self.un_icon != value {
            self.un_icon = value;
            self.dirty = true;
        }
    }

    pub fn set_icon(&mut self, value: Option<String>) {
        if self.icon != value {
            self.icon = value;
            self.dirty = true;
        }
    }
}

#[derive(Default)]
struct AbilityBindingAccumulator {
    hotkey: Option<String>,
    unhotkey: Option<String>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    research_hotkey: Option<String>,
    research_button_position: Option<ButtonPosition>,
    tip: Option<String>,
    research_tip: Option<String>,
    un_tip: Option<String>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    un_ubertip: Option<String>,
    icon: Option<String>,
    un_icon: Option<String>,
    modifier: Option<String>,
}

impl AbilityBindingAccumulator {
    fn apply(&mut self, key: &str, value: &str) {
        let lowercase_key = key.to_lowercase();
        match lowercase_key.as_str() {
            "hotkey" if !value.is_empty() && self.hotkey.is_none() => {
                let hotkey = value.to_string();
                self.hotkey = Some(hotkey);
            }
            "buttonpos" if self.button_position.is_none() => {
                self.button_position = ButtonPosition::try_from(value).ok();
            }
            "unbuttonpos" if self.unbutton_position.is_none() => {
                self.unbutton_position = ButtonPosition::try_from(value).ok();
            }
            "unhotkey" if !value.is_empty() && self.unhotkey.is_none() => {
                let unhotkey = value.to_string();
                self.unhotkey = Some(unhotkey);
            }
            "researchhotkey" if !value.is_empty() && self.research_hotkey.is_none() => {
                let research_hotkey = value.to_string();
                self.research_hotkey = Some(research_hotkey);
            }
            "researchbuttonpos" if self.research_button_position.is_none() => {
                self.research_button_position = ButtonPosition::try_from(value).ok();
            }
            "tip" if self.tip.is_none() => {
                self.tip = Some(value.to_string());
            }
            "researchtip" if self.research_tip.is_none() => {
                self.research_tip = Some(value.to_string());
            }
            "untip" if self.un_tip.is_none() => {
                self.un_tip = Some(value.to_string());
            }
            "ubertip" if self.ubertip.is_none() => {
                self.ubertip = Some(value.to_string());
            }
            "researchubertip" if self.research_ubertip.is_none() => {
                self.research_ubertip = Some(value.to_string());
            }
            "unubertip" if self.un_ubertip.is_none() => {
                self.un_ubertip = Some(value.to_string());
            }
            "icon" if !value.is_empty() && self.icon.is_none() => {
                let icon = value.to_string();
                self.icon = Some(icon);
            }
            "art" if !value.is_empty() && self.icon.is_none() => {
                let art = value.to_string();
                self.icon = Some(art);
            }
            "unart" if !value.is_empty() && self.un_icon.is_none() => {
                let un_art = value.to_string();
                self.un_icon = Some(un_art);
            }
            "modifier" if !value.is_empty() && self.modifier.is_none() => {
                let modifier = value.to_string();
                self.modifier = Some(modifier);
            }
            _ => {}
        }
    }

    fn into_binding(self) -> AbilityBinding {
        AbilityBinding {
            hotkey: self.hotkey,
            unhotkey: self.unhotkey,
            button_position: self.button_position,
            unbutton_position: self.unbutton_position,
            research_hotkey: self.research_hotkey,
            research_button_position: self.research_button_position,
            tip: self.tip,
            research_tip: self.research_tip,
            un_tip: self.un_tip,
            ubertip: self.ubertip,
            research_ubertip: self.research_ubertip,
            un_ubertip: self.un_ubertip,
            icon: self.icon,
            un_icon: self.un_icon,
            modifier: self.modifier,
            dirty: false,
        }
    }
}

pub struct BindingEntry<'a> {
    id: &'a str,
    binding: &'a AbilityBinding,
}

impl<'a> BindingEntry<'a> {
    pub fn id(&self) -> &'a str {
        self.id
    }

    pub fn binding(&self) -> &'a AbilityBinding {
        self.binding
    }
}

#[derive(Default, Debug, Clone)]
pub struct CommandBinding {
    hotkey: Option<String>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    tip: Option<String>,
    un_tip: Option<String>,
    dirty: bool,
}

impl CommandBinding {
    pub fn hotkey(&self) -> Option<&str> {
        self.hotkey.as_deref()
    }

    pub fn button_position(&self) -> Option<&ButtonPosition> {
        self.button_position.as_ref()
    }

    pub fn unbutton_position(&self) -> Option<&ButtonPosition> {
        self.unbutton_position.as_ref()
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn un_tip(&self) -> Option<&str> {
        self.un_tip.as_deref()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn set_hotkey(&mut self, value: Option<String>) {
        if self.hotkey != value {
            self.hotkey = value;
            self.dirty = true;
        }
    }

    pub fn set_button_position(&mut self, value: Option<ButtonPosition>) {
        if self.button_position != value {
            self.button_position = value;
            self.dirty = true;
        }
    }

    pub fn set_unbutton_position(&mut self, value: Option<ButtonPosition>) {
        if self.unbutton_position != value {
            self.unbutton_position = value;
            self.dirty = true;
        }
    }

    pub fn set_tip(&mut self, value: Option<String>) {
        if self.tip != value {
            self.tip = value;
            self.dirty = true;
        }
    }

    pub fn set_un_tip(&mut self, value: Option<String>) {
        if self.un_tip != value {
            self.un_tip = value;
            self.dirty = true;
        }
    }
}

#[derive(Default)]
struct CommandBindingAccumulator {
    hotkey: Option<String>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    tip: Option<String>,
    un_tip: Option<String>,
}

impl CommandBindingAccumulator {
    fn apply(&mut self, key: &str, value: &str) {
        let lowercase_key = key.to_lowercase();
        match lowercase_key.as_str() {
            "hotkey" if !value.is_empty() && self.hotkey.is_none() => {
                let hotkey = value.to_string();
                self.hotkey = Some(hotkey);
            }
            "buttonpos" if self.button_position.is_none() => {
                self.button_position = ButtonPosition::try_from(value).ok();
            }
            "unbuttonpos" if self.unbutton_position.is_none() => {
                self.unbutton_position = ButtonPosition::try_from(value).ok();
            }
            "tip" if self.tip.is_none() => {
                self.tip = Some(value.to_string());
            }
            "untip" if self.un_tip.is_none() => {
                self.un_tip = Some(value.to_string());
            }
            _ => {}
        }
    }

    fn into_binding(self) -> CommandBinding {
        CommandBinding {
            hotkey: self.hotkey,
            button_position: self.button_position,
            unbutton_position: self.unbutton_position,
            tip: self.tip,
            un_tip: self.un_tip,
            dirty: false,
        }
    }
}

pub struct CommandEntry<'a> {
    name: &'a str,
    binding: &'a CommandBinding,
}

impl<'a> CommandEntry<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn binding(&self) -> &'a CommandBinding {
        self.binding
    }
}

fn is_command_section(section_name: &str) -> bool {
    let lowered = section_name.to_ascii_lowercase();
    lowered.starts_with("cmd")
}

pub struct CustomKeysFile {
    /// Keyed by lowercase object ID for case-insensitive lookup.
    bindings: HashMap<String, AbilityBinding>,
    /// Lowercase object IDs in file order, for ordered iteration.
    order: Vec<String>,
    /// Original-case IDs indexed by their lowercase form, for round-trip serialisation.
    original_ids: HashMap<String, String>,
    /// Built-in command sections (e.g. CommandMove, CommandStop), keyed by lowercase name.
    commands: HashMap<String, CommandBinding>,
    /// Lowercase command names in file order, for ordered iteration.
    command_order: Vec<String>,
    /// Original-case command names indexed by lowercase form.
    original_command_names: HashMap<String, String>,
    /// Verbatim text of each ability section (`[id]` line through trailing blank line) for byte-identical
    /// preservation of untouched bindings during patch-mode export.
    raw_sections: HashMap<String, String>,
    /// Verbatim text of each command section.
    raw_command_sections: HashMap<String, String>,
}

impl CustomKeysFile {
    pub fn binding(&self, object_id: &str) -> Option<&AbilityBinding> {
        let lowercase_id = object_id.to_lowercase();
        self.bindings.get(&lowercase_id)
    }

    pub fn binding_mut(&mut self, object_id: &str) -> Option<&mut AbilityBinding> {
        let lowercase_id = object_id.to_lowercase();
        self.bindings.get_mut(&lowercase_id)
    }

    pub fn binding_or_default_mut(&mut self, object_id: &str) -> &mut AbilityBinding {
        let lowercase_id = object_id.to_lowercase();
        if !self.bindings.contains_key(&lowercase_id) {
            let original_id_string = object_id.to_string();
            self.order.push(lowercase_id.clone());
            self.original_ids.insert(lowercase_id.clone(), original_id_string);
            self.bindings.insert(lowercase_id.clone(), AbilityBinding::default());
        }
        self.bindings.get_mut(&lowercase_id).expect("binding was just inserted")
    }

    pub fn bindings_in_order(&self) -> impl Iterator<Item = BindingEntry<'_>> {
        self.order.iter().filter_map(|lowercase_id| {
            let original_id = self
                .original_ids
                .get(lowercase_id)
                .map(String::as_str)
                .unwrap_or(lowercase_id);
            self.bindings.get(lowercase_id).map(|binding| BindingEntry {
                id: original_id,
                binding,
            })
        })
    }

    pub fn command(&self, name: &str) -> Option<&CommandBinding> {
        let lowercase_name = name.to_lowercase();
        self.commands.get(&lowercase_name)
    }

    pub fn command_mut(&mut self, name: &str) -> Option<&mut CommandBinding> {
        let lowercase_name = name.to_lowercase();
        self.commands.get_mut(&lowercase_name)
    }

    pub fn command_or_default_mut(&mut self, name: &str) -> &mut CommandBinding {
        let lowercase_name = name.to_lowercase();
        if !self.commands.contains_key(&lowercase_name) {
            let original_name_string = name.to_string();
            self.command_order.push(lowercase_name.clone());
            self.original_command_names
                .insert(lowercase_name.clone(), original_name_string);
            self.commands.insert(lowercase_name.clone(), CommandBinding::default());
        }
        self.commands
            .get_mut(&lowercase_name)
            .expect("command was just inserted")
    }

    pub fn commands_in_order(&self) -> impl Iterator<Item = CommandEntry<'_>> {
        self.command_order.iter().filter_map(|lowercase_name| {
            let original_name = self
                .original_command_names
                .get(lowercase_name)
                .map(String::as_str)
                .unwrap_or(lowercase_name);
            self.commands.get(lowercase_name).map(|binding| CommandEntry {
                name: original_name,
                binding,
            })
        })
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let content = text.as_str();
        let parsed = Self::from(content);
        Ok(parsed)
    }

    pub fn default_path() -> Option<PathBuf> {
        let home = home_directory()?;
        let native_path = home
            .join("Documents")
            .join("Warcraft III")
            .join("CustomKeyBindings")
            .join("CustomKeys.txt");
        if native_path.exists() {
            return Some(native_path);
        }
        #[cfg(target_os = "linux")]
        {
            let wine_prefix = std::env::var("WINEPREFIX")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home.join(".wine"));
            if let Ok(user) = std::env::var("USER") {
                let wine_path = wine_prefix
                    .join("drive_c")
                    .join("users")
                    .join(user)
                    .join("Documents")
                    .join("Warcraft III")
                    .join("CustomKeyBindings")
                    .join("CustomKeys.txt");
                if wine_path.exists() {
                    return Some(wine_path);
                }
            }
        }
        None
    }

    pub fn load_default() -> io::Result<Self> {
        let path = Self::default_path().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "CustomKeys.txt not found in ~/Documents/Warcraft III/CustomKeyBindings/ or Wine prefix",
            )
        })?;
        Self::load(path)
    }

    pub fn to_file_content(&self) -> String {
        let mut output = String::new();
        for lowercase_name in &self.command_order {
            let display_name = self
                .original_command_names
                .get(lowercase_name)
                .map(String::as_str)
                .unwrap_or(lowercase_name);
            let Some(binding) = self.commands.get(lowercase_name) else {
                continue;
            };
            if !binding.is_dirty()
                && let Some(raw_text) = self.raw_command_sections.get(lowercase_name)
            {
                output.push_str(raw_text);
                if !raw_text.ends_with("\n\n") {
                    output.push('\n');
                }
                continue;
            }
            Self::format_command_section(&mut output, display_name, binding);
        }
        for lowercase_id in &self.order {
            let display_id = self
                .original_ids
                .get(lowercase_id)
                .map(String::as_str)
                .unwrap_or(lowercase_id);
            let Some(binding) = self.bindings.get(lowercase_id) else {
                continue;
            };
            if !binding.is_dirty()
                && let Some(raw_text) = self.raw_sections.get(lowercase_id)
            {
                output.push_str(raw_text);
                if !raw_text.ends_with("\n\n") {
                    output.push('\n');
                }
                continue;
            }
            Self::format_ability_section(&mut output, display_id, binding);
        }
        output
    }

    fn format_command_section(output: &mut String, display_name: &str, binding: &CommandBinding) {
        output.push('[');
        output.push_str(display_name);
        output.push_str("]\n");
        if let Some(hotkey) = &binding.hotkey {
            output.push_str("Hotkey=");
            output.push_str(hotkey);
            output.push('\n');
        }
        if let Some(button_position) = &binding.button_position {
            output.push_str("Buttonpos=");
            let column_string = button_position.column.to_string();
            output.push_str(&column_string);
            output.push(',');
            let row_string = button_position.row.to_string();
            output.push_str(&row_string);
            output.push('\n');
        }
        if let Some(unbutton_position) = &binding.unbutton_position {
            output.push_str("Unbuttonpos=");
            let column_string = unbutton_position.column.to_string();
            output.push_str(&column_string);
            output.push(',');
            let row_string = unbutton_position.row.to_string();
            output.push_str(&row_string);
            output.push('\n');
        }
        if let Some(tip) = &binding.tip {
            output.push_str("Tip=");
            output.push_str(tip);
            output.push('\n');
        }
        if let Some(un_tip) = &binding.un_tip {
            output.push_str("UnTip=");
            output.push_str(un_tip);
            output.push('\n');
        }
        output.push('\n');
    }

    fn format_ability_section(output: &mut String, display_id: &str, binding: &AbilityBinding) {
        output.push('[');
        output.push_str(display_id);
        output.push_str("]\n");
        if let Some(hotkey) = &binding.hotkey {
            output.push_str("Hotkey=");
            output.push_str(hotkey);
            output.push('\n');
        }
        if let Some(unhotkey) = &binding.unhotkey {
            output.push_str("Unhotkey=");
            output.push_str(unhotkey);
            output.push('\n');
        }
        if let Some(button_position) = &binding.button_position {
            output.push_str("Buttonpos=");
            let column_string = button_position.column.to_string();
            output.push_str(&column_string);
            output.push(',');
            let row_string = button_position.row.to_string();
            output.push_str(&row_string);
            output.push('\n');
        }
        if let Some(unbutton_position) = &binding.unbutton_position {
            output.push_str("Unbuttonpos=");
            let column_string = unbutton_position.column.to_string();
            output.push_str(&column_string);
            output.push(',');
            let row_string = unbutton_position.row.to_string();
            output.push_str(&row_string);
            output.push('\n');
        }
        if let Some(hotkey) = &binding.research_hotkey {
            output.push_str("Researchhotkey=");
            output.push_str(hotkey);
            output.push('\n');
        }
        if let Some(research_button_position) = &binding.research_button_position {
            output.push_str("Researchbuttonpos=");
            let column_string = research_button_position.column.to_string();
            output.push_str(&column_string);
            output.push(',');
            let row_string = research_button_position.row.to_string();
            output.push_str(&row_string);
            output.push('\n');
        }
        if let Some(tip) = &binding.tip {
            output.push_str("Tip=");
            output.push_str(tip);
            output.push('\n');
        }
        if let Some(research_tip) = &binding.research_tip {
            output.push_str("Researchtip=");
            output.push_str(research_tip);
            output.push('\n');
        }
        if let Some(un_tip) = &binding.un_tip {
            output.push_str("UnTip=");
            output.push_str(un_tip);
            output.push('\n');
        }
        if let Some(icon) = &binding.icon {
            output.push_str("Icon=");
            output.push_str(icon);
            output.push('\n');
        }
        if let Some(modifier) = &binding.modifier {
            output.push_str("Modifier=");
            output.push_str(modifier);
            output.push('\n');
        }
        output.push('\n');
    }

    fn parse_section_header(line: &str) -> Option<String> {
        let without_brackets = line.strip_prefix('[')?.strip_suffix(']')?;
        let id = without_brackets.trim();
        if id.is_empty() {
            None
        } else {
            let id_string = id.to_string();
            Some(id_string)
        }
    }

    fn parse_key_value(line: &str) -> Option<KeyValuePair> {
        let (key_str, value_str) = line.split_once('=')?;

        let key = key_str.trim().to_string();
        let value = value_str.to_string();
        let pair = KeyValuePair { key, value };
        Some(pair)
    }
}

enum SectionAccumulator {
    Ability(Box<AbilityBindingAccumulator>),
    Command(CommandBindingAccumulator),
}

impl From<&str> for CustomKeysFile {
    fn from(text: &str) -> Self {
        let mut bindings: HashMap<String, AbilityBinding> = HashMap::new();
        let mut order: Vec<String> = Vec::new();
        let mut original_ids: HashMap<String, String> = HashMap::new();
        let mut commands: HashMap<String, CommandBinding> = HashMap::new();
        let mut command_order: Vec<String> = Vec::new();
        let mut original_command_names: HashMap<String, String> = HashMap::new();
        let mut raw_sections: HashMap<String, String> = HashMap::new();
        let mut raw_command_sections: HashMap<String, String> = HashMap::new();
        let mut current_lowercase_key: Option<String> = None;
        let mut current_raw_text: String = String::new();
        let mut current_is_command: bool = false;
        let mut accumulator: Option<SectionAccumulator> = None;

        for line in text.lines() {
            let trimmed = line.trim();
            let is_blank = trimmed.is_empty();
            let is_comment = trimmed.starts_with("//") || trimmed.starts_with(';');
            let header_id = if is_blank || is_comment {
                None
            } else {
                Self::parse_section_header(trimmed)
            };

            if let Some(original_id) = header_id {
                let stores = SectionStores {
                    bindings: &mut bindings,
                    commands: &mut commands,
                    raw_sections: &mut raw_sections,
                    raw_command_sections: &mut raw_command_sections,
                };
                let pending = PendingSection {
                    lowercase_key: current_lowercase_key.take(),
                    accumulator: accumulator.take(),
                    raw_text: std::mem::take(&mut current_raw_text),
                    is_command: current_is_command,
                };
                Self::flush_section(stores, pending);
                let lowercase_id = original_id.to_lowercase();
                if is_command_section(&original_id) {
                    if commands.contains_key(&lowercase_id) {
                        current_lowercase_key = None;
                        accumulator = None;
                        current_is_command = false;
                    } else {
                        let lookup_key = lowercase_id.clone();
                        original_command_names
                            .entry(lookup_key)
                            .or_insert_with(|| original_id.clone());
                        let order_entry = lowercase_id.clone();
                        command_order.push(order_entry);
                        current_lowercase_key = Some(lowercase_id);
                        current_is_command = true;
                        accumulator = Some(SectionAccumulator::Command(CommandBindingAccumulator::default()));
                        current_raw_text.push_str(line);
                        current_raw_text.push('\n');
                    }
                } else if bindings.contains_key(&lowercase_id) {
                    current_lowercase_key = None;
                    accumulator = None;
                    current_is_command = false;
                } else {
                    let original_ids_key = lowercase_id.clone();
                    original_ids
                        .entry(original_ids_key)
                        .or_insert_with(|| original_id.clone());
                    let order_entry = lowercase_id.clone();
                    order.push(order_entry);
                    current_lowercase_key = Some(lowercase_id);
                    current_is_command = false;
                    accumulator = Some(SectionAccumulator::Ability(Box::default()));
                    current_raw_text.push_str(line);
                    current_raw_text.push('\n');
                }
            } else {
                if accumulator.is_some() {
                    current_raw_text.push_str(line);
                    current_raw_text.push('\n');
                }
                if !is_blank
                    && !is_comment
                    && let Some(pair) = Self::parse_key_value(trimmed)
                    && let Some(current_accumulator) = accumulator.as_mut()
                {
                    let KeyValuePair { key, value } = pair;
                    match current_accumulator {
                        SectionAccumulator::Ability(ability_accumulator) => ability_accumulator.apply(&key, &value),
                        SectionAccumulator::Command(command_accumulator) => command_accumulator.apply(&key, &value),
                    }
                }
            }
        }

        let final_stores = SectionStores {
            bindings: &mut bindings,
            commands: &mut commands,
            raw_sections: &mut raw_sections,
            raw_command_sections: &mut raw_command_sections,
        };
        let final_pending = PendingSection {
            lowercase_key: current_lowercase_key,
            accumulator,
            raw_text: current_raw_text,
            is_command: current_is_command,
        };
        Self::flush_section(final_stores, final_pending);

        CustomKeysFile {
            bindings,
            order,
            original_ids,
            commands,
            command_order,
            original_command_names,
            raw_sections,
            raw_command_sections,
        }
    }
}

struct SectionStores<'a> {
    bindings: &'a mut HashMap<String, AbilityBinding>,
    commands: &'a mut HashMap<String, CommandBinding>,
    raw_sections: &'a mut HashMap<String, String>,
    raw_command_sections: &'a mut HashMap<String, String>,
}

struct PendingSection {
    lowercase_key: Option<String>,
    accumulator: Option<SectionAccumulator>,
    raw_text: String,
    is_command: bool,
}

impl CustomKeysFile {
    fn flush_section(stores: SectionStores<'_>, pending: PendingSection) {
        let Some(lowercase_key) = pending.lowercase_key else {
            return;
        };
        let Some(finished_accumulator) = pending.accumulator else {
            return;
        };
        let raw_text = pending.raw_text;
        let is_command = pending.is_command;
        match finished_accumulator {
            SectionAccumulator::Ability(ability_accumulator) => {
                let binding = ability_accumulator.into_binding();
                stores.bindings.insert(lowercase_key.clone(), binding);
                if !raw_text.is_empty() && !is_command {
                    stores.raw_sections.insert(lowercase_key, raw_text);
                }
            }
            SectionAccumulator::Command(command_accumulator) => {
                let binding = command_accumulator.into_binding();
                stores.commands.insert(lowercase_key.clone(), binding);
                if !raw_text.is_empty() && is_command {
                    stores.raw_command_sections.insert(lowercase_key, raw_text);
                }
            }
        }
    }
}

impl From<String> for CustomKeysFile {
    fn from(text: String) -> Self {
        let text_ref = text.as_str();
        Self::from(text_ref)
    }
}

struct KeyValuePair {
    key: String,
    value: String,
}

fn home_directory() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    } else {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_entry_with_hotkey_and_buttonpos() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let hotkey = binding.hotkey();
        assert_eq!(hotkey, Some("Q"));
        let position = binding.button_position().unwrap();
        let column = position.column();
        assert_eq!(column, 0);
        let row = position.row();
        assert_eq!(row, 2);
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let input = "[Hpal]\nHotkey=T\nButtonpos=3,0\n";
        let file = CustomKeysFile::from(input);
        let exact_result = file.binding("Hpal");
        assert!(exact_result.is_some());
        let lower_result = file.binding("hpal");
        assert!(lower_result.is_some());
        let upper_result = file.binding("HPAL");
        assert!(upper_result.is_some());
    }

    #[test]
    fn missing_hotkey_returns_none() {
        let input = "[AHbz]\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHbz").unwrap();
        let hotkey = binding.hotkey();
        assert_eq!(hotkey, None);
    }

    #[test]
    fn empty_hotkey_value_treated_as_absent() {
        let input = "[AHbz]\nHotkey=\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHbz").unwrap();
        let hotkey = binding.hotkey();
        assert_eq!(hotkey, None);
    }

    #[test]
    fn research_fields_parsed() {
        let input = "[AHhb]\nResearchhotkey=T\nResearchbuttonpos=3,1\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let research_hotkey = binding.research_hotkey();
        assert_eq!(research_hotkey, Some("T"));
        let position = binding.research_button_position().unwrap();
        let column = position.column();
        assert_eq!(column, 3);
        let row = position.row();
        assert_eq!(row, 1);
    }

    #[test]
    fn bindings_in_order_preserves_file_order() {
        let input = "[AHhb]\nHotkey=Q\n\n[AHbz]\nHotkey=W\n";
        let file = CustomKeysFile::from(input);
        let ids: Vec<&str> = file.bindings_in_order().map(|entry| entry.id()).collect();
        assert_eq!(ids, vec!["AHhb", "AHbz"]);
    }

    #[test]
    fn comment_lines_are_skipped() {
        let input = "// This is a comment\n[AHhb]\nHotkey=Q\n; Also a comment\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let hotkey = binding.hotkey();
        assert_eq!(hotkey, Some("Q"));
        let has_button_position = binding.button_position().is_some();
        assert!(has_button_position);
    }

    #[test]
    fn unknown_keys_are_silently_ignored() {
        let input = "[AHhb]\nHotkey=Q\nUnknownField=something\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let hotkey = binding.hotkey();
        assert_eq!(hotkey, Some("Q"));
    }

    #[test]
    fn malformed_buttonpos_gives_none() {
        let input = "[AHhb]\nButtonpos=notanumber\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let has_button_position = binding.button_position().is_none();
        assert!(has_button_position);
    }

    #[test]
    fn round_trip_preserves_original_casing_of_id() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,0\n\n";
        let file = CustomKeysFile::from(input);
        let content = file.to_file_content();
        let contains_header = content.contains("[AHhb]");
        assert!(contains_header);
    }

    #[test]
    fn duplicate_section_uses_first_occurrence() {
        let input = "[AHhb]\nHotkey=Q\n\n[AHhb]\nHotkey=W\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        let hotkey = binding.hotkey();
        assert_eq!(hotkey, Some("Q"));
    }

    #[test]
    fn untouched_sections_round_trip_byte_identically() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n//inline comment\nIcon=ReplaceableTextures\\CommandButtons\\BTNAvatar.blp\n\n[AHbz]\nHotkey=W\nButtonpos=1,2\n\n";
        let file = CustomKeysFile::from(input);
        let output = file.to_file_content();
        assert!(output.contains("[AHhb]"));
        assert!(output.contains("BTNAvatar.blp"));
        assert!(output.contains("[AHbz]"));
    }

    #[test]
    fn touched_section_uses_formatted_output() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n\n[AHbz]\nHotkey=W\nButtonpos=1,2\n\n";
        let mut file = CustomKeysFile::from(input);
        let binding = file.binding_or_default_mut("AHhb");
        binding.set_hotkey(Some("R".to_string()));
        let output = file.to_file_content();
        assert!(output.contains("Hotkey=R"));
        assert!(output.contains("[AHbz]\nHotkey=W"));
    }

    #[test]
    fn parses_command_section() {
        let input = "[CmdMove]\nHotkey=M\nButtonpos=1,2\nTip=Move\n";
        let file = CustomKeysFile::from(input);
        let binding = file.command("CmdMove").expect("CmdMove parsed");
        assert_eq!(binding.hotkey(), Some("M"));
        let position = binding.button_position().expect("position parsed");
        assert_eq!(position.column(), 1);
        assert_eq!(position.row(), 2);
    }

    #[test]
    fn dirty_setter_only_marks_dirty_on_actual_change() {
        let input = "[AHhb]\nHotkey=Q\n\n";
        let mut file = CustomKeysFile::from(input);
        let binding = file.binding_or_default_mut("AHhb");
        binding.set_hotkey(Some("Q".to_string()));
        assert!(!binding.is_dirty(), "setting same value should not mark dirty");
        binding.set_hotkey(Some("R".to_string()));
        assert!(binding.is_dirty(), "setting different value should mark dirty");
    }

    #[test]
    fn round_trip_of_baseline_preserves_known_sections() {
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let file = CustomKeysFile::from(baseline);
        let output = file.to_file_content();
        let known_sections = [
            "[CmdAttack]",
            "[CmdMove]",
            "[CmdRally]",
            "[CmdCancel]",
            "[CmdBuildHuman]",
            "[Hpal]",
            "[hkee]",
            "[Rhpm]",
            "[AHhb]",
        ];
        for section_marker in known_sections {
            assert!(
                output.contains(section_marker),
                "round-trip output is missing section {section_marker:?}",
            );
        }
        use std::collections::BTreeSet;
        let collect_unique_sections = |text: &str| -> BTreeSet<String> {
            text.lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.starts_with('[') && trimmed.ends_with(']') {
                        Some(trimmed.to_ascii_lowercase())
                    } else {
                        None
                    }
                })
                .collect()
        };
        let baseline_unique = collect_unique_sections(baseline);
        let output_unique = collect_unique_sections(&output);
        assert_eq!(
            baseline_unique, output_unique,
            "round-trip preserves the set of unique section headers (duplicates in the source are deduped)",
        );
    }
}
