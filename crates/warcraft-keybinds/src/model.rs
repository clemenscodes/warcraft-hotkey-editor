use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
pub use warcraft_api::{ColumnIndex, GridCoordinate, RowIndex};
use warcraft_api::{
    SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId, WarcraftObjectKind,
};
use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};

use crate::custom_keys::CustomKeys;
use crate::identity::ability_id::AbilityId;
use crate::identity::hotkey_token::HotkeyToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hotkey {
    Letter(char),
    FunctionKey(u8),
    VirtualKey(u32),
    MultiLevel { tokens: [Option<HotkeyToken>; 4] },
}

impl fmt::Display for Hotkey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Letter(character) => write!(formatter, "{character}"),
            Self::FunctionKey(number) => write!(formatter, "F{number}"),
            Self::VirtualKey(code) => write!(formatter, "{code}"),
            Self::MultiLevel { tokens } => {
                let mut first = true;
                for token in tokens.iter().flatten() {
                    if !first {
                        formatter.write_str(",")?;
                    }
                    write!(formatter, "{token}")?;
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
            let mut tokens: [Option<HotkeyToken>; 4] = [None; 4];
            for (index, segment) in text.split(',').enumerate() {
                if index >= 4 {
                    return Err(());
                }
                tokens[index] = Some(HotkeyToken::try_from(segment.trim()).map_err(|_| ())?);
            }
            return Ok(Self::MultiLevel { tokens });
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

impl Hotkey {
    pub fn first_token(&self) -> Option<HotkeyToken> {
        match self {
            Self::MultiLevel { tokens } => tokens.iter().flatten().next().copied(),
            other => HotkeyToken::try_from(other).ok(),
        }
    }

    pub fn level_count(&self) -> usize {
        match self {
            Self::MultiLevel { tokens } => tokens.iter().flatten().count(),
            _ => 1,
        }
    }

    pub fn replicated(token: HotkeyToken, count: usize) -> Self {
        let clamped_count = count.clamp(1, 4);
        if clamped_count == 1 {
            Self::from(token)
        } else {
            let mut tokens: [Option<HotkeyToken>; 4] = [None; 4];
            for slot in tokens.iter_mut().take(clamped_count) {
                *slot = Some(token);
            }
            Self::MultiLevel { tokens }
        }
    }

    pub fn accepts_grid_letter(&self) -> bool {
        let Some(token) = self.first_token() else {
            return true;
        };
        char::try_from(token).is_ok()
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

/// Resolved section identity: the canonical database ID and the binding kind.
/// Returned by `SectionResolution::from_section_id`; replaces a raw tuple.
pub(crate) struct SectionResolution {
    pub(crate) canonical_id: WarcraftObjectId,
    pub(crate) kind: SectionKind,
}

impl SectionResolution {
    /// Look up `section_id` in the game database and system-keybind table.
    /// Returns `None` for unknown section IDs.
    pub(crate) fn from_section_id(section_id: &str) -> Option<Self> {
        if let Some((canonical_id, warcraft_object)) = WARCRAFT_DATABASE.by_id_and_key(section_id) {
            let section_kind = match warcraft_object.kind() {
                WarcraftObjectKind::Command => SectionKind::Command,
                _ => SectionKind::Ability,
            };
            return Some(Self {
                canonical_id,
                kind: section_kind,
            });
        }
        let lowercase_id = section_id.to_ascii_lowercase();
        if let Some(system_keybind) = WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .find(|system_keybind| system_keybind.section_id().to_ascii_lowercase() == lowercase_id)
        {
            let section_id_str = system_keybind.section_id();
            let canonical_id = WarcraftObjectId::new(section_id_str);
            let system_class = system_keybind.class();
            return Some(Self {
                canonical_id,
                kind: SectionKind::System(system_class),
            });
        }
        None
    }
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
                self.button_position = GridCoordinate::try_from(value).ok();
            }
            BindingFieldKey::UnButtonPos if self.unbutton_position.is_none() => {
                self.unbutton_position = GridCoordinate::try_from(value).ok();
            }
            BindingFieldKey::ResearchHotkey if self.research_hotkey.is_none() => {
                self.research_hotkey = Hotkey::try_from(value).ok();
            }
            BindingFieldKey::ResearchButtonPos if self.research_button_position.is_none() => {
                self.research_button_position = GridCoordinate::try_from(value).ok();
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

#[cfg(test)]
mod model_tests {
    use super::*;

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
                Some(HotkeyToken::Letter { character: 'Q' }),
                Some(HotkeyToken::Letter { character: 'W' }),
                Some(HotkeyToken::Letter { character: 'E' }),
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
                Some(HotkeyToken::Letter { character: 'Q' }),
                Some(HotkeyToken::Letter { character: 'Q' }),
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
        assert!(matches!(resolution.kind, SectionKind::Ability));
    }

    #[test]
    fn section_resolution_resolves_known_command() {
        let resolution = SectionResolution::from_section_id("CmdAttack").unwrap();
        assert!(matches!(resolution.kind, SectionKind::Command));
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
        assert_eq!(lower_unwrapped.canonical_id, upper_unwrapped.canonical_id);
    }
}

#[derive(Default)]
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
        let mut binding = AbilityBinding::default();
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

#[derive(Default)]
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
        let mut binding = CommandBinding::default();
        binding.set_hotkey(hotkey);
        binding.set_button_position(button_position);
        binding.set_unbutton_position(unbutton_position);
        binding.set_tip(tip);
        binding.set_un_tip(un_tip);
        binding
    }
}

#[derive(Default)]
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
mod builder_tests {
    use super::*;
    use warcraft_api::{SystemKeybindClass, SystemKeybindModifier};

    #[test]
    fn hotkey_letter_is_uppercased() {
        let hotkey = Hotkey::from('q');
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_uppercase_letter_is_unchanged() {
        let hotkey = Hotkey::from('Q');
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_function_key_single_digit() {
        let hotkey = Hotkey::FunctionKey(1);
        assert_eq!(hotkey.to_string(), "F1");
    }

    #[test]
    fn hotkey_function_key_double_digit() {
        let hotkey = Hotkey::FunctionKey(12);
        assert_eq!(hotkey.to_string(), "F12");
    }

    #[test]
    fn hotkey_virtual_key_passes_value_through() {
        let hotkey = Hotkey::VirtualKey(512);
        assert_eq!(hotkey.to_string(), "512");
    }

    #[test]
    fn hotkey_display_matches_letter() {
        let hotkey = Hotkey::from('W');
        let displayed = hotkey.to_string();
        assert_eq!(displayed, "W");
    }

    #[test]
    fn hotkey_into_string() {
        let hotkey = Hotkey::from('E');
        let converted: String = hotkey.into();
        assert_eq!(converted, "E");
    }

    #[test]
    fn hotkey_parses_letter_from_str() {
        let hotkey = Hotkey::try_from("Q").unwrap();
        assert_eq!(hotkey, Hotkey::Letter('Q'));
    }

    #[test]
    fn hotkey_parses_function_key_from_str() {
        let hotkey = Hotkey::try_from("F5").unwrap();
        assert_eq!(hotkey, Hotkey::FunctionKey(5));
    }

    #[test]
    fn hotkey_parses_function_key_case_insensitive() {
        let hotkey = Hotkey::try_from("f5").unwrap();
        assert_eq!(hotkey, Hotkey::FunctionKey(5));
    }

    #[test]
    fn hotkey_empty_string_returns_err() {
        assert!(Hotkey::try_from("").is_err());
    }

    #[test]
    fn hotkey_numeric_string_becomes_virtual_key() {
        let hotkey = Hotkey::try_from("512").unwrap();
        assert_eq!(hotkey, Hotkey::VirtualKey(512));
    }

    #[test]
    fn hotkey_multi_level_parses_from_comma_separated() {
        use crate::identity::hotkey_token::HotkeyToken;
        let hotkey = Hotkey::try_from("Q,Q,Q").unwrap();
        let expected = Hotkey::MultiLevel {
            tokens: [
                Some(HotkeyToken::Letter { character: 'Q' }),
                Some(HotkeyToken::Letter { character: 'Q' }),
                Some(HotkeyToken::Letter { character: 'Q' }),
                None,
            ],
        };
        assert_eq!(hotkey, expected);
    }

    #[test]
    fn hotkey_multi_level_displays_with_commas() {
        use crate::identity::hotkey_token::HotkeyToken;
        let hotkey = Hotkey::MultiLevel {
            tokens: [
                Some(HotkeyToken::Letter { character: 'Q' }),
                Some(HotkeyToken::Letter { character: 'W' }),
                None,
                None,
            ],
        };
        assert_eq!(hotkey.to_string(), "Q,W");
    }

    #[test]
    fn modifier_alt_displays_correctly() {
        assert_eq!(AbilityModifier::Alt.to_string(), "Alt");
    }

    #[test]
    fn modifier_ctrl_displays_correctly() {
        assert_eq!(AbilityModifier::Ctrl.to_string(), "Ctrl");
    }

    #[test]
    fn modifier_ctrl_or_alt_displays_correctly() {
        assert_eq!(AbilityModifier::CtrlOrAlt.to_string(), "Ctrl_or_Alt");
    }

    #[test]
    fn modifier_shift_displays_correctly() {
        assert_eq!(AbilityModifier::Shift.to_string(), "Shift");
    }

    #[test]
    fn modifier_into_string() {
        let converted: String = AbilityModifier::Alt.into();
        assert_eq!(converted, "Alt");
    }

    #[test]
    fn modifier_parses_from_str() {
        assert_eq!(
            AbilityModifier::try_from("Alt").unwrap(),
            AbilityModifier::Alt
        );
        assert_eq!(
            AbilityModifier::try_from("Ctrl").unwrap(),
            AbilityModifier::Ctrl
        );
        assert_eq!(
            AbilityModifier::try_from("Ctrl_or_Alt").unwrap(),
            AbilityModifier::CtrlOrAlt
        );
        assert_eq!(
            AbilityModifier::try_from("Shift").unwrap(),
            AbilityModifier::Shift
        );
    }

    #[test]
    fn modifier_parse_is_case_insensitive() {
        assert_eq!(
            AbilityModifier::try_from("ALT").unwrap(),
            AbilityModifier::Alt
        );
        assert_eq!(
            AbilityModifier::try_from("ctrl").unwrap(),
            AbilityModifier::Ctrl
        );
    }

    #[test]
    fn modifier_unknown_value_returns_err() {
        assert!(AbilityModifier::try_from("Meta").is_err());
    }

    #[test]
    fn ability_builder_empty_produces_all_none_binding() {
        let binding = AbilityBinding::builder().build();
        assert!(binding.hotkey().is_none());
        assert!(binding.unhotkey().is_none());
        assert!(binding.button_position().is_none());
        assert!(binding.unbutton_position().is_none());
        assert!(binding.research_hotkey().is_none());
        assert!(binding.research_button_position().is_none());
        assert!(binding.tip().is_none());
        assert!(binding.research_tip().is_none());
        assert!(binding.un_tip().is_none());
        assert!(binding.ubertip().is_none());
        assert!(binding.research_ubertip().is_none());
        assert!(binding.un_ubertip().is_none());
        assert!(binding.icon().is_none());
        assert!(binding.un_icon().is_none());
        assert!(binding.modifier().is_none());
    }

    #[test]
    fn ability_builder_sets_hotkey() {
        let hotkey = Hotkey::from('Q');
        let expected = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        assert_eq!(binding.hotkey(), Some(&expected));
    }

    #[test]
    fn ability_builder_sets_unhotkey() {
        let hotkey = Hotkey::from('W');
        let expected = Hotkey::from('W');
        let binding = AbilityBinding::builder().unhotkey(hotkey).build();
        assert_eq!(binding.unhotkey(), Some(&expected));
    }

    #[test]
    fn ability_builder_sets_button_position() {
        let position = GridCoordinate::new(ColumnIndex::Two, RowIndex::One);
        let binding = AbilityBinding::builder().button_position(position).build();
        assert_eq!(
            binding.button_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::Two, RowIndex::One))
        );
    }

    #[test]
    fn ability_builder_sets_unbutton_position() {
        let position = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        let binding = AbilityBinding::builder()
            .unbutton_position(position)
            .build();
        assert_eq!(
            binding.unbutton_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::Three, RowIndex::Two))
        );
    }

    #[test]
    fn ability_builder_sets_research_hotkey() {
        let hotkey = Hotkey::from('R');
        let expected = Hotkey::from('R');
        let binding = AbilityBinding::builder().research_hotkey(hotkey).build();
        assert_eq!(binding.research_hotkey(), Some(&expected));
    }

    #[test]
    fn ability_builder_sets_research_button_position() {
        let position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let binding = AbilityBinding::builder()
            .research_button_position(position)
            .build();
        assert_eq!(
            binding.research_button_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::One, RowIndex::Zero))
        );
    }

    #[test]
    fn ability_builder_sets_tip() {
        let binding = AbilityBinding::builder().tip("Cast Holy Light").build();
        assert_eq!(binding.tip(), Some("Cast Holy Light"));
    }

    #[test]
    fn ability_builder_sets_research_tip() {
        let binding = AbilityBinding::builder()
            .research_tip("Research Paladin")
            .build();
        assert_eq!(binding.research_tip(), Some("Research Paladin"));
    }

    #[test]
    fn ability_builder_sets_un_tip() {
        let binding = AbilityBinding::builder().un_tip("Cancel").build();
        assert_eq!(binding.un_tip(), Some("Cancel"));
    }

    #[test]
    fn ability_builder_sets_ubertip() {
        let binding = AbilityBinding::builder()
            .ubertip("Heals a friendly unit.")
            .build();
        assert_eq!(binding.ubertip(), Some("Heals a friendly unit."));
    }

    #[test]
    fn ability_builder_sets_research_ubertip() {
        let binding = AbilityBinding::builder()
            .research_ubertip("Researches something.")
            .build();
        assert_eq!(binding.research_ubertip(), Some("Researches something."));
    }

    #[test]
    fn ability_builder_sets_un_ubertip() {
        let binding = AbilityBinding::builder()
            .un_ubertip("Off form description.")
            .build();
        assert_eq!(binding.un_ubertip(), Some("Off form description."));
    }

    #[test]
    fn ability_builder_sets_icon() {
        let binding = AbilityBinding::builder()
            .icon("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp")
            .build();
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn ability_builder_sets_un_icon() {
        let binding = AbilityBinding::builder()
            .un_icon("ReplaceableTextures\\CommandButtons\\BTNCancel.blp")
            .build();
        assert_eq!(
            binding.un_icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNCancel.blp"),
        );
    }

    #[test]
    fn ability_builder_sets_modifier() {
        let binding = AbilityBinding::builder()
            .modifier(AbilityModifier::Alt)
            .build();
        assert_eq!(binding.modifier(), Some(AbilityModifier::Alt));
    }

    #[test]
    fn ability_builder_sets_modifier_ctrl_or_alt() {
        let binding = AbilityBinding::builder()
            .modifier(AbilityModifier::CtrlOrAlt)
            .build();
        assert_eq!(binding.modifier(), Some(AbilityModifier::CtrlOrAlt));
    }

    #[test]
    fn ability_builder_all_fields_survive_serialization_round_trip() {
        let hotkey = Hotkey::from('Q');
        let unhotkey = Hotkey::from('W');
        let research_hotkey = Hotkey::from('E');
        let button_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let unbutton_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Two);
        let research_button_position = GridCoordinate::new(ColumnIndex::Three, RowIndex::Zero);
        let binding = AbilityBinding::builder()
            .hotkey(hotkey)
            .unhotkey(unhotkey)
            .button_position(button_position)
            .unbutton_position(unbutton_position)
            .research_hotkey(research_hotkey)
            .research_button_position(research_button_position)
            .tip("My Tip")
            .research_tip("Research Tip")
            .un_tip("Un Tip")
            .ubertip("Uber Tip")
            .research_ubertip("Research Uber")
            .un_ubertip("Un Uber")
            .icon("buttons\\BTNFoo.blp")
            .un_icon("buttons\\BTNBar.blp")
            .modifier(AbilityModifier::Shift)
            .build();
        let file = CustomKeys::builder().ability("Ahrl", binding).build();
        let serialized = file.to_string();
        let reparsed = CustomKeys::from(serialized.as_str());
        let reparsed_binding = reparsed
            .binding("Ahrl")
            .expect("Ahrl must survive round-trip");
        assert_eq!(reparsed_binding.hotkey(), Some(&Hotkey::Letter('Q')));
        assert_eq!(reparsed_binding.unhotkey(), Some(&Hotkey::Letter('W')));
        assert_eq!(
            reparsed_binding.button_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two))
        );
        assert_eq!(
            reparsed_binding.unbutton_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::One, RowIndex::Two))
        );
        assert_eq!(
            reparsed_binding.research_hotkey(),
            Some(&Hotkey::Letter('E'))
        );
        assert_eq!(
            reparsed_binding.research_button_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::Three, RowIndex::Zero)),
        );
        assert_eq!(reparsed_binding.tip(), Some("My Tip"));
        assert_eq!(reparsed_binding.research_tip(), Some("Research Tip"));
        assert_eq!(reparsed_binding.un_tip(), Some("Un Tip"));
        assert_eq!(reparsed_binding.ubertip(), Some("Uber Tip"));
        assert_eq!(reparsed_binding.research_ubertip(), Some("Research Uber"));
        assert_eq!(reparsed_binding.un_ubertip(), Some("Un Uber"));
        assert_eq!(reparsed_binding.icon(), Some("buttons\\BTNFoo.blp"));
        assert_eq!(reparsed_binding.modifier(), Some(AbilityModifier::Shift));
    }

    #[test]
    fn ability_builder_function_key_hotkey_round_trips() {
        let hotkey = Hotkey::FunctionKey(5);
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().ability("Ahrl", binding).build();
        let serialized = file.to_string();
        let reparsed = CustomKeys::from(serialized.as_str());
        let hotkey_value = reparsed
            .binding("Ahrl")
            .and_then(|binding| binding.hotkey());
        assert_eq!(hotkey_value, Some(&Hotkey::FunctionKey(5)));
    }

    #[test]
    fn command_builder_empty_produces_all_none_binding() {
        let binding = CommandBinding::builder().build();
        assert!(binding.hotkey().is_none());
        assert!(binding.button_position().is_none());
        assert!(binding.unbutton_position().is_none());
        assert!(binding.tip().is_none());
        assert!(binding.un_tip().is_none());
    }

    #[test]
    fn command_builder_sets_hotkey() {
        let hotkey = Hotkey::from('M');
        let expected = Hotkey::from('M');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        assert_eq!(binding.hotkey(), Some(&expected));
    }

    #[test]
    fn command_builder_sets_button_position() {
        let position = GridCoordinate::new(ColumnIndex::One, RowIndex::Two);
        let binding = CommandBinding::builder().button_position(position).build();
        assert_eq!(
            binding.button_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::One, RowIndex::Two))
        );
    }

    #[test]
    fn command_builder_sets_unbutton_position() {
        let position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::One);
        let binding = CommandBinding::builder()
            .unbutton_position(position)
            .build();
        assert_eq!(
            binding.unbutton_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::Zero, RowIndex::One))
        );
    }

    #[test]
    fn command_builder_sets_tip() {
        let binding = CommandBinding::builder().tip("Move").build();
        assert_eq!(binding.tip(), Some("Move"));
    }

    #[test]
    fn command_builder_sets_un_tip() {
        let binding = CommandBinding::builder().un_tip("Cancel Move").build();
        assert_eq!(binding.un_tip(), Some("Cancel Move"));
    }

    #[test]
    fn command_builder_all_fields_survive_serialization_round_trip() {
        let hotkey = Hotkey::from('M');
        let button_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Two);
        let unbutton_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::Two);
        let binding = CommandBinding::builder()
            .hotkey(hotkey)
            .button_position(button_position)
            .unbutton_position(unbutton_position)
            .tip("Move Unit")
            .un_tip("Cancel Move")
            .build();
        let file = CustomKeys::builder().command("CmdMove", binding).build();
        let serialized = file.to_string();
        let reparsed = CustomKeys::from(serialized.as_str());
        let reparsed_binding = reparsed
            .command("CmdMove")
            .expect("CmdMove must survive round-trip");
        assert_eq!(reparsed_binding.hotkey(), Some(&Hotkey::Letter('M')));
        assert_eq!(
            reparsed_binding.button_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::One, RowIndex::Two))
        );
        assert_eq!(
            reparsed_binding.unbutton_position().copied(),
            Some(GridCoordinate::new(ColumnIndex::Two, RowIndex::Two)),
        );
        assert_eq!(reparsed_binding.tip(), Some("Move Unit"));
        assert_eq!(reparsed_binding.un_tip(), Some("Cancel Move"));
    }

    #[test]
    fn file_builder_single_ability_entry_is_accessible() {
        let hotkey = Hotkey::from('Q');
        let expected = Hotkey::from('Q');
        let position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let binding = AbilityBinding::builder()
            .hotkey(hotkey)
            .button_position(position)
            .build();
        let file = CustomKeys::builder().ability("Ahrl", binding).build();
        let retrieved = file.binding("Ahrl").expect("Ahrl must be present");
        assert_eq!(retrieved.hotkey(), Some(&expected));
    }

    #[test]
    fn file_builder_lookup_uses_canonical_case() {
        let hotkey = Hotkey::from('T');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().ability("Hpal", binding).build();
        assert!(file.binding("Hpal").is_some());
    }

    #[test]
    fn file_builder_multiple_entries_iterate_in_alphabetical_order() {
        let binding_ahrl = AbilityBinding::builder().tip("First").build();
        let binding_ahbz = AbilityBinding::builder().tip("Second").build();
        let binding_ahhb = AbilityBinding::builder().tip("Third").build();
        let file = CustomKeys::builder()
            .ability("Ahrl", binding_ahrl)
            .ability("AHbz", binding_ahbz)
            .ability("AHhb", binding_ahhb)
            .build();
        let ids: Vec<&str> = file
            .bindings_in_order()
            .map(|entry| entry.ability_id().value())
            .collect();
        assert_eq!(ids, ["AHbz", "AHhb", "Ahrl"]);
    }

    #[test]
    fn file_builder_command_entry_is_accessible() {
        let hotkey = Hotkey::from('A');
        let expected = Hotkey::from('A');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().command("CmdAttack", binding).build();
        let retrieved = file
            .command("CmdAttack")
            .expect("CmdAttack must be present");
        assert_eq!(retrieved.hotkey(), Some(&expected));
    }

    #[test]
    fn file_builder_system_entry_is_accessible() {
        let binding = SystemBinding::new(Hotkey::VirtualKey(9), SystemKeybindClass::Game, None);
        let file = CustomKeys::builder()
            .system("IsHeroSelect", binding)
            .build();
        let retrieved = file
            .system("IsHeroSelect")
            .expect("IsHeroSelect must be present");
        assert_eq!(retrieved.hotkey(), &Hotkey::VirtualKey(9));
        assert_eq!(retrieved.class(), SystemKeybindClass::Game);
    }

    #[test]
    fn file_builder_mixed_entry_types_coexist() {
        let ability_hotkey = Hotkey::from('Q');
        let ability = AbilityBinding::builder().hotkey(ability_hotkey).build();
        let command_hotkey = Hotkey::from('A');
        let command = CommandBinding::builder().hotkey(command_hotkey).build();
        let system = SystemBinding::new(Hotkey::VirtualKey(9), SystemKeybindClass::Game, None);
        let file = CustomKeys::builder()
            .ability("Ahrl", ability)
            .command("CmdAttack", command)
            .system("IsHeroSelect", system)
            .build();
        assert!(file.binding("Ahrl").is_some());
        assert!(file.command("CmdAttack").is_some());
        assert!(file.system("IsHeroSelect").is_some());
    }

    #[test]
    fn file_builder_ability_is_not_returned_as_command() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().ability("Ahrl", binding).build();
        assert!(file.command("Ahrl").is_none());
        assert!(file.system("Ahrl").is_none());
    }

    #[test]
    fn file_builder_serializes_ability_section_header() {
        let binding = AbilityBinding::builder().tip("test").build();
        let file = CustomKeys::builder().ability("AHhb", binding).build();
        let serialized = file.to_string();
        assert!(
            serialized.contains("[AHhb]"),
            "section header must appear in output with its canonical id case"
        );
    }

    #[test]
    fn file_builder_serializes_command_section_header() {
        let binding = CommandBinding::builder().tip("Move").build();
        let file = CustomKeys::builder().command("CmdMove", binding).build();
        let serialized = file.to_string();
        assert!(
            serialized.contains("[CmdMove]"),
            "command section header must appear in output with its canonical id case"
        );
    }

    #[test]
    fn file_builder_round_trips_through_parse() {
        let hotkey = Hotkey::from('Q');
        let position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let binding = AbilityBinding::builder()
            .hotkey(hotkey)
            .button_position(position)
            .tip("Holy Light")
            .build();
        let original_file = CustomKeys::builder().ability("Ahrl", binding).build();
        let serialized = original_file.to_string();
        let reparsed_file = CustomKeys::from(serialized.as_str());
        let original_binding = original_file.binding("Ahrl").expect("present in original");
        let reparsed_binding = reparsed_file
            .binding("Ahrl")
            .expect("present after round-trip");
        assert_eq!(original_binding.hotkey(), reparsed_binding.hotkey());
        assert_eq!(
            original_binding.button_position(),
            reparsed_binding.button_position()
        );
        assert_eq!(original_binding.tip(), reparsed_binding.tip());
    }

    #[test]
    fn file_builder_system_entry_survives_serialization() {
        let binding = SystemBinding::new(
            Hotkey::VirtualKey(49),
            SystemKeybindClass::ControlGroup,
            Some(SystemKeybindModifier::Ctrl),
        );
        let file = CustomKeys::builder().system("Ctr1", binding).build();
        let serialized = file.to_string();
        let reparsed = CustomKeys::from(serialized.as_str());
        let retrieved = reparsed.system("Ctr1").expect("must survive round-trip");
        assert_eq!(retrieved.hotkey(), &Hotkey::VirtualKey(49));
        assert_eq!(retrieved.class(), SystemKeybindClass::ControlGroup);
        assert_eq!(retrieved.modifier(), Some(SystemKeybindModifier::Ctrl));
    }
}
