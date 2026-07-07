use super::builders::AbilityBindingBuilder;
use super::hotkey::{AbilityModifier, Hotkey};
use std::fmt;
use warcraft_api::{GridCoordinate, WarcraftObjectId};

/// Slot data for a single command-card position.
/// Shared by the primary (on) and alt (off/un) states of an ability. Crate-internal:
/// the parser (`section`) fills it field-wise, `AbilityBinding` reads it via getters.
#[derive(Default, Debug, Clone)]
pub(crate) struct AbilitySlotData {
    pub(crate) hotkey: Option<Hotkey>,
    pub(crate) button_position: Option<GridCoordinate>,
    pub(crate) tip: Option<String>,
    pub(crate) ubertip: Option<String>,
    pub(crate) icon: Option<String>,
}

/// Slot data for the research/upgrade button of an upgradeable ability. Crate-internal.
#[derive(Default, Debug, Clone)]
pub(crate) struct ResearchSlotData {
    pub(crate) hotkey: Option<Hotkey>,
    pub(crate) button_position: Option<GridCoordinate>,
    pub(crate) tip: Option<String>,
    pub(crate) ubertip: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct AbilityBinding {
    primary: AbilitySlotData,
    alt: AbilitySlotData,
    research: ResearchSlotData,
    modifier: Option<AbilityModifier>,
}

impl AbilityBinding {
    /// Assemble a binding from its parsed slots. The parser (`section`) builds the
    /// slot data; this keeps `AbilityBinding`'s own fields private.
    pub(crate) fn from_parts(
        primary: AbilitySlotData,
        alt: AbilitySlotData,
        research: ResearchSlotData,
        modifier: Option<AbilityModifier>,
    ) -> Self {
        Self {
            primary,
            alt,
            research,
            modifier,
        }
    }

    pub fn hotkey(&self) -> Option<&Hotkey> {
        self.primary.hotkey.as_ref()
    }

    pub fn unhotkey(&self) -> Option<&Hotkey> {
        self.alt.hotkey.as_ref()
    }

    pub fn button_position(&self) -> Option<&GridCoordinate> {
        self.primary.button_position.as_ref()
    }

    pub fn unbutton_position(&self) -> Option<&GridCoordinate> {
        self.alt.button_position.as_ref()
    }

    pub fn research_hotkey(&self) -> Option<&Hotkey> {
        self.research.hotkey.as_ref()
    }

    pub fn research_button_position(&self) -> Option<&GridCoordinate> {
        self.research.button_position.as_ref()
    }

    pub fn tip(&self) -> Option<&str> {
        self.primary.tip.as_deref()
    }

    pub fn research_tip(&self) -> Option<&str> {
        self.research.tip.as_deref()
    }

    pub fn un_tip(&self) -> Option<&str> {
        self.alt.tip.as_deref()
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.primary.ubertip.as_deref()
    }

    pub fn research_ubertip(&self) -> Option<&str> {
        self.research.ubertip.as_deref()
    }

    pub fn un_ubertip(&self) -> Option<&str> {
        self.alt.ubertip.as_deref()
    }

    pub fn icon(&self) -> Option<&str> {
        self.primary.icon.as_deref()
    }

    pub fn un_icon(&self) -> Option<&str> {
        self.alt.icon.as_deref()
    }

    pub fn modifier(&self) -> Option<AbilityModifier> {
        self.modifier
    }

    pub fn set_hotkey(&mut self, value: Option<Hotkey>) {
        self.primary.hotkey = value;
    }

    pub fn set_unhotkey(&mut self, value: Option<Hotkey>) {
        self.alt.hotkey = value;
    }

    pub fn set_button_position(&mut self, value: Option<GridCoordinate>) {
        self.primary.button_position = value;
    }

    pub fn set_unbutton_position(&mut self, value: Option<GridCoordinate>) {
        self.alt.button_position = value;
    }

    pub fn set_research_hotkey(&mut self, value: Option<Hotkey>) {
        self.research.hotkey = value;
    }

    pub fn set_research_button_position(&mut self, value: Option<GridCoordinate>) {
        self.research.button_position = value;
    }

    pub fn set_tip(&mut self, value: Option<String>) {
        self.primary.tip = value;
    }

    pub fn set_research_tip(&mut self, value: Option<String>) {
        self.research.tip = value;
    }

    pub fn set_un_tip(&mut self, value: Option<String>) {
        self.alt.tip = value;
    }

    pub fn set_ubertip(&mut self, value: Option<String>) {
        self.primary.ubertip = value;
    }

    pub fn set_research_ubertip(&mut self, value: Option<String>) {
        self.research.ubertip = value;
    }

    pub fn set_un_ubertip(&mut self, value: Option<String>) {
        self.alt.ubertip = value;
    }

    pub fn set_icon(&mut self, value: Option<String>) {
        self.primary.icon = value;
    }

    pub fn set_un_icon(&mut self, value: Option<String>) {
        self.alt.icon = value;
    }

    pub fn set_modifier(&mut self, value: Option<AbilityModifier>) {
        self.modifier = value;
    }

    pub fn builder() -> AbilityBindingBuilder {
        AbilityBindingBuilder::default()
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
        if let Some(hotkey) = self.unhotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Unhotkey={hotkey_string}")?;
        }
        if let Some(position) = self.button_position() {
            let position_string = position.to_string();
            writeln!(formatter, "Buttonpos={position_string}")?;
        }
        if let Some(position) = self.unbutton_position() {
            let position_string = position.to_string();
            writeln!(formatter, "Unbuttonpos={position_string}")?;
        }
        if let Some(hotkey) = self.research_hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Researchhotkey={hotkey_string}")?;
        }
        if let Some(position) = self.research_button_position() {
            let position_string = position.to_string();
            writeln!(formatter, "Researchbuttonpos={position_string}")?;
        }
        if let Some(value) = self.tip() {
            writeln!(formatter, "Tip={value}")?;
        }
        if let Some(value) = self.research_tip() {
            writeln!(formatter, "Researchtip={value}")?;
        }
        if let Some(value) = self.un_tip() {
            writeln!(formatter, "UnTip={value}")?;
        }
        if let Some(value) = self.ubertip() {
            writeln!(formatter, "Ubertip={value}")?;
        }
        if let Some(value) = self.research_ubertip() {
            writeln!(formatter, "Researchubertip={value}")?;
        }
        if let Some(value) = self.un_ubertip() {
            writeln!(formatter, "Unubertip={value}")?;
        }
        if let Some(value) = self.icon() {
            writeln!(formatter, "Icon={value}")?;
        }
        if let Some(modifier) = self.modifier() {
            let modifier_string = modifier.to_string();
            writeln!(formatter, "Modifier={modifier_string}")?;
        }
        writeln!(formatter)
    }
}
