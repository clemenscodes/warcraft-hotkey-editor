use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use warcraft_api::{SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Hotkey {
    Letter(char),
    FunctionKey(u8),
    VirtualKey(u32),
    MultiLevel(Vec<Self>),
}

impl fmt::Display for Hotkey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Letter(character) => write!(formatter, "{character}"),
            Self::FunctionKey(number) => write!(formatter, "F{number}"),
            Self::VirtualKey(code) => write!(formatter, "{code}"),
            Self::MultiLevel(levels) => {
                let mut first = true;
                for level in levels {
                    if !first {
                        formatter.write_str(",")?;
                    }
                    write!(formatter, "{level}")?;
                    first = false;
                }
                Ok(())
            }
        }
    }
}

impl TryFrom<&str> for Hotkey {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, ()> {
        if text.is_empty() {
            return Err(());
        }
        if text.contains(',') {
            let levels: Result<Vec<Self>, ()> = text
                .split(',')
                .map(|segment| Self::try_from(segment.trim()))
                .collect();
            let level_vec = levels?;
            if level_vec.is_empty() {
                return Err(());
            }
            return Ok(Self::MultiLevel(level_vec));
        }
        let lowercase = text.to_ascii_lowercase();
        if let Some(rest) = lowercase.strip_prefix('f')
            && let Ok(number) = rest.parse::<u8>()
            && (1..=12).contains(&number)
        {
            return Ok(Self::FunctionKey(number));
        }
        let mut character_iter = text.chars();
        if let Some(character) = character_iter.next()
            && character_iter.next().is_none()
            && character.is_ascii_alphabetic()
        {
            return Ok(Self::Letter(character.to_ascii_uppercase()));
        }
        if let Ok(code) = text.parse::<u32>() {
            return Ok(Self::VirtualKey(code));
        }
        Err(())
    }
}

impl From<char> for Hotkey {
    fn from(character: char) -> Self {
        Self::Letter(character.to_ascii_uppercase())
    }
}

impl From<Hotkey> for String {
    fn from(hotkey: Hotkey) -> Self {
        hotkey.to_string()
    }
}

#[derive(Debug)]
pub struct ParseHotkeyError;

impl fmt::Display for ParseHotkeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid hotkey")
    }
}

impl std::error::Error for ParseHotkeyError {}

impl FromStr for Hotkey {
    type Err = ParseHotkeyError;

    fn from_str(text: &str) -> Result<Self, ParseHotkeyError> {
        Self::try_from(text).map_err(|()| ParseHotkeyError)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityModifier {
    Alt,
    Ctrl,
    CtrlOrAlt,
    Shift,
}

impl fmt::Display for AbilityModifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Alt => "Alt",
            Self::Ctrl => "Ctrl",
            Self::CtrlOrAlt => "Ctrl_or_Alt",
            Self::Shift => "Shift",
        };
        formatter.write_str(text)
    }
}

impl TryFrom<&str> for AbilityModifier {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, ()> {
        match text.to_ascii_lowercase().as_str() {
            "alt" => Ok(Self::Alt),
            "ctrl" => Ok(Self::Ctrl),
            "ctrl_or_alt" => Ok(Self::CtrlOrAlt),
            "shift" => Ok(Self::Shift),
            _ => Err(()),
        }
    }
}

impl From<AbilityModifier> for String {
    fn from(modifier: AbilityModifier) -> Self {
        modifier.to_string()
    }
}

#[derive(Debug)]
pub struct ParseAbilityModifierError;

impl fmt::Display for ParseAbilityModifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid ability modifier")
    }
}

impl std::error::Error for ParseAbilityModifierError {}

impl FromStr for AbilityModifier {
    type Err = ParseAbilityModifierError;

    fn from_str(text: &str) -> Result<Self, ParseAbilityModifierError> {
        Self::try_from(text).map_err(|()| ParseAbilityModifierError)
    }
}

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

impl fmt::Display for ButtonPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{},{}", self.column, self.row)
    }
}

#[derive(Debug)]
pub struct ParseButtonPositionError;

impl fmt::Display for ParseButtonPositionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid button position")
    }
}

impl std::error::Error for ParseButtonPositionError {}

impl FromStr for ButtonPosition {
    type Err = ParseButtonPositionError;

    fn from_str(text: &str) -> Result<Self, ParseButtonPositionError> {
        Self::try_from(text).map_err(|()| ParseButtonPositionError)
    }
}

/// Slot data for a single command-card position.
/// Shared by the primary (on) and alt (off/un) states of an ability.
#[derive(Default, Debug, Clone)]
struct AbilitySlotData {
    hotkey: Option<Hotkey>,
    button_position: Option<ButtonPosition>,
    tip: Option<String>,
    ubertip: Option<String>,
    icon: Option<String>,
}

/// Slot data for the research/upgrade button of an upgradeable ability.
#[derive(Default, Debug, Clone)]
struct ResearchSlotData {
    hotkey: Option<Hotkey>,
    button_position: Option<ButtonPosition>,
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

    pub fn button_position(&self) -> Option<&ButtonPosition> {
        self.primary.button_position.as_ref()
    }

