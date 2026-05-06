use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

use warcraft_api::WarcraftObjectId;

use crate::model::{
    AbilityBinding, BindingEntry, CommandBinding, CommandEntry, SystemBinding, WarcraftKeybinding,
};

pub struct CustomKeysFile {
    entries: BTreeMap<String, WarcraftKeybinding>,
}

impl CustomKeysFile {
    pub(crate) fn from_parts(entries: BTreeMap<String, WarcraftKeybinding>) -> Self {
        Self { entries }
    }

    pub fn get(&self, id: WarcraftObjectId) -> Option<&WarcraftKeybinding> {
        let key = id.value().to_lowercase();
        self.entries.get(&key)
    }

    pub fn get_mut(&mut self, id: WarcraftObjectId) -> Option<&mut WarcraftKeybinding> {
        let key = id.value().to_lowercase();
        self.entries.get_mut(&key)
    }

    pub fn set(&mut self, id: WarcraftObjectId, binding: WarcraftKeybinding) {
        let key = id.value().to_lowercase();
        self.entries.insert(key, binding);
    }

    pub fn binding(&self, id: &str) -> Option<&AbilityBinding> {
        let key = id.to_lowercase();
        self.entries.get(&key)?.as_ability()
    }

    pub fn binding_mut(&mut self, id: &str) -> Option<&mut AbilityBinding> {
        let key = id.to_lowercase();
        self.entries.get_mut(&key)?.as_ability_mut()
    }

