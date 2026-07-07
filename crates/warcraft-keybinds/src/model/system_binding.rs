use super::hotkey::Hotkey;
use std::fmt;
use warcraft_api::{SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId};

/// Binding for a system-level hotkey section.
/// Sections are identified by a class-discriminator field
/// (`GameCommand=1`, `CtrlGroupCommand=1`, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemBinding {
    hotkey: Hotkey,
    class: SystemKeybindClass,
    modifier: Option<SystemKeybindModifier>,
}

impl SystemBinding {
    pub fn new(
        hotkey: Hotkey,
        class: SystemKeybindClass,
        modifier: Option<SystemKeybindModifier>,
    ) -> Self {
        Self {
            hotkey,
            class,
            modifier,
        }
    }

    pub fn hotkey(&self) -> &Hotkey {
        &self.hotkey
    }

    pub fn class(&self) -> SystemKeybindClass {
        self.class
    }

    pub fn modifier(&self) -> Option<SystemKeybindModifier> {
        self.modifier
    }

    pub fn set_hotkey(&mut self, value: Hotkey) {
        self.hotkey = value;
    }

    pub(crate) fn write_section(
        &self,
        formatter: &mut fmt::Formatter<'_>,
        id: WarcraftObjectId,
    ) -> fmt::Result {
        let id_str = id.value();
        writeln!(formatter, "[{id_str}]")?;
        let hotkey = self.hotkey();
        writeln!(formatter, "Hotkey={hotkey}")?;
        let binding_class = self.class();
        let class_field = binding_class.ini_field();
        writeln!(formatter, "{class_field}")?;
        if let Some(modifier) = self.modifier()
            && let Some(modifier_text) = modifier.ini_str()
        {
            writeln!(formatter, "Modifier={modifier_text}")?;
        }
        writeln!(formatter)
    }
}