    pub fn unbutton_position(&self) -> Option<&ButtonPosition> {
        self.alt.button_position.as_ref()
    }

    pub fn research_hotkey(&self) -> Option<&Hotkey> {
        self.research.hotkey.as_ref()
    }

    pub fn research_button_position(&self) -> Option<&ButtonPosition> {
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

    pub fn set_button_position(&mut self, value: Option<ButtonPosition>) {
        self.primary.button_position = value;
    }

    pub fn set_unbutton_position(&mut self, value: Option<ButtonPosition>) {
        self.alt.button_position = value;
    }

    pub fn set_research_hotkey(&mut self, value: Option<Hotkey>) {
        self.research.hotkey = value;
    }

    pub fn set_research_button_position(&mut self, value: Option<ButtonPosition>) {
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

    pub fn builder() -> crate::builder::AbilityBindingBuilder {
        crate::builder::AbilityBindingBuilder::new()
    }
}

#[derive(Default, Debug, Clone)]
pub struct CommandBinding {
    hotkey: Option<Hotkey>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    tip: Option<String>,
    un_tip: Option<String>,
}

impl CommandBinding {
    pub fn hotkey(&self) -> Option<&Hotkey> {
        self.hotkey.as_ref()
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

    pub fn set_hotkey(&mut self, value: Option<Hotkey>) {
        self.hotkey = value;
    }

    pub fn set_button_position(&mut self, value: Option<ButtonPosition>) {
        self.button_position = value;
    }

    pub fn set_unbutton_position(&mut self, value: Option<ButtonPosition>) {
        self.unbutton_position = value;
    }

    pub fn set_tip(&mut self, value: Option<String>) {
        self.tip = value;
    }

    pub fn set_un_tip(&mut self, value: Option<String>) {
        self.un_tip = value;
    }

    pub fn builder() -> crate::builder::CommandBindingBuilder {
        crate::builder::CommandBindingBuilder::new()
    }
}

/// Binding for a system-level hotkey section.
/// Sections are identified by a class-discriminator field
/// (`GameCommand=1`, `CtrlGroupCommand=1`, etc.).
#[derive(Debug, Clone)]
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

pub struct BindingEntry<'a> {
    id: WarcraftObjectId,
    binding: &'a AbilityBinding,
}

impl<'a> BindingEntry<'a> {
    pub(crate) fn new(id: WarcraftObjectId, binding: &'a AbilityBinding) -> Self {
        Self { id, binding }
    }

    pub fn id(&self) -> WarcraftObjectId {
        self.id
    }

    pub fn binding(&self) -> &'a AbilityBinding {
        self.binding
    }
}

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

/// The type of a CustomKeys.txt section, determined from the game database.
#[derive(Debug, Clone, Copy)]
pub(crate) enum SectionKind {
    Ability,
    Command,
    System(SystemKeybindClass),
}

/// Typed discriminator for INI field names found inside a CustomKeys.txt section.
/// `"icon"` and `"art"` both map to `Icon`; `"unart"` maps to `UnIcon`.
enum BindingFieldKey {
    Hotkey,
    Unhotkey,
    ButtonPos,
    UnButtonPos,
    ResearchHotkey,
    ResearchButtonPos,
    Tip,
    ResearchTip,
    UnTip,
    Ubertip,
    ResearchUbertip,
    UnUbertip,
    Icon,
    UnIcon,
    Modifier,
}

impl TryFrom<&str> for BindingFieldKey {
    type Error = ();

    fn try_from(key: &str) -> Result<Self, ()> {
        match key.to_ascii_lowercase().as_str() {
            "hotkey" => Ok(Self::Hotkey),
            "unhotkey" => Ok(Self::Unhotkey),
            "buttonpos" => Ok(Self::ButtonPos),
            "unbuttonpos" => Ok(Self::UnButtonPos),
            "researchhotkey" => Ok(Self::ResearchHotkey),
            "researchbuttonpos" => Ok(Self::ResearchButtonPos),
            "tip" => Ok(Self::Tip),
            "researchtip" => Ok(Self::ResearchTip),
            "untip" => Ok(Self::UnTip),
            "ubertip" => Ok(Self::Ubertip),
            "researchubertip" => Ok(Self::ResearchUbertip),
            "unubertip" => Ok(Self::UnUbertip),
            "icon" | "art" => Ok(Self::Icon),
            "unart" => Ok(Self::UnIcon),
            "modifier" => Ok(Self::Modifier),
            _ => Err(()),
        }
    }
}

/// Accumulates all fields of a section before converting to a [`WarcraftKeybinding`].
pub(crate) struct SectionAccumulator {
    kind: SectionKind,
    hotkey: Option<Hotkey>,
    unhotkey: Option<Hotkey>,
    button_position: Option<ButtonPosition>,
    unbutton_position: Option<ButtonPosition>,
    research_hotkey: Option<Hotkey>,
    research_button_position: Option<ButtonPosition>,
    tip: Option<String>,
    research_tip: Option<String>,
    un_tip: Option<String>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
    un_ubertip: Option<String>,
    icon: Option<String>,
    un_icon: Option<String>,
    modifier: Option<AbilityModifier>,
    system_modifier: Option<SystemKeybindModifier>,
}

impl SectionAccumulator {
    pub(crate) fn new(kind: SectionKind) -> Self {
        Self {
            kind,
            hotkey: None,
            unhotkey: None,
            button_position: None,
            unbutton_position: None,
            research_hotkey: None,
            research_button_position: None,
            tip: None,
            research_tip: None,
            un_tip: None,
            ubertip: None,
            research_ubertip: None,
            un_ubertip: None,
            icon: None,
            un_icon: None,
            modifier: None,
            system_modifier: None,
        }
    }

