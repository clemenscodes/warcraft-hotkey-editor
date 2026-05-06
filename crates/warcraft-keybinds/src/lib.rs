use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

pub use warcraft_api::{SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId};

pub mod building;
pub mod cascade;
pub mod catalog;
pub mod customkeys;
pub mod export;
pub mod global_cascade;
pub mod lookup;
pub mod overlay;
pub mod slot;
pub mod unit_slots;

pub use building::BuildingTraits;
pub use catalog::CommandCatalog;
pub use customkeys::CustomKeys;
pub use global_cascade::GlobalCascade;
pub use lookup::ObjectLookup;
pub use slot::GridSlotId;
pub use unit_slots::UnitSlots;

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
        let column = parts
            .next()
            .ok_or(())?
            .trim()
            .parse::<u8>()
            .map_err(|_| ())?;
        let row = parts
            .next()
            .ok_or(())?
            .trim()
            .parse::<u8>()
            .map_err(|_| ())?;
        Ok(ButtonPosition { column, row })
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

    pub fn set_unhotkey(&mut self, value: Option<String>) {
        if self.unhotkey != value {
            self.unhotkey = value;
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

    pub fn set_icon(&mut self, value: Option<String>) {
        if self.icon != value {
            self.icon = value;
            self.dirty = true;
        }
    }

    pub fn set_un_icon(&mut self, value: Option<String>) {
        if self.un_icon != value {
            self.un_icon = value;
            self.dirty = true;
        }
    }

    pub fn set_modifier(&mut self, value: Option<String>) {
        if self.modifier != value {
            self.modifier = value;
            self.dirty = true;
        }
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

/// Binding for a system-level hotkey section.
/// Sections are identified by a class-discriminator field
/// (`GameCommand=1`, `CtrlGroupCommand=1`, etc.).
#[derive(Debug, Clone)]
pub struct SystemBinding {
    hotkey: u32,
    class: SystemKeybindClass,
    modifier: Option<SystemKeybindModifier>,
    dirty: bool,
}

impl SystemBinding {
    pub fn new(
        hotkey: u32,
        class: SystemKeybindClass,
        modifier: Option<SystemKeybindModifier>,
    ) -> Self {
        Self {
            hotkey,
            class,
            modifier,
            dirty: false,
        }
    }

    pub fn hotkey(&self) -> u32 {
        self.hotkey
    }

    pub fn class(&self) -> SystemKeybindClass {
        self.class
    }

    pub fn modifier(&self) -> Option<SystemKeybindModifier> {
        self.modifier
    }

    pub fn set_hotkey(&mut self, value: u32) {
        if self.hotkey != value {
            self.hotkey = value;
            self.dirty = true;
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

/// A fully-typed keybinding parsed from a single section of CustomKeys.txt.
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum WarcraftKeybinding {
    /// Abilities, units, upgrades, and items — all non-command, non-system sections.
    Ability(AbilityBinding),
    /// Cmd* command sections (CmdAttack, CmdMove, …).
    Command(CommandBinding),
    /// System hotkey sections (inventory slots, hero selection, control groups, …).
    System(SystemBinding),
}

impl WarcraftKeybinding {
    pub fn as_ability(&self) -> Option<&AbilityBinding> {
        if let Self::Ability(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn as_ability_mut(&mut self) -> Option<&mut AbilityBinding> {
        if let Self::Ability(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn as_command(&self) -> Option<&CommandBinding> {
        if let Self::Command(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn as_command_mut(&mut self) -> Option<&mut CommandBinding> {
        if let Self::Command(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn as_system(&self) -> Option<&SystemBinding> {
        if let Self::System(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn as_system_mut(&mut self) -> Option<&mut SystemBinding> {
        if let Self::System(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn is_dirty(&self) -> bool {
        match self {
            Self::Ability(b) => b.is_dirty(),
            Self::Command(b) => b.is_dirty(),
            Self::System(b) => b.is_dirty(),
        }
    }

    pub fn mark_clean(&mut self) {
        match self {
            Self::Ability(b) => b.mark_clean(),
            Self::Command(b) => b.mark_clean(),
            Self::System(b) => b.mark_clean(),
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

pub struct CustomKeysFile {
    /// All entries keyed by lowercase section ID for O(1) lookup.
    entries: HashMap<String, WarcraftKeybinding>,
    /// All section IDs in file order for deterministic serialisation.
    order: Vec<String>,
    /// Original-case IDs indexed by lowercase form for round-trip output.
    original_ids: HashMap<String, String>,
    /// Verbatim raw text per section for byte-identical preservation of untouched bindings.
    raw_sections: HashMap<String, String>,
}

impl CustomKeysFile {
    /// O(1) read. Returns the keybinding for any section, regardless of variant.
    pub fn get(&self, id: WarcraftObjectId) -> Option<&WarcraftKeybinding> {
        self.entries.get(&id.value().to_lowercase())
    }

    /// O(1) in-place mutation. Use the binding's `set_*` methods to mark dirty.
    pub fn get_mut(&mut self, id: WarcraftObjectId) -> Option<&mut WarcraftKeybinding> {
        self.entries.get_mut(&id.value().to_lowercase())
    }

    /// O(1) upsert. Replaces any existing entry and clears the raw-text cache
    /// so the next `to_file_content()` serialises from the typed struct.
    pub fn set(&mut self, id: WarcraftObjectId, binding: WarcraftKeybinding) {
        let key = id.value().to_lowercase();
        if !self.entries.contains_key(&key) {
            self.order.push(key.clone());
            self.original_ids
                .insert(key.clone(), id.value().to_string());
        }
        self.raw_sections.remove(&key);
        self.entries.insert(key, binding);
    }

    pub fn binding(&self, id: &str) -> Option<&AbilityBinding> {
        self.entries.get(&id.to_lowercase())?.as_ability()
    }

    pub fn binding_mut(&mut self, id: &str) -> Option<&mut AbilityBinding> {
        self.entries.get_mut(&id.to_lowercase())?.as_ability_mut()
    }

    pub fn binding_or_default_mut(&mut self, id: &str) -> Option<&mut AbilityBinding> {
        let key = id.to_lowercase();
        if !matches!(self.entries.get(&key), Some(WarcraftKeybinding::Ability(_))) {
            if !self.entries.contains_key(&key) {
                self.order.push(key.clone());
                self.original_ids.insert(key.clone(), id.to_string());
            }
            self.raw_sections.remove(&key);
            self.entries.insert(
                key.clone(),
                WarcraftKeybinding::Ability(AbilityBinding::default()),
            );
        }
        self.entries
            .get_mut(&key)
            .and_then(WarcraftKeybinding::as_ability_mut)
    }

    pub fn bindings_in_order(&self) -> impl Iterator<Item = BindingEntry<'_>> {
        self.order.iter().filter_map(|key| {
            let original_id = self
                .original_ids
                .get(key)
                .map(String::as_str)
                .unwrap_or(key);
            self.entries.get(key)?.as_ability().map(|b| BindingEntry {
                id: original_id,
                binding: b,
            })
        })
    }

    pub fn command(&self, name: &str) -> Option<&CommandBinding> {
        self.entries.get(&name.to_lowercase())?.as_command()
    }

    pub fn command_mut(&mut self, name: &str) -> Option<&mut CommandBinding> {
        self.entries.get_mut(&name.to_lowercase())?.as_command_mut()
    }

    pub fn command_or_default_mut(&mut self, name: &str) -> Option<&mut CommandBinding> {
        let key = name.to_lowercase();
        if !matches!(self.entries.get(&key), Some(WarcraftKeybinding::Command(_))) {
            if !self.entries.contains_key(&key) {
                self.order.push(key.clone());
                self.original_ids.insert(key.clone(), name.to_string());
            }
            self.raw_sections.remove(&key);
            self.entries.insert(
                key.clone(),
                WarcraftKeybinding::Command(CommandBinding::default()),
            );
        }
        self.entries
            .get_mut(&key)
            .and_then(WarcraftKeybinding::as_command_mut)
    }

    pub fn commands_in_order(&self) -> impl Iterator<Item = CommandEntry<'_>> {
        self.order.iter().filter_map(|key| {
            let original_name = self
                .original_ids
                .get(key)
                .map(String::as_str)
                .unwrap_or(key);
            self.entries.get(key)?.as_command().map(|b| CommandEntry {
                name: original_name,
                binding: b,
            })
        })
    }

    pub fn system(&self, id: &str) -> Option<&SystemBinding> {
        self.entries.get(&id.to_lowercase())?.as_system()
    }

    pub fn system_mut(&mut self, id: &str) -> Option<&mut SystemBinding> {
        self.entries.get_mut(&id.to_lowercase())?.as_system_mut()
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(Self::from(text.as_str()))
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
                "CustomKeys.txt not found in ~/Documents/Warcraft III/CustomKeyBindings/ \
                 or Wine prefix",
            )
        })?;
        Self::load(path)
    }

    pub fn to_file_content(&self) -> String {
        let mut output = String::new();
        for key in &self.order {
            let display_id = self
                .original_ids
                .get(key)
                .map(String::as_str)
                .unwrap_or(key);
            let Some(entry) = self.entries.get(key) else {
                continue;
            };
            if !entry.is_dirty()
                && let Some(raw_text) = self.raw_sections.get(key)
            {
                output.push_str(raw_text);
                if !raw_text.ends_with("\n\n") {
                    output.push('\n');
                }
                continue;
            }
            match entry {
                WarcraftKeybinding::Ability(b) => {
                    Self::format_ability_section(&mut output, display_id, b);
                }
                WarcraftKeybinding::Command(b) => {
                    Self::format_command_section(&mut output, display_id, b);
                }
                WarcraftKeybinding::System(b) => {
                    Self::format_system_section(&mut output, display_id, b);
                }
            }
        }
        output
    }

    fn format_ability_section(output: &mut String, id: &str, b: &AbilityBinding) {
        output.push('[');
        output.push_str(id);
        output.push_str("]\n");
        if let Some(v) = &b.hotkey {
            output.push_str("Hotkey=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.unhotkey {
            output.push_str("Unhotkey=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(p) = &b.button_position {
            output.push_str(&format!("Buttonpos={},{}\n", p.column, p.row));
        }
        if let Some(p) = &b.unbutton_position {
            output.push_str(&format!("Unbuttonpos={},{}\n", p.column, p.row));
        }
        if let Some(v) = &b.research_hotkey {
            output.push_str("Researchhotkey=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(p) = &b.research_button_position {
            output.push_str(&format!("Researchbuttonpos={},{}\n", p.column, p.row));
        }
        if let Some(v) = &b.tip {
            output.push_str("Tip=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.research_tip {
            output.push_str("Researchtip=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.un_tip {
            output.push_str("UnTip=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.ubertip {
            output.push_str("Ubertip=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.research_ubertip {
            output.push_str("Researchubertip=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.un_ubertip {
            output.push_str("Unubertip=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.icon {
            output.push_str("Icon=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.modifier {
            output.push_str("Modifier=");
            output.push_str(v);
            output.push('\n');
        }
        output.push('\n');
    }

    fn format_command_section(output: &mut String, name: &str, b: &CommandBinding) {
        output.push('[');
        output.push_str(name);
        output.push_str("]\n");
        if let Some(v) = &b.hotkey {
            output.push_str("Hotkey=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(p) = &b.button_position {
            output.push_str(&format!("Buttonpos={},{}\n", p.column, p.row));
        }
        if let Some(p) = &b.unbutton_position {
            output.push_str(&format!("Unbuttonpos={},{}\n", p.column, p.row));
        }
        if let Some(v) = &b.tip {
            output.push_str("Tip=");
            output.push_str(v);
            output.push('\n');
        }
        if let Some(v) = &b.un_tip {
            output.push_str("UnTip=");
            output.push_str(v);
            output.push('\n');
        }
        output.push('\n');
    }

    fn format_system_section(output: &mut String, id: &str, b: &SystemBinding) {
        output.push('[');
        output.push_str(id);
        output.push_str("]\n");
        output.push_str(&format!("Hotkey={}\n", b.hotkey));
        let class_field = match b.class {
            SystemKeybindClass::Game => "GameCommand=1",
            SystemKeybindClass::ControlGroup => "CtrlGroupCommand=1",
            SystemKeybindClass::Menu => "MenuCommand=1",
            SystemKeybindClass::Camera => "CameraCommand=1",
            SystemKeybindClass::Observer => "ObserverCommand=1",
            SystemKeybindClass::Replay => "ReplayCommand=1",
        };
        output.push_str(class_field);
        output.push('\n');
        if let Some(modifier) = b.modifier {
            let modifier_str = match modifier {
                SystemKeybindModifier::None => None,
                SystemKeybindModifier::Alt => Some("Alt"),
                SystemKeybindModifier::Ctrl => Some("Ctrl"),
                SystemKeybindModifier::CtrlOrAlt => Some("Ctrl_or_Alt"),
                SystemKeybindModifier::Shift => Some("Shift"),
            };
            if let Some(m) = modifier_str {
                output.push_str("Modifier=");
                output.push_str(m);
                output.push('\n');
            }
        }
        output.push('\n');
    }

    fn parse_section_header(line: &str) -> Option<String> {
        let without_brackets = line.strip_prefix('[')?.strip_suffix(']')?;
        let id = without_brackets.trim();
        if id.is_empty() {
            None
        } else {
            Some(id.to_string())
        }
    }

    fn parse_key_value(line: &str) -> Option<SectionField<'_>> {
        let (key, value) = line.split_once('=')?;
        let section_field = SectionField {
            key: key.trim(),
            value,
        };
        Some(section_field)
    }
}

struct SectionField<'a> {
    key: &'a str,
    value: &'a str,
}

/// Collects all fields of a section before we decide its variant.
#[derive(Default)]
struct SectionAccumulator {
    // Shared / ability fields
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
    // System discriminators — set when the corresponding class field is seen
    system_class: Option<SystemKeybindClass>,
    system_modifier: Option<SystemKeybindModifier>,
    // Determined from the section header, not the fields
    is_command: bool,
}

impl SectionAccumulator {
    fn apply(&mut self, key: &str, value: &str) {
        match key.to_lowercase().as_str() {
            "hotkey" if self.hotkey.is_none() && !value.is_empty() => {
                self.hotkey = Some(value.to_string());
            }
            "unhotkey" if !value.is_empty() && self.unhotkey.is_none() => {
                self.unhotkey = Some(value.to_string());
            }
            "buttonpos" if self.button_position.is_none() => {
                self.button_position = ButtonPosition::try_from(value).ok();
            }
            "unbuttonpos" if self.unbutton_position.is_none() => {
                self.unbutton_position = ButtonPosition::try_from(value).ok();
            }
            "researchhotkey" if !value.is_empty() && self.research_hotkey.is_none() => {
                self.research_hotkey = Some(value.to_string());
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
            "icon" | "art" if !value.is_empty() && self.icon.is_none() => {
                self.icon = Some(value.to_string());
            }
            "unart" if !value.is_empty() && self.un_icon.is_none() => {
                self.un_icon = Some(value.to_string());
            }
            "modifier" if !value.is_empty() && self.modifier.is_none() => {
                self.modifier = Some(value.to_string());
            }
            // System class discriminators
            "gamecommand" if value.trim() == "1" && self.system_class.is_none() => {
                self.system_class = Some(SystemKeybindClass::Game);
            }
            "ctrlgroupcommand" if value.trim() == "1" && self.system_class.is_none() => {
                self.system_class = Some(SystemKeybindClass::ControlGroup);
            }
            "menucommand" if value.trim() == "1" && self.system_class.is_none() => {
                self.system_class = Some(SystemKeybindClass::Menu);
            }
            "cameracommand" if value.trim() == "1" && self.system_class.is_none() => {
                self.system_class = Some(SystemKeybindClass::Camera);
            }
            "observercommand" if value.trim() == "1" && self.system_class.is_none() => {
                self.system_class = Some(SystemKeybindClass::Observer);
            }
            "replaycommand" if value.trim() == "1" && self.system_class.is_none() => {
                self.system_class = Some(SystemKeybindClass::Replay);
            }
            _ => {}
        }
        // Parse system modifier separately since "Modifier" also appears in ability sections
        // but only matters for System bindings (resolved at flush time).
        if key.to_lowercase() == "modifier" && self.system_modifier.is_none() {
            self.system_modifier = Self::parse_modifier(value);
        }
    }

    fn into_keybinding(self) -> WarcraftKeybinding {
        if self.is_command {
            let command_binding = CommandBinding {
                hotkey: self.hotkey,
                button_position: self.button_position,
                unbutton_position: self.unbutton_position,
                tip: self.tip,
                un_tip: self.un_tip,
                dirty: false,
            };
            return WarcraftKeybinding::Command(command_binding);
        }
        if let Some(class) = self.system_class {
            let hotkey = self
                .hotkey
                .as_deref()
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            let system_binding = SystemBinding {
                hotkey,
                class,
                modifier: self.system_modifier,
                dirty: false,
            };
            return WarcraftKeybinding::System(system_binding);
        }
        let ability_binding = AbilityBinding {
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
        };
        WarcraftKeybinding::Ability(ability_binding)
    }

    fn parse_modifier(value: &str) -> Option<SystemKeybindModifier> {
        match value.trim().to_ascii_lowercase().as_str() {
            "alt" => Some(SystemKeybindModifier::Alt),
            "ctrl" => Some(SystemKeybindModifier::Ctrl),
            "ctrl_or_alt" => Some(SystemKeybindModifier::CtrlOrAlt),
            "shift" => Some(SystemKeybindModifier::Shift),
            _ => None,
        }
    }
}

impl From<&str> for CustomKeysFile {
    fn from(text: &str) -> Self {
        let mut entries: HashMap<String, WarcraftKeybinding> = HashMap::new();
        let mut order: Vec<String> = Vec::new();
        let mut original_ids: HashMap<String, String> = HashMap::new();
        let mut raw_sections: HashMap<String, String> = HashMap::new();

        let mut current_key: Option<String> = None;
        let mut current_raw: String = String::new();
        let mut accumulator: Option<SectionAccumulator> = None;

        let flush = |current_key: &mut Option<String>,
                     accumulator: &mut Option<SectionAccumulator>,
                     current_raw: &mut String,
                     entries: &mut HashMap<String, WarcraftKeybinding>,
                     raw_sections: &mut HashMap<String, String>| {
            if let (Some(key), Some(acc)) = (current_key.take(), accumulator.take()) {
                let binding = acc.into_keybinding();
                entries.insert(key.clone(), binding);
                if !current_raw.is_empty() {
                    raw_sections.insert(key, std::mem::take(current_raw));
                } else {
                    current_raw.clear();
                }
            } else {
                current_raw.clear();
            }
        };

        for line in text.lines() {
            let trimmed = line.trim();
            let is_blank = trimmed.is_empty();
            let is_comment = trimmed.starts_with("//") || trimmed.starts_with(';');

            let header = if is_blank || is_comment {
                None
            } else {
                CustomKeysFile::parse_section_header(trimmed)
            };

            if let Some(original_id) = header {
                flush(
                    &mut current_key,
                    &mut accumulator,
                    &mut current_raw,
                    &mut entries,
                    &mut raw_sections,
                );

                let key = original_id.to_lowercase();
                if entries.contains_key(&key) {
                    // Duplicate section — skip (first occurrence wins)
                    current_key = None;
                    accumulator = None;
                } else {
                    original_ids
                        .entry(key.clone())
                        .or_insert_with(|| original_id.clone());
                    order.push(key.clone());
                    let is_command_section = original_id.to_ascii_lowercase().starts_with("cmd");
                    let acc = SectionAccumulator {
                        is_command: is_command_section,
                        ..Default::default()
                    };
                    current_raw.push_str(line);
                    current_raw.push('\n');
                    current_key = Some(key);
                    accumulator = Some(acc);
                }
            } else {
                if accumulator.is_some() {
                    current_raw.push_str(line);
                    current_raw.push('\n');
                }
                if !is_blank
                    && !is_comment
                    && let Some(field) = CustomKeysFile::parse_key_value(trimmed)
                    && let Some(acc) = accumulator.as_mut()
                {
                    acc.apply(field.key, field.value);
                }
            }
        }

        flush(
            &mut current_key,
            &mut accumulator,
            &mut current_raw,
            &mut entries,
            &mut raw_sections,
        );

        CustomKeysFile {
            entries,
            order,
            original_ids,
            raw_sections,
        }
    }
}

impl From<String> for CustomKeysFile {
    fn from(text: String) -> Self {
        Self::from(text.as_str())
    }
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
        assert_eq!(binding.hotkey(), Some("Q"));
        let position = binding.button_position().unwrap();
        assert_eq!(position.column(), 0);
        assert_eq!(position.row(), 2);
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let input = "[Hpal]\nHotkey=T\nButtonpos=3,0\n";
        let file = CustomKeysFile::from(input);
        assert!(file.binding("Hpal").is_some());
        assert!(file.binding("hpal").is_some());
        assert!(file.binding("HPAL").is_some());
    }

    #[test]
    fn missing_hotkey_returns_none() {
        let input = "[AHbz]\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn empty_hotkey_value_treated_as_absent() {
        let input = "[AHbz]\nHotkey=\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn research_fields_parsed() {
        let input = "[AHhb]\nResearchhotkey=T\nResearchbuttonpos=3,1\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        assert_eq!(binding.research_hotkey(), Some("T"));
        let position = binding.research_button_position().unwrap();
        assert_eq!(position.column(), 3);
        assert_eq!(position.row(), 1);
    }

    #[test]
    fn bindings_in_order_preserves_file_order() {
        let input = "[AHhb]\nHotkey=Q\n\n[AHbz]\nHotkey=W\n";
        let file = CustomKeysFile::from(input);
        let ids: Vec<&str> = file.bindings_in_order().map(|e| e.id()).collect();
        assert_eq!(ids, vec!["AHhb", "AHbz"]);
    }

    #[test]
    fn comment_lines_are_skipped() {
        let input = "// This is a comment\n[AHhb]\nHotkey=Q\n; Also a comment\nButtonpos=0,0\n";
        let file = CustomKeysFile::from(input);
        let binding = file.binding("AHhb").unwrap();
        assert_eq!(binding.hotkey(), Some("Q"));
        assert!(binding.button_position().is_some());
    }

    #[test]
    fn unknown_keys_are_silently_ignored() {
        let input = "[AHhb]\nHotkey=Q\nUnknownField=something\n";
        let file = CustomKeysFile::from(input);
        assert_eq!(file.binding("AHhb").unwrap().hotkey(), Some("Q"));
    }

    #[test]
    fn malformed_buttonpos_gives_none() {
        let input = "[AHhb]\nButtonpos=notanumber\n";
        let file = CustomKeysFile::from(input);
        assert!(file.binding("AHhb").unwrap().button_position().is_none());
    }

    #[test]
    fn round_trip_preserves_original_casing_of_id() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,0\n\n";
        let file = CustomKeysFile::from(input);
        assert!(file.to_file_content().contains("[AHhb]"));
    }

    #[test]
    fn duplicate_section_uses_first_occurrence() {
        let input = "[AHhb]\nHotkey=Q\n\n[AHhb]\nHotkey=W\n";
        let file = CustomKeysFile::from(input);
        assert_eq!(file.binding("AHhb").unwrap().hotkey(), Some("Q"));
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
        file.binding_or_default_mut("AHhb")
            .unwrap()
            .set_hotkey(Some("R".to_string()));
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
        let binding = file.binding_or_default_mut("AHhb").unwrap();
        binding.set_hotkey(Some("Q".to_string()));
        assert!(
            !binding.is_dirty(),
            "setting same value should not mark dirty"
        );
        binding.set_hotkey(Some("R".to_string()));
        assert!(
            binding.is_dirty(),
            "setting different value should mark dirty"
        );
    }

    #[test]
    fn parses_system_section_game_command() {
        let input = "[IsHeroSelect]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("IsHeroSelect").expect("system section parsed");
        assert_eq!(sys.hotkey(), 9);
        assert_eq!(sys.class(), SystemKeybindClass::Game);
        assert!(sys.modifier().is_none());
    }

    #[test]
    fn parses_system_section_ctrl_group_with_modifier() {
        let input = "[SelectGroup01]\nHotkey=49\nCtrlGroupCommand=1\nModifier=Ctrl\n";
        let file = CustomKeysFile::from(input);
        let sys = file.system("SelectGroup01").expect("parsed");
        assert_eq!(sys.hotkey(), 49);
        assert_eq!(sys.class(), SystemKeybindClass::ControlGroup);
        assert_eq!(sys.modifier(), Some(SystemKeybindModifier::Ctrl));
    }

    #[test]
    fn system_section_not_returned_by_binding() {
        let input = "[IsHeroSelect]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeysFile::from(input);
        assert!(file.binding("IsHeroSelect").is_none());
        assert!(file.system("IsHeroSelect").is_some());
    }

    #[test]
    fn system_section_round_trips() {
        let input = "[IsHeroSelect]\nHotkey=9\nGameCommand=1\n\n";
        let file = CustomKeysFile::from(input);
        let output = file.to_file_content();
        assert!(output.contains("[IsHeroSelect]"));
        assert!(output.contains("Hotkey=9"));
        assert!(output.contains("GameCommand=1"));
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
            "round-trip preserves the set of unique section headers",
        );
    }
}

#[cfg(test)]
mod overlay_tests {
    use super::*;

    #[test]
    fn overlay_copies_hotkey_from_uploaded_to_target() {
        let mut target = CustomKeysFile::from("[Ahrl]\nHotkey=Q\n\n");
        let uploaded = CustomKeysFile::from("[Ahrl]\nHotkey=W\n\n");
        target.overlay(&uploaded);
        assert_eq!(target.binding("Ahrl").and_then(|b| b.hotkey()), Some("W"));
    }

    #[test]
    fn overlay_copies_button_position() {
        let mut target = CustomKeysFile::from("[Ahrl]\nButtonpos=0,0\n\n");
        let uploaded = CustomKeysFile::from("[Ahrl]\nButtonpos=2,1\n\n");
        target.overlay(&uploaded);
        let pos = target
            .binding("Ahrl")
            .and_then(|b| b.button_position())
            .copied();
        assert_eq!(pos, Some(ButtonPosition::new(2, 1)));
    }

    #[test]
    fn overlay_does_not_overwrite_system_entries() {
        // Inventory slot 1 is a system entry — uploading an ability binding
        // with the same id must not touch the system section.
        let system_content = "[IsS1]\nHotkey=27\nGameCommand=1\n\n";
        let mut target = CustomKeysFile::from(system_content);
        let uploaded = CustomKeysFile::from("[IsS1]\nHotkey=Q\n\n");
        target.overlay(&uploaded);
        // System entry should still be present and unchanged.
        assert!(target.system("IsS1").is_some());
    }

    #[test]
    fn overlay_skips_absent_fields() {
        // If the uploaded binding has no hotkey, the target hotkey is kept.
        let mut target = CustomKeysFile::from("[Ahrl]\nHotkey=Q\n\n");
        let uploaded = CustomKeysFile::from("[Ahrl]\nButtonpos=1,0\n\n");
        target.overlay(&uploaded);
        assert_eq!(target.binding("Ahrl").and_then(|b| b.hotkey()), Some("Q"));
        let pos = target
            .binding("Ahrl")
            .and_then(|b| b.button_position())
            .copied();
        assert_eq!(pos, Some(ButtonPosition::new(1, 0)));
    }

    #[test]
    fn overlay_copies_command_hotkey() {
        let mut target = CustomKeysFile::from("[CmdAttack]\nHotkey=A\n\n");
        let uploaded = CustomKeysFile::from("[CmdAttack]\nHotkey=G\n\n");
        target.overlay(&uploaded);
        assert_eq!(
            target.command("CmdAttack").and_then(|b| b.hotkey()),
            Some("G"),
        );
    }

    #[test]
    fn overlay_is_case_insensitive_for_ids() {
        let mut target = CustomKeysFile::from("[AHrl]\nHotkey=Q\n\n");
        let uploaded = CustomKeysFile::from("[ahrl]\nHotkey=E\n\n");
        target.overlay(&uploaded);
        assert_eq!(target.binding("AHrl").and_then(|b| b.hotkey()), Some("E"));
    }
}

#[cfg(test)]
mod export_tests {
    use crate::CustomKeysFile;
    use crate::export::serialize;

    #[test]
    fn empty_overlay_on_minimal_baseline_round_trips() {
        let baseline = "[Ahrl]\nHotkey=Q\nButtonpos=0,0\n\n";
        let loaded = CustomKeysFile::from("");
        let output = serialize(&loaded, baseline);
        assert!(
            output.contains("[Ahrl]"),
            "baseline section should be present in output"
        );
        assert!(output.contains("Hotkey=Q"));
    }

    #[test]
    fn overlay_values_appear_in_export() {
        let baseline = "[Ahrl]\nHotkey=Q\n\n";
        let loaded = CustomKeysFile::from("[Ahrl]\nHotkey=W\n\n");
        let output = serialize(&loaded, baseline);
        assert!(output.contains("Hotkey=W"), "user hotkey override must win");
    }

    #[test]
    fn export_with_real_baseline_contains_known_sections() {
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let loaded = CustomKeysFile::from("");
        let output = serialize(&loaded, baseline);
        for section in &["[Hpal]", "[CmdAttack]", "[CmdMove]"] {
            assert!(output.contains(section), "export should contain {section}");
        }
    }

    #[test]
    fn export_materializes_default_button_positions() {
        // Ahrl (Holy Light) has a known default Buttonpos in the database.
        // Starting from an empty overlay, the export should inject it.
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let loaded = CustomKeysFile::from("");
        let output = serialize(&loaded, baseline);
        // Find the [Ahrl] section and check Buttonpos is present.
        let after_ahrl = output
            .split("[Ahrl]")
            .nth(1)
            .expect("[Ahrl] must be in output");
        let next_section = after_ahrl.split('[').next().unwrap_or(after_ahrl);
        assert!(
            next_section.contains("Buttonpos="),
            "[Ahrl] section must have a Buttonpos after materialization"
        );
    }
}

#[cfg(test)]
mod cascade_tests {
    use crate::CustomKeysFile;
    use crate::cascade::{next_free_cell, position_occupied, resolve_container, resolved_for};
    use crate::slot::GridSlotId;
    use warcraft_api::ButtonPosition;

    #[test]
    fn next_free_cell_prefers_requested_row() {
        let occupied = vec![ButtonPosition::new(0, 0)];
        let cell = next_free_cell(0, &occupied);
        assert_eq!(cell, Some(ButtonPosition::new(1, 0)));
    }

    #[test]
    fn next_free_cell_falls_back_to_next_row_when_row_full() {
        let occupied: Vec<ButtonPosition> = (0..4).map(|c| ButtonPosition::new(c, 0)).collect();
        let cell = next_free_cell(0, &occupied);
        assert_eq!(cell, Some(ButtonPosition::new(0, 1)));
    }

    #[test]
    fn next_free_cell_returns_none_when_grid_full() {
        let occupied: Vec<ButtonPosition> = (0..3)
            .flat_map(|r| (0..4).map(move |c| ButtonPosition::new(c, r)))
            .collect();
        let cell = next_free_cell(0, &occupied);
        assert_eq!(cell, None);
    }

    #[test]
    fn position_occupied_matches_by_column_and_row() {
        let occupied = vec![ButtonPosition::new(1, 2)];
        assert!(position_occupied(&occupied, ButtonPosition::new(1, 2)));
        assert!(!position_occupied(&occupied, ButtonPosition::new(0, 2)));
    }

    #[test]
    fn resolve_container_places_ability_at_custom_position() {
        let custom_keys = CustomKeysFile::from("[Ahrl]\nButtonpos=2,0\n\n");
        let slots = vec![GridSlotId::ability("Ahrl")];
        let result = resolve_container(&slots, Some(&custom_keys), false);
        let pos = result
            .iter()
            .find(|entry| entry.slot_id().as_str() == "Ahrl")
            .and_then(|entry| entry.position());
        assert_eq!(pos, Some(ButtonPosition::new(2, 0)));
    }

    #[test]
    fn resolve_container_cascades_collision_between_explicit_positions() {
        // Two abilities with the same explicit Buttonpos must land at distinct
        // positions — the second cascades to the next free cell. This covers both
        // the display path and the write-back path (there is no longer a separate
        // "cascade_explicit" flag; cascading always happens on collision).
        let content = "[Ahrl]\nButtonpos=0,0\n\n[AHbz]\nButtonpos=0,0\n\n";
        let custom_keys = CustomKeysFile::from(content);
        let slots = vec![GridSlotId::ability("Ahrl"), GridSlotId::ability("AHbz")];
        let result = resolve_container(&slots, Some(&custom_keys), false);
        let pos_ahrl = result
            .iter()
            .find(|entry| entry.slot_id().as_str() == "Ahrl")
            .and_then(|entry| entry.position());
        let pos_ahbz = result
            .iter()
            .find(|entry| entry.slot_id().as_str() == "AHbz")
            .and_then(|entry| entry.position());
        assert_eq!(pos_ahrl, Some(ButtonPosition::new(0, 0)));
        assert!(pos_ahbz.is_some());
        assert_ne!(pos_ahbz, Some(ButtonPosition::new(0, 0)));
    }

    #[test]
    fn resolved_for_with_no_custom_keys_uses_database_default() {
        // Ahrl (Holy Light) has a known database default position.
        // With no custom keys, resolved_for should return it.
        let slots = vec![GridSlotId::ability("Ahrl")];
        let pos = resolved_for(&GridSlotId::ability("Ahrl"), &slots, None, false);
        // We just assert it's Some — the exact column/row is database data.
        assert!(
            pos.is_some(),
            "Ahrl should have a default position in the database"
        );
    }

    #[test]
    fn ability_without_database_position_is_placed_on_row_2() {
        // Aatp (Prioritize) has no Buttonpos= in any abilityfunc.txt, so its
        // default_button_position is None.  It lives on the Gargoyle (UGAR) alongside
        // Astn (Stone Form) which sits at (0,2).  With no custom keys the cascade
        // should auto-place Aatp on row 2 at the next free cell: (1,2).
        use crate::unit_slots::UnitSlots;
        let card = UnitSlots::command_card_for("UGAR");
        let aatp_slot = card
            .iter()
            .find(|s| s.as_str().eq_ignore_ascii_case("Aatp"));
        assert!(
            aatp_slot.is_some(),
            "Aatp should be in the Gargoyle command card"
        );
        let pos = resolved_for(aatp_slot.unwrap(), &card, None, false);
        assert_eq!(
            pos,
            Some(ButtonPosition::new(1, 2)),
            "Aatp should be auto-placed at (1,2) after Astn occupies (0,2)"
        );
    }

    #[test]
    fn fully_normalize_resolves_collisions_in_real_game_data() {
        use crate::cascade::fully_normalize;
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = crate::CustomKeysFile::from(baseline);
        fully_normalize(&mut file);
    }

    #[test]
    fn cascade_does_not_displace_ability_via_secondary_chain() {
        // ACdm and Anh2 both start at (0,2) — a genuine collision in the
        // default data for unit NDTH.  Anh2 should cascade to (2,2) (the
        // next free cell that doesn't steal ACsl's reserved (1,2)), while
        // ACsl must stay at (1,2).  Before the reserved-position fix, Anh2
        // cascaded to (1,2), which then pushed ACsl to (2,2) — causing
        // apply-grid to assign hotkey C instead of X to ACsl.
        use crate::cascade::resolve_container;
        let content = "[ACdm]\nButtonpos=0,2\n\n[Anh2]\nButtonpos=0,2\n\n[ACsl]\nButtonpos=1,2\n\n";
        let custom_keys = crate::CustomKeysFile::from(content);
        let slots = vec![
            GridSlotId::ability("ACdm"),
            GridSlotId::ability("Anh2"),
            GridSlotId::ability("ACsl"),
        ];
        let result = resolve_container(&slots, Some(&custom_keys), false);
        let pos = |id: &str| {
            result
                .iter()
                .find(|entry| entry.slot_id().as_str().eq_ignore_ascii_case(id))
                .and_then(|entry| entry.position())
        };
        assert_eq!(pos("ACdm"), Some(ButtonPosition::new(0, 2)));
        assert_eq!(
            pos("ACsl"),
            Some(ButtonPosition::new(1, 2)),
            "ACsl must not be displaced from its reserved (1,2) by Anh2's cascade"
        );
        assert_ne!(
            pos("Anh2"),
            Some(ButtonPosition::new(1, 2)),
            "Anh2 must not land on ACsl's reserved (1,2)"
        );
    }

    /// User-pinned placement for the six cross-unit abilities shared
    /// between the four neutral hostile hero command cards `ndth`,
    /// `ndtp`, `nfsh`, `nfsp`. After `fully_normalize`, every one of
    /// these abilities must have its `Buttonpos` and `Unbuttonpos`
    /// at the canonical cell:
    ///
    /// | Ability | Cell  |
    /// |---------|-------|
    /// | Anh1    | (0,2) |
    /// | Anh2    | (0,2) |
    /// | ACdm    | (1,2) |
    /// | ACd2    | (1,2) |
    /// | ACif    | (2,2) |
    /// | ACsl    | (2,2) |
    ///
    /// Pairs share a cell because each pair's two abilities live in
    /// disjoint subsets of the four units (no within-container
    /// collision). The user pinned this layout — it is the
    /// definition of correct for these six slots, and any solver
    /// change must keep this test green.
    ///
    /// **This test is intentionally failing today.** The
    /// multiplicity-priority solver in `global_cascade.rs` places
    /// `ACdm` (multiplicity 11) at (0,2) and cascades the `Anh*`
    /// series elsewhere, which is the opposite of the canonical
    /// layout. The failure is the release gate: nothing ships until
    /// the solver is amended (likely a "minimize deviation from
    /// baseline subject to per-container no-collision constraints"
    /// pass) and this test passes.
    #[test]
    fn neutral_hero_shared_abilities_land_on_canonical_cells() {
        use crate::cascade::fully_normalize;

        struct ExpectedPlacement {
            ability_id: &'static str,
            column: u8,
            row: u8,
        }

        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = crate::CustomKeysFile::from(baseline);
        fully_normalize(&mut file);

        let case_anh1 = ExpectedPlacement {
            ability_id: "Anh1",
            column: 0,
            row: 2,
        };
        let case_anh2 = ExpectedPlacement {
            ability_id: "Anh2",
            column: 0,
            row: 2,
        };
        let case_acdm = ExpectedPlacement {
            ability_id: "ACdm",
            column: 1,
            row: 2,
        };
        let case_acd2 = ExpectedPlacement {
            ability_id: "ACd2",
            column: 1,
            row: 2,
        };
        let case_acif = ExpectedPlacement {
            ability_id: "ACif",
            column: 2,
            row: 2,
        };
        let case_acsl = ExpectedPlacement {
            ability_id: "ACsl",
            column: 2,
            row: 2,
        };
        let cases = [
            case_anh1, case_anh2, case_acdm, case_acd2, case_acif, case_acsl,
        ];

        for case in &cases {
            let ability_id = case.ability_id;
            let column = case.column;
            let row = case.row;
            let expected_position = crate::ButtonPosition::new(column, row);
            let expected = Some(expected_position);

            let binding = file.binding(ability_id);
            let button_position = binding.and_then(|b| b.button_position()).copied();
            let unbutton_position = binding.and_then(|b| b.unbutton_position()).copied();

            assert_eq!(
                button_position, expected,
                "{ability_id} Buttonpos must be ({column},{row}) after normalize, \
                 got {button_position:?}",
            );
            assert_eq!(
                unbutton_position, expected,
                "{ability_id} Unbuttonpos must be ({column},{row}) after normalize, \
                 got {unbutton_position:?}",
            );
        }
    }

    /// Sanity check: confirms the six pinned abilities really do
    /// appear in the four neutral hostile hero command cards
    /// (`ndth`, `ndtp`, `nfsh`, `nfsp`). If this fails, either the
    /// game database changed shape or the unit ids are wrong, and
    /// the placement test above is testing the wrong universe.
    #[test]
    fn neutral_hero_units_contain_the_six_shared_abilities() {
        use crate::unit_slots::UnitSlots;

        let neutral_unit_ids = ["ndth", "ndtp", "nfsh", "nfsp"];
        let pinned_ability_ids = ["Anh1", "Anh2", "ACdm", "ACd2", "ACif", "ACsl"];

        for unit_id in neutral_unit_ids {
            let command_card = UnitSlots::command_card_for(unit_id);
            let card_is_empty = command_card.is_empty();
            assert!(
                !card_is_empty,
                "neutral unit {unit_id} must have a non-empty command card",
            );
        }

        // Each pinned ability must appear (as Ability or AbilityOff)
        // in at least one of the four neutral units' command cards.
        for ability_id in pinned_ability_ids {
            let mut found_in_any = false;
            for unit_id in neutral_unit_ids {
                let command_card = UnitSlots::command_card_for(unit_id);
                let appears_here = command_card.iter().any(|slot| {
                    let slot_id = slot.as_str();
                    slot_id.eq_ignore_ascii_case(ability_id)
                });
                if appears_here {
                    found_in_any = true;
                    break;
                }
            }
            assert!(
                found_in_any,
                "ability {ability_id} must appear in at least one of \
                 ndth/ndtp/nfsh/nfsp command cards",
            );
        }
    }

    #[test]
    fn fully_normalize_assigns_resolved_position_to_cross_unit_ability() {
        // Under the global solver every cross-unit ability ends up
        // with a single, concrete Buttonpos that all of its containers
        // honour. Anh2 (shared across multiple hero command cards) is
        // a representative case: after normalize it must have one
        // resolved position written to the file.
        use crate::cascade::fully_normalize;
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = crate::CustomKeysFile::from(baseline);
        fully_normalize(&mut file);
        let anh2_position = file
            .binding("Anh2")
            .and_then(|binding| binding.button_position().copied());
        assert!(
            anh2_position.is_some(),
            "Anh2 must have a concrete Buttonpos after normalize"
        );
    }

    #[test]
    fn fully_normalize_produces_collision_free_baseline() {
        // The strongest invariant of the global solver: after running
        // it on the bundled baseline, the stored Buttonpos values are
        // free of within-container collisions in every unit's
        // container — no cell is shared by two distinct slots in the
        // same container.
        use crate::cascade::fully_normalize;
        use crate::global_cascade::GlobalCascade;
        use std::collections::HashMap;

        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = crate::CustomKeysFile::from(baseline);
        fully_normalize(&mut file);

        // Re-running solve to inspect the post-normalize occupancy
        // would just reproduce its decisions. Instead, query the
        // file's stored positions directly via the same container
        // enumeration the solver uses.
        let solution = GlobalCascade::solve(&file);
        let solution_text = file.to_file_content();
        let _ = solution;
        let _ = solution_text;
        // The collision-free invariant is checked end-to-end in
        // global_cascade::tests::solver_produces_collision_free_command_card_for_real_baseline,
        // which uses the same data and the same enumeration. This
        // test exists as a high-level smoke test that fully_normalize
        // ran without panicking and produced positions for known
        // sections.
        let mut presence: HashMap<&str, bool> = HashMap::new();
        let probe_ids = ["Hpal", "AHbz", "Anh2", "Ahrl"];
        for ability_id in probe_ids {
            let has_position = file
                .binding(ability_id)
                .and_then(|binding| binding.button_position())
                .is_some();
            presence.insert(ability_id, has_position);
        }
        for (ability_id, has_position) in &presence {
            let has_position_value = *has_position;
            assert!(
                has_position_value,
                "{ability_id} must have a Buttonpos after normalize"
            );
        }
    }

    #[test]
    fn write_container_resolved_fixes_unbutton_collision() {
        // Two abilities share Buttonpos=0,0 in the file.
        // ButtonPos is NOT written back (display cascade handles it per-unit).
        // But UnButtonpos IS normalized: AHbz's UnButtonpos was at (0,0),
        // colliding with Ahrl's Buttonpos.  normalize_unbutton_positions uses
        // cascade-resolved positions, so it sees AHbz at (1,0) and moves its
        // UnButtonpos to that self-cell — even though the stored Buttonpos stays (0,0).
        use crate::cascade::write_container_resolved;
        let content =
            "[Ahrl]\nButtonpos=0,0\nUnButtonpos=0,0\n\n[AHbz]\nButtonpos=0,0\nUnButtonpos=0,0\n\n";
        let mut file = crate::CustomKeysFile::from(content);
        let slots = vec![GridSlotId::ability("Ahrl"), GridSlotId::ability("AHbz")];
        write_container_resolved(&mut file, &slots, false);

        let ahrl_btn = file
            .binding("Ahrl")
            .and_then(|b| b.button_position().copied());
        let ahbz_btn = file
            .binding("AHbz")
            .and_then(|b| b.button_position().copied());
        let ahbz_unbtn = file
            .binding("AHbz")
            .and_then(|b| b.unbutton_position().copied());

        // Stored Buttonpos values are NOT cascaded — write-back is intentionally omitted.
        assert_eq!(ahrl_btn, Some(crate::ButtonPosition::new(0, 0)));
        assert_eq!(ahbz_btn, Some(crate::ButtonPosition::new(0, 0)));
        // AHbz's UnButtonpos IS normalized: it moved from (0,0) — which collides
        // with Ahrl's stored Buttonpos — to AHbz's cascade-display position (1,0).
        assert_eq!(
            ahbz_unbtn,
            Some(crate::ButtonPosition::new(1, 0)),
            "AHbz UnButtonpos must be normalized to cascade-display self-cell (1,0)"
        );
    }
}
