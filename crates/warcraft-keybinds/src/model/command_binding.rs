use super::builders::CommandBindingBuilder;
use super::hotkey::Hotkey;
use std::fmt;
use warcraft_api::{GridCoordinate, WarcraftObjectId};

#[derive(Default, Debug, Clone)]
pub struct CommandBinding {
    hotkey: Option<Hotkey>,
    button_position: Option<GridCoordinate>,
    unbutton_position: Option<GridCoordinate>,
    tip: Option<String>,
    un_tip: Option<String>,
}

impl CommandBinding {
    /// Assemble a command binding from its parsed fields (the parser's entry point);
    /// keeps `CommandBinding`'s own fields private.
    pub(crate) fn from_parts(
        hotkey: Option<Hotkey>,
        button_position: Option<GridCoordinate>,
        unbutton_position: Option<GridCoordinate>,
        tip: Option<String>,
        un_tip: Option<String>,
    ) -> Self {
        Self {
            hotkey,
            button_position,
            unbutton_position,
            tip,
            un_tip,
        }
    }

    pub fn hotkey(&self) -> Option<&Hotkey> {
        self.hotkey.as_ref()
    }

    pub fn button_position(&self) -> Option<&GridCoordinate> {
        self.button_position.as_ref()
    }

    pub fn unbutton_position(&self) -> Option<&GridCoordinate> {
        self.unbutton_position.as_ref()
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn un_tip(&self) -> Option<&str> {
        self.un_tip.as_deref()
    }

    pub fn set_hotkey(&mut self, value: Option<Hotkey>) {
        self.hotkey = value;
    }

    pub fn set_button_position(&mut self, value: Option<GridCoordinate>) {
        self.button_position = value;
    }

    pub fn set_unbutton_position(&mut self, value: Option<GridCoordinate>) {
        self.unbutton_position = value;
    }

    pub fn set_tip(&mut self, value: Option<String>) {
        self.tip = value;
    }

    pub fn set_un_tip(&mut self, value: Option<String>) {
        self.un_tip = value;
    }

    pub fn builder() -> CommandBindingBuilder {
        CommandBindingBuilder::default()
    }

    pub(crate) fn write_section(
        &self,
        formatter: &mut fmt::Formatter<'_>,
        id: WarcraftObjectId,
    ) -> fmt::Result {
        let id_str = id.value();
        writeln!(formatter, "[{id_str}]")?;
        if let Some(hotkey) = self.hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Hotkey={hotkey_string}")?;
        }
        if let Some(position) = self.button_position() {
            let position_string = position.to_string();
            writeln!(formatter, "Buttonpos={position_string}")?;
        }
        if let Some(position) = self.unbutton_position() {
            let position_string = position.to_string();
            writeln!(formatter, "Unbuttonpos={position_string}")?;
        }
        if let Some(value) = self.tip() {
            writeln!(formatter, "Tip={value}")?;
        }
        if let Some(value) = self.un_tip() {
            writeln!(formatter, "UnTip={value}")?;
        }
        writeln!(formatter)
    }
}
