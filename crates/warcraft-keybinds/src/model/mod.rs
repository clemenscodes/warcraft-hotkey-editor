use crate::identity::ability_id::AbilityId;
use std::fmt;
use std::ops::Deref;
pub use warcraft_api::{ColumnIndex, GridCoordinate, RowIndex};

use warcraft_api::{SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId};

pub mod builders;
pub mod hotkey;
pub(crate) mod section;

pub use builders::{AbilityBindingBuilder, CommandBindingBuilder, CustomKeysBuilder};
pub use hotkey::{AbilityModifier, Hotkey, ParseAbilityModifierError, ParseHotkeyError};
pub(crate) use section::{SectionAccumulator, SectionResolution};

/// Slot data for a single command-card position.
/// Shared by the primary (on) and alt (off/un) states of an ability.
#[derive(Default, Debug, Clone)]
struct AbilitySlotData {
    hotkey: Option<Hotkey>,
    button_position: Option<GridCoordinate>,
    tip: Option<String>,
    ubertip: Option<String>,
    icon: Option<String>,
}

/// Slot data for the research/upgrade button of an upgradeable ability.
#[derive(Default, Debug, Clone)]
struct ResearchSlotData {
    hotkey: Option<Hotkey>,
    button_position: Option<GridCoordinate>,
    tip: Option<String>,
    ubertip: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct AbilityBinding {
    primary: AbilitySlotData,
    alt: AbilitySlotData,
    research: ResearchSlotData,
    modifier: Option<AbilityModifier>,
}

impl AbilityBinding {
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

#[derive(Default, Debug, Clone)]
pub struct CommandBinding {
    hotkey: Option<Hotkey>,
    button_position: Option<GridCoordinate>,
    unbutton_position: Option<GridCoordinate>,
    tip: Option<String>,
    un_tip: Option<String>,
}

impl CommandBinding {
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
        if let Self::Ability(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_ability_mut(&mut self) -> Option<&mut AbilityBinding> {
        if let Self::Ability(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_command(&self) -> Option<&CommandBinding> {
        if let Self::Command(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_command_mut(&mut self) -> Option<&mut CommandBinding> {
        if let Self::Command(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_system(&self) -> Option<&SystemBinding> {
        if let Self::System(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_system_mut(&mut self) -> Option<&mut SystemBinding> {
        if let Self::System(binding) = self {
            Some(binding)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BindingEntry<'a> {
    ability_id: AbilityId,
    binding: &'a AbilityBinding,
}

impl<'a> BindingEntry<'a> {
    pub(crate) fn new(ability_id: AbilityId, binding: &'a AbilityBinding) -> Self {
        Self {
            ability_id,
            binding,
        }
    }

    pub fn ability_id(&self) -> AbilityId {
        self.ability_id
    }

    pub fn binding(&self) -> &'a AbilityBinding {
        self.binding
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CommandEntry<'a> {
    name: WarcraftObjectId,
    binding: &'a CommandBinding,
}

impl<'a> CommandEntry<'a> {
    pub(crate) fn new(name: WarcraftObjectId, binding: &'a CommandBinding) -> Self {
        Self { name, binding }
    }

    pub fn name(&self) -> WarcraftObjectId {
        self.name
    }

    pub fn binding(&self) -> &'a CommandBinding {
        self.binding
    }
}

impl<'a> Deref for BindingEntry<'a> {
    type Target = AbilityBinding;

    fn deref(&self) -> &AbilityBinding {
        self.binding
    }
}

impl<'a> Deref for CommandEntry<'a> {
    type Target = CommandBinding;

    fn deref(&self) -> &CommandBinding {
        self.binding
    }
}

#[cfg(test)]
mod model_tests {
    use super::section::SectionKind;
    use super::*;
    use crate::identity::keycode::Letter;

    #[test]
    fn hotkey_letter_is_normalized_to_uppercase() {
        let hotkey = Hotkey::from('q');
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_try_from_empty_string_returns_err() {
        assert!(Hotkey::try_from("").is_err());
    }

    #[test]
    fn hotkey_try_from_letter_returns_uppercased() {
        let hotkey = Hotkey::try_from("w").unwrap();
        assert_eq!(hotkey, Hotkey::Letter('W'));
    }

    #[test]
    fn hotkey_try_from_function_key_case_insensitive() {
        let hotkey_lower = Hotkey::try_from("f3").unwrap();
        let hotkey_upper = Hotkey::try_from("F3").unwrap();
        assert_eq!(hotkey_lower, Hotkey::FunctionKey(3));
        assert_eq!(hotkey_upper, Hotkey::FunctionKey(3));
    }

    #[test]
    fn hotkey_try_from_virtual_key_numeric_string() {
        let hotkey = Hotkey::try_from("27").unwrap();
        assert_eq!(hotkey, Hotkey::VirtualKey(27));
    }

    #[test]
    fn hotkey_try_from_multi_level_comma_separated() {
        use crate::identity::hotkey_token::HotkeyToken;
        let hotkey = Hotkey::try_from("Q,W,E").unwrap();
        let expected = Hotkey::MultiLevel {
            tokens: [
                Some(HotkeyToken::Letter(Letter::Q)),
                Some(HotkeyToken::Letter(Letter::W)),
                Some(HotkeyToken::Letter(Letter::E)),
                None,
            ],
        };
        assert_eq!(hotkey, expected);
    }

    #[test]
    fn hotkey_display_letter() {
        let hotkey = Hotkey::Letter('A');
        assert_eq!(hotkey.to_string(), "A");
    }

    #[test]
    fn hotkey_display_function_key() {
        let hotkey = Hotkey::FunctionKey(7);
        assert_eq!(hotkey.to_string(), "F7");
    }

    #[test]
    fn hotkey_display_virtual_key() {
        let hotkey = Hotkey::VirtualKey(9);
        assert_eq!(hotkey.to_string(), "9");
    }

    #[test]
    fn hotkey_display_multi_level() {
        use crate::identity::hotkey_token::HotkeyToken;
        let hotkey = Hotkey::MultiLevel {
            tokens: [
                Some(HotkeyToken::Letter(Letter::Q)),
                Some(HotkeyToken::Letter(Letter::Q)),
                None,
                None,
            ],
        };
        assert_eq!(hotkey.to_string(), "Q,Q");
    }

    #[test]
    fn hotkey_from_string_roundtrip() {
        let original = Hotkey::FunctionKey(12);
        let string_form: String = original.into();
        let reparsed = Hotkey::try_from(string_form.as_str()).unwrap();
        assert_eq!(original, reparsed);
    }

    #[test]
    fn button_position_try_from_valid_string() {
        let position = GridCoordinate::try_from("2,1").unwrap();
        assert_eq!(position.column(), ColumnIndex::Two);
        assert_eq!(position.row(), RowIndex::One);
    }

    #[test]
    fn button_position_try_from_invalid_returns_err() {
        assert!(GridCoordinate::try_from("notanumber").is_err());
        assert!(GridCoordinate::try_from("1").is_err());
        assert!(GridCoordinate::try_from("").is_err());
    }

    #[test]
    fn button_position_display_roundtrip() {
        let position = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        let displayed = position.to_string();
        let reparsed = GridCoordinate::try_from(displayed.as_str()).unwrap();
        assert_eq!(position, reparsed);
    }

    #[test]
    fn ability_modifier_display_variants() {
        assert_eq!(AbilityModifier::Alt.to_string(), "Alt");
        assert_eq!(AbilityModifier::Ctrl.to_string(), "Ctrl");
        assert_eq!(AbilityModifier::CtrlOrAlt.to_string(), "Ctrl_or_Alt");
        assert_eq!(AbilityModifier::Shift.to_string(), "Shift");
    }

    #[test]
    fn ability_modifier_try_from_case_insensitive() {
        assert_eq!(
            AbilityModifier::try_from("ALT").unwrap(),
            AbilityModifier::Alt
        );
        assert_eq!(
            AbilityModifier::try_from("shift").unwrap(),
            AbilityModifier::Shift
        );
    }

    #[test]
    fn section_resolution_resolves_known_ability() {
        let resolution = SectionResolution::from_section_id("Hpal").unwrap();
        assert!(matches!(resolution.kind(), SectionKind::Ability));
    }

    #[test]
    fn section_resolution_resolves_known_command() {
        let resolution = SectionResolution::from_section_id("CmdAttack").unwrap();
        assert!(matches!(resolution.kind(), SectionKind::Command));
    }

    #[test]
    fn section_resolution_returns_none_for_unknown_id() {
        let result = SectionResolution::from_section_id("ZZZUnknown");
        assert!(result.is_none());
    }

    #[test]
    fn section_resolution_is_case_insensitive() {
        let lower = SectionResolution::from_section_id("hpal");
        let upper = SectionResolution::from_section_id("HPAL");
        assert!(lower.is_some());
        assert!(upper.is_some());
        let lower_unwrapped = lower.unwrap();
        let upper_unwrapped = upper.unwrap();
        assert_eq!(
            lower_unwrapped.canonical_id(),
            upper_unwrapped.canonical_id()
        );
    }
}
