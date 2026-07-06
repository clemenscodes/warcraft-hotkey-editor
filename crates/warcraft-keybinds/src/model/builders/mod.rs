use super::AbilityBinding;
use super::AbilityModifier;
use super::CommandBinding;
use super::GridCoordinate;
use super::Hotkey;
use super::SystemBinding;
use crate::custom_keys::CustomKeys;
use crate::identity::ability_id::AbilityId;
use warcraft_api::WarcraftObjectId;

#[derive(Debug, Default)]
pub struct AbilityBindingBuilder {
    hotkey: Option<Hotkey>,
    unhotkey: Option<Hotkey>,
    button_position: Option<GridCoordinate>,
    unbutton_position: Option<GridCoordinate>,
    research_hotkey: Option<Hotkey>,
    research_button_position: Option<GridCoordinate>,
    tip: Option<String>,
    research_tip: Option<String>,
    un_tip: Option<String>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    un_ubertip: Option<String>,
    icon: Option<String>,
    un_icon: Option<String>,
    modifier: Option<AbilityModifier>,
}

impl AbilityBindingBuilder {
    pub fn hotkey(mut self, hotkey: Hotkey) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    pub fn unhotkey(mut self, hotkey: Hotkey) -> Self {
        self.unhotkey = Some(hotkey);
        self
    }

    pub fn button_position(mut self, position: GridCoordinate) -> Self {
        self.button_position = Some(position);
        self
    }

    pub fn unbutton_position(mut self, position: GridCoordinate) -> Self {
        self.unbutton_position = Some(position);
        self
    }

    pub fn research_hotkey(mut self, hotkey: Hotkey) -> Self {
        self.research_hotkey = Some(hotkey);
        self
    }

    pub fn research_button_position(mut self, position: GridCoordinate) -> Self {
        self.research_button_position = Some(position);
        self
    }

    pub fn tip(mut self, text: impl Into<String>) -> Self {
        self.tip = Some(text.into());
        self
    }

    pub fn research_tip(mut self, text: impl Into<String>) -> Self {
        self.research_tip = Some(text.into());
        self
    }

    pub fn un_tip(mut self, text: impl Into<String>) -> Self {
        self.un_tip = Some(text.into());
        self
    }

    pub fn ubertip(mut self, text: impl Into<String>) -> Self {
        self.ubertip = Some(text.into());
        self
    }

    pub fn research_ubertip(mut self, text: impl Into<String>) -> Self {
        self.research_ubertip = Some(text.into());
        self
    }

    pub fn un_ubertip(mut self, text: impl Into<String>) -> Self {
        self.un_ubertip = Some(text.into());
        self
    }

    pub fn icon(mut self, path: impl Into<String>) -> Self {
        self.icon = Some(path.into());
        self
    }

    pub fn un_icon(mut self, path: impl Into<String>) -> Self {
        self.un_icon = Some(path.into());
        self
    }

    pub fn modifier(mut self, ability_modifier: AbilityModifier) -> Self {
        self.modifier = Some(ability_modifier);
        self
    }

    pub fn build(self) -> AbilityBinding {
        AbilityBinding::from(self)
    }
}

impl From<AbilityBindingBuilder> for AbilityBinding {
    fn from(builder: AbilityBindingBuilder) -> Self {
        let AbilityBindingBuilder {
            hotkey,
            unhotkey,
            button_position,
            unbutton_position,
            research_hotkey,
            research_button_position,
            tip,
            research_tip,
            un_tip,
            ubertip,
            research_ubertip,
            un_ubertip,
            icon,
            un_icon,
            modifier,
        } = builder;
        let mut binding = Self::default();
        binding.set_hotkey(hotkey);
        binding.set_unhotkey(unhotkey);
        binding.set_button_position(button_position);
        binding.set_unbutton_position(unbutton_position);
        binding.set_research_hotkey(research_hotkey);
        binding.set_research_button_position(research_button_position);
        binding.set_tip(tip);
        binding.set_research_tip(research_tip);
        binding.set_un_tip(un_tip);
        binding.set_ubertip(ubertip);
        binding.set_research_ubertip(research_ubertip);
        binding.set_un_ubertip(un_ubertip);
        binding.set_icon(icon);
        binding.set_un_icon(un_icon);
        binding.set_modifier(modifier);
        binding
    }
}

#[derive(Debug, Default)]
pub struct CommandBindingBuilder {
    hotkey: Option<Hotkey>,
    button_position: Option<GridCoordinate>,
    unbutton_position: Option<GridCoordinate>,
    tip: Option<String>,
    un_tip: Option<String>,
}

impl CommandBindingBuilder {
    pub fn hotkey(mut self, hotkey: Hotkey) -> Self {
        self.hotkey = Some(hotkey);
        self
    }

    pub fn button_position(mut self, position: GridCoordinate) -> Self {
        self.button_position = Some(position);
        self
    }

    pub fn unbutton_position(mut self, position: GridCoordinate) -> Self {
        self.unbutton_position = Some(position);
        self
    }

    pub fn tip(mut self, text: impl Into<String>) -> Self {
        self.tip = Some(text.into());
        self
    }

    pub fn un_tip(mut self, text: impl Into<String>) -> Self {
        self.un_tip = Some(text.into());
        self
    }

    pub fn build(self) -> CommandBinding {
        CommandBinding::from(self)
    }
}

impl From<CommandBindingBuilder> for CommandBinding {
    fn from(builder: CommandBindingBuilder) -> Self {
        let CommandBindingBuilder {
            hotkey,
            button_position,
            unbutton_position,
            tip,
            un_tip,
        } = builder;
        let mut binding = Self::default();
        binding.set_hotkey(hotkey);
        binding.set_button_position(button_position);
        binding.set_unbutton_position(unbutton_position);
        binding.set_tip(tip);
        binding.set_un_tip(un_tip);
        binding
    }
}

#[derive(Debug, Default)]
pub struct CustomKeysBuilder {
    file: CustomKeys,
}

impl CustomKeysBuilder {
    pub fn ability(mut self, id: impl Into<AbilityId>, binding: AbilityBinding) -> Self {
        self.file.put_ability(id, binding);
        self
    }

    pub fn command(mut self, name: impl Into<WarcraftObjectId>, binding: CommandBinding) -> Self {
        self.file.put_command(name, binding);
        self
    }

    pub fn system(mut self, id: impl Into<WarcraftObjectId>, binding: SystemBinding) -> Self {
        self.file.put_system(id, binding);
        self
    }

    pub fn build(self) -> CustomKeys {
        CustomKeys::from(self)
    }
}

impl From<CustomKeysBuilder> for CustomKeys {
    fn from(builder: CustomKeysBuilder) -> Self {
        builder.file
    }
}

#[cfg(test)]
mod tests;