    pub fn binding_or_default_mut(&mut self, id: &str) -> Option<&mut AbilityBinding> {
        let key = id.to_lowercase();
        if !matches!(self.entries.get(&key), Some(WarcraftKeybinding::Ability(_))) {
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
        self.entries.iter().filter_map(|(id, binding)| {
            binding
                .as_ability()
                .map(|ability| BindingEntry::new(id, ability))
        })
    }

    pub fn command(&self, name: &str) -> Option<&CommandBinding> {
        let key = name.to_lowercase();
        self.entries.get(&key)?.as_command()
    }

    pub fn command_mut(&mut self, name: &str) -> Option<&mut CommandBinding> {
        let key = name.to_lowercase();
        self.entries.get_mut(&key)?.as_command_mut()
    }

    pub fn command_or_default_mut(&mut self, name: &str) -> Option<&mut CommandBinding> {
        let key = name.to_lowercase();
        if !matches!(self.entries.get(&key), Some(WarcraftKeybinding::Command(_))) {
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
        self.entries.iter().filter_map(|(name, binding)| {
            binding
                .as_command()
                .map(|command| CommandEntry::new(name, command))
        })
    }

    pub fn system(&self, id: &str) -> Option<&SystemBinding> {
        let key = id.to_lowercase();
        self.entries.get(&key)?.as_system()
    }

    pub fn system_mut(&mut self, id: &str) -> Option<&mut SystemBinding> {
        let key = id.to_lowercase();
        self.entries.get_mut(&key)?.as_system_mut()
    }

    pub fn builder() -> crate::builder::CustomKeysFileBuilder {
        crate::builder::CustomKeysFileBuilder::new()
    }

    pub fn put_ability(&mut self, id: &str, binding: AbilityBinding) {
        let key = id.to_lowercase();
        self.entries
            .insert(key, WarcraftKeybinding::Ability(binding));
    }

    pub fn put_command(&mut self, name: &str, binding: CommandBinding) {
        let key = name.to_lowercase();
        self.entries
            .insert(key, WarcraftKeybinding::Command(binding));
    }

    pub fn put_system(&mut self, id: &str, binding: SystemBinding) {
        let key = id.to_lowercase();
        self.entries
            .insert(key, WarcraftKeybinding::System(binding));
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
        for (id, entry) in &self.entries {
            match entry {
                WarcraftKeybinding::Ability(binding) => {
                    Self::format_ability_section(&mut output, id, binding);
                }
                WarcraftKeybinding::Command(binding) => {
                    Self::format_command_section(&mut output, id, binding);
                }
                WarcraftKeybinding::System(binding) => {
                    Self::format_system_section(&mut output, id, binding);
                }
            }
        }
        output
    }

    fn format_ability_section(output: &mut String, id: &str, binding: &AbilityBinding) {
        output.push('[');
        output.push_str(id);
        output.push_str("]\n");
        if let Some(hotkey) = binding.hotkey() {
            let hotkey_string = hotkey.to_string();
            output.push_str("Hotkey=");
            output.push_str(&hotkey_string);
            output.push('\n');
        }
        if let Some(hotkey) = binding.unhotkey() {
            let hotkey_string = hotkey.to_string();
            output.push_str("Unhotkey=");
            output.push_str(&hotkey_string);
            output.push('\n');
        }
        if let Some(position) = binding.button_position() {
            let column = position.column();
            let row = position.row();
            let buttonpos_line = format!("Buttonpos={column},{row}\n");
            output.push_str(&buttonpos_line);
        }
        if let Some(position) = binding.unbutton_position() {
            let column = position.column();
            let row = position.row();
            let unbuttonpos_line = format!("Unbuttonpos={column},{row}\n");
            output.push_str(&unbuttonpos_line);
        }
        if let Some(hotkey) = binding.research_hotkey() {
            let hotkey_string = hotkey.to_string();
            output.push_str("Researchhotkey=");
            output.push_str(&hotkey_string);
            output.push('\n');
        }
        if let Some(position) = binding.research_button_position() {
            let column = position.column();
            let row = position.row();
            let researchbuttonpos_line = format!("Researchbuttonpos={column},{row}\n");
            output.push_str(&researchbuttonpos_line);
        }
        if let Some(value) = binding.tip() {
            output.push_str("Tip=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(value) = binding.research_tip() {
            output.push_str("Researchtip=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(value) = binding.un_tip() {
            output.push_str("UnTip=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(value) = binding.ubertip() {
            output.push_str("Ubertip=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(value) = binding.research_ubertip() {
            output.push_str("Researchubertip=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(value) = binding.un_ubertip() {
            output.push_str("Unubertip=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(value) = binding.icon() {
            output.push_str("Icon=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(modifier) = binding.modifier() {
            let modifier_string = modifier.to_string();
            output.push_str("Modifier=");
            output.push_str(&modifier_string);
            output.push('\n');
        }
        output.push('\n');
    }

    fn format_command_section(output: &mut String, name: &str, binding: &CommandBinding) {
        output.push('[');
        output.push_str(name);
        output.push_str("]\n");
        if let Some(hotkey) = binding.hotkey() {
            let hotkey_string = hotkey.to_string();
            output.push_str("Hotkey=");
            output.push_str(&hotkey_string);
            output.push('\n');
        }
        if let Some(position) = binding.button_position() {
            let column = position.column();
            let row = position.row();
            let buttonpos_line = format!("Buttonpos={column},{row}\n");
            output.push_str(&buttonpos_line);
        }
        if let Some(position) = binding.unbutton_position() {
            let column = position.column();
            let row = position.row();
            let unbuttonpos_line = format!("Unbuttonpos={column},{row}\n");
            output.push_str(&unbuttonpos_line);
        }
        if let Some(value) = binding.tip() {
            output.push_str("Tip=");
            output.push_str(value);
            output.push('\n');
        }
        if let Some(value) = binding.un_tip() {
            output.push_str("UnTip=");
            output.push_str(value);
            output.push('\n');
        }
        output.push('\n');
    }

    fn format_system_section(output: &mut String, id: &str, binding: &SystemBinding) {
        output.push('[');
        output.push_str(id);
        output.push_str("]\n");
        let hotkey = binding.hotkey();
        let hotkey_line = format!("Hotkey={hotkey}\n");
        output.push_str(&hotkey_line);
        let class_field = binding.class().ini_field();
        output.push_str(class_field);
        output.push('\n');
        if let Some(modifier) = binding.modifier()
            && let Some(modifier_text) = modifier.ini_str()
        {
            output.push_str("Modifier=");
            output.push_str(modifier_text);
            output.push('\n');
        }
        output.push('\n');
    }
}

fn home_directory() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    } else {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}
