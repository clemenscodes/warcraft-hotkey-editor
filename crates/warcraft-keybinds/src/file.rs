use std::collections::BTreeMap;
use std::fmt;

use crate::model::{
    AbilityBinding, BindingEntry, CommandBinding, CommandEntry, SystemBinding, WarcraftKeybinding,
};

#[derive(Clone)]
pub struct CustomKeysFile {
    entries: BTreeMap<String, WarcraftKeybinding>,
}

impl CustomKeysFile {
    pub(crate) fn from_parts(entries: BTreeMap<String, WarcraftKeybinding>) -> Self {
        Self { entries }
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
        let entry = self
            .entries
            .entry(key)
            .or_insert_with(|| WarcraftKeybinding::Ability(AbilityBinding::default()));
        entry.as_ability_mut()
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
        let entry = self
            .entries
            .entry(key)
            .or_insert_with(|| WarcraftKeybinding::Command(CommandBinding::default()));
        entry.as_command_mut()
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

    fn write_ability_section(
        formatter: &mut fmt::Formatter<'_>,
        id: &str,
        binding: &AbilityBinding,
    ) -> fmt::Result {
        writeln!(formatter, "[{id}]")?;
        if let Some(hotkey) = binding.hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Hotkey={hotkey_string}")?;
        }
        if let Some(hotkey) = binding.unhotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Unhotkey={hotkey_string}")?;
        }
        if let Some(position) = binding.button_position() {
            writeln!(formatter, "Buttonpos={position}")?;
        }
        if let Some(position) = binding.unbutton_position() {
            writeln!(formatter, "Unbuttonpos={position}")?;
        }
        if let Some(hotkey) = binding.research_hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Researchhotkey={hotkey_string}")?;
        }
        if let Some(position) = binding.research_button_position() {
            writeln!(formatter, "Researchbuttonpos={position}")?;
        }
        if let Some(value) = binding.tip() {
            writeln!(formatter, "Tip={value}")?;
        }
        if let Some(value) = binding.research_tip() {
            writeln!(formatter, "Researchtip={value}")?;
        }
        if let Some(value) = binding.un_tip() {
            writeln!(formatter, "UnTip={value}")?;
        }
        if let Some(value) = binding.ubertip() {
            writeln!(formatter, "Ubertip={value}")?;
        }
        if let Some(value) = binding.research_ubertip() {
            writeln!(formatter, "Researchubertip={value}")?;
        }
        if let Some(value) = binding.un_ubertip() {
            writeln!(formatter, "Unubertip={value}")?;
        }
        if let Some(value) = binding.icon() {
            writeln!(formatter, "Icon={value}")?;
        }
        if let Some(modifier) = binding.modifier() {
            let modifier_string = modifier.to_string();
            writeln!(formatter, "Modifier={modifier_string}")?;
        }
        writeln!(formatter)
    }

    fn write_command_section(
        formatter: &mut fmt::Formatter<'_>,
        id: &str,
        binding: &CommandBinding,
    ) -> fmt::Result {
        writeln!(formatter, "[{id}]")?;
        if let Some(hotkey) = binding.hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Hotkey={hotkey_string}")?;
        }
        if let Some(position) = binding.button_position() {
            writeln!(formatter, "Buttonpos={position}")?;
        }
        if let Some(position) = binding.unbutton_position() {
            writeln!(formatter, "Unbuttonpos={position}")?;
        }
        if let Some(value) = binding.tip() {
            writeln!(formatter, "Tip={value}")?;
        }
        if let Some(value) = binding.un_tip() {
            writeln!(formatter, "UnTip={value}")?;
        }
        writeln!(formatter)
    }

    fn write_system_section(
        formatter: &mut fmt::Formatter<'_>,
        id: &str,
        binding: &SystemBinding,
    ) -> fmt::Result {
        writeln!(formatter, "[{id}]")?;
        let hotkey = binding.hotkey();
        writeln!(formatter, "Hotkey={hotkey}")?;
        let binding_class = binding.class();
        let class_field = binding_class.ini_field();
        writeln!(formatter, "{class_field}")?;
        if let Some(modifier) = binding.modifier()
            && let Some(modifier_text) = modifier.ini_str()
        {
            writeln!(formatter, "Modifier={modifier_text}")?;
        }
        writeln!(formatter)
    }
}

impl fmt::Display for CustomKeysFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (id, entry) in &self.entries {
            match entry {
                WarcraftKeybinding::Ability(binding) => {
                    Self::write_ability_section(formatter, id, binding)?;
                }
                WarcraftKeybinding::Command(binding) => {
                    Self::write_command_section(formatter, id, binding)?;
                }
                WarcraftKeybinding::System(binding) => {
                    Self::write_system_section(formatter, id, binding)?;
                }
            }
        }
        Ok(())
    }
}

impl IntoIterator for CustomKeysFile {
    type Item = (String, WarcraftKeybinding);
    type IntoIter = std::collections::btree_map::IntoIter<String, WarcraftKeybinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl Extend<(String, WarcraftKeybinding)> for CustomKeysFile {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (String, WarcraftKeybinding)>,
    {
        for (key, binding) in iter {
            match binding {
                WarcraftKeybinding::Ability(source_binding) => {
                    if self.system(&key).is_some() {
                        continue;
                    }
                    let Some(target_binding) = self.binding_or_default_mut(&key) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = hotkey.clone();
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(hotkey) = source_binding.research_hotkey() {
                        let hotkey_clone = hotkey.clone();
                        target_binding.set_research_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.research_button_position().copied() {
                        target_binding.set_research_button_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.research_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_research_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                    if let Some(icon) = source_binding.icon() {
                        let icon_string = icon.to_string();
                        target_binding.set_icon(Some(icon_string));
                    }
                }
                WarcraftKeybinding::Command(source_binding) => {
                    let Some(target_binding) = self.command_or_default_mut(&key) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = hotkey.clone();
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                }
                WarcraftKeybinding::System(_) => {}
            }
        }
    }
}