    pub(crate) fn apply(&mut self, key: &str, value: &str) {
        let Ok(field_key) = BindingFieldKey::try_from(key) else {
            return;
        };
        match field_key {
            BindingFieldKey::Hotkey if self.hotkey.is_none() => {
                self.hotkey = Hotkey::try_from(value).ok();
            }
            BindingFieldKey::Unhotkey if self.unhotkey.is_none() => {
                self.unhotkey = Hotkey::try_from(value).ok();
            }
            BindingFieldKey::ButtonPos if self.button_position.is_none() => {
                self.button_position = ButtonPosition::try_from(value).ok();
            }
            BindingFieldKey::UnButtonPos if self.unbutton_position.is_none() => {
                self.unbutton_position = ButtonPosition::try_from(value).ok();
            }
            BindingFieldKey::ResearchHotkey if self.research_hotkey.is_none() => {
                self.research_hotkey = Hotkey::try_from(value).ok();
            }
            BindingFieldKey::ResearchButtonPos if self.research_button_position.is_none() => {
                self.research_button_position = ButtonPosition::try_from(value).ok();
            }
            BindingFieldKey::Tip if self.tip.is_none() => {
                self.tip = Some(value.to_string());
            }
            BindingFieldKey::ResearchTip if self.research_tip.is_none() => {
                self.research_tip = Some(value.to_string());
            }
            BindingFieldKey::UnTip if self.un_tip.is_none() => {
                self.un_tip = Some(value.to_string());
            }
            BindingFieldKey::Ubertip if self.ubertip.is_none() => {
                self.ubertip = Some(value.to_string());
            }
            BindingFieldKey::ResearchUbertip if self.research_ubertip.is_none() => {
                self.research_ubertip = Some(value.to_string());
            }
            BindingFieldKey::UnUbertip if self.un_ubertip.is_none() => {
                self.un_ubertip = Some(value.to_string());
            }
            BindingFieldKey::Icon if !value.is_empty() && self.icon.is_none() => {
                self.icon = Some(value.to_string());
            }
            BindingFieldKey::UnIcon if !value.is_empty() && self.un_icon.is_none() => {
                self.un_icon = Some(value.to_string());
            }
            BindingFieldKey::Modifier => {
                if self.modifier.is_none() {
                    self.modifier = AbilityModifier::try_from(value).ok();
                }
                if self.system_modifier.is_none() {
                    self.system_modifier = SystemKeybindModifier::try_from(value).ok();
                }
            }
            _ => {}
        }
    }
}

impl From<SectionAccumulator> for WarcraftKeybinding {
    fn from(accumulator: SectionAccumulator) -> Self {
        match accumulator.kind {
            SectionKind::Command => {
                let command_binding = CommandBinding {
                    hotkey: accumulator.hotkey,
                    button_position: accumulator.button_position,
                    unbutton_position: accumulator.unbutton_position,
                    tip: accumulator.tip,
                    un_tip: accumulator.un_tip,
                };
                Self::Command(command_binding)
            }
            SectionKind::System(class) => {
                let missing_hotkey = Hotkey::VirtualKey(0);
                let hotkey = accumulator.hotkey.unwrap_or(missing_hotkey);
                let system_binding = SystemBinding {
                    hotkey,
                    class,
                    modifier: accumulator.system_modifier,
                };
                Self::System(system_binding)
            }
            SectionKind::Ability => {
                let primary_slot = AbilitySlotData {
                    hotkey: accumulator.hotkey,
                    button_position: accumulator.button_position,
                    tip: accumulator.tip,
                    ubertip: accumulator.ubertip,
                    icon: accumulator.icon,
                };
                let alt_slot = AbilitySlotData {
                    hotkey: accumulator.unhotkey,
                    button_position: accumulator.unbutton_position,
                    tip: accumulator.un_tip,
                    ubertip: accumulator.un_ubertip,
                    icon: accumulator.un_icon,
                };
                let research_slot = ResearchSlotData {
                    hotkey: accumulator.research_hotkey,
                    button_position: accumulator.research_button_position,
                    tip: accumulator.research_tip,
                    ubertip: accumulator.research_ubertip,
                };
                let ability_binding = AbilityBinding {
                    primary: primary_slot,
                    alt: alt_slot,
                    research: research_slot,
                    modifier: accumulator.modifier,
                };
                Self::Ability(ability_binding)
            }
        }
    }
}
