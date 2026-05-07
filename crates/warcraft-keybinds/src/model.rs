use std::fmt;
use warcraft_api::{SystemKeybindClass, SystemKeybindModifier};

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
}

impl AbilityBinding {
    pub fn hotkey(&self) -> Option<&Hotkey> {
        self.hotkey.as_ref()
    }

    pub fn unhotkey(&self) -> Option<&Hotkey> {
        self.unhotkey.as_ref()
    }

    pub fn button_position(&self) -> Option<&ButtonPosition> {
        self.button_position.as_ref()
    }

    pub fn unbutton_position(&self) -> Option<&ButtonPosition> {
        self.unbutton_position.as_ref()
    }

    pub fn research_hotkey(&self) -> Option<&Hotkey> {
        self.research_hotkey.as_ref()
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

    pub fn modifier(&self) -> Option<AbilityModifier> {
        self.modifier
    }

    pub fn set_hotkey(&mut self, value: Option<Hotkey>) {
        self.hotkey = value;
    }

    pub fn set_unhotkey(&mut self, value: Option<Hotkey>) {
        self.unhotkey = value;
    }

    pub fn set_button_position(&mut self, value: Option<ButtonPosition>) {
        self.button_position = value;
    }

    pub fn set_unbutton_position(&mut self, value: Option<ButtonPosition>) {
        self.unbutton_position = value;
    }

    pub fn set_research_hotkey(&mut self, value: Option<Hotkey>) {
        self.research_hotkey = value;
    }

    pub fn set_research_button_position(&mut self, value: Option<ButtonPosition>) {
        self.research_button_position = value;
    }

    pub fn set_tip(&mut self, value: Option<String>) {
        self.tip = value;
    }

    pub fn set_research_tip(&mut self, value: Option<String>) {
        self.research_tip = value;
    }

    pub fn set_un_tip(&mut self, value: Option<String>) {
        self.un_tip = value;
    }

    pub fn set_ubertip(&mut self, value: Option<String>) {
        self.ubertip = value;
    }

    pub fn set_research_ubertip(&mut self, value: Option<String>) {
        self.research_ubertip = value;
    }

    pub fn set_un_ubertip(&mut self, value: Option<String>) {
        self.un_ubertip = value;
    }

    pub fn set_icon(&mut self, value: Option<String>) {
        self.icon = value;
    }

    pub fn set_un_icon(&mut self, value: Option<String>) {
        self.un_icon = value;
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
    id: &'a str,
    binding: &'a AbilityBinding,
}

impl<'a> BindingEntry<'a> {
    pub(crate) fn new(id: &'a str, binding: &'a AbilityBinding) -> Self {
        Self { id, binding }
    }

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
    pub(crate) fn new(name: &'a str, binding: &'a CommandBinding) -> Self {
        Self { name, binding }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn binding(&self) -> &'a CommandBinding {
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
        let lowercase_key = key.to_lowercase();
        match lowercase_key.as_str() {
            "hotkey" if self.hotkey.is_none() => {
                self.hotkey = Hotkey::try_from(value).ok();
            }
            "unhotkey" if self.unhotkey.is_none() => {
                self.unhotkey = Hotkey::try_from(value).ok();
            }
            "buttonpos" if self.button_position.is_none() => {
                self.button_position = ButtonPosition::try_from(value).ok();
            }
            "unbuttonpos" if self.unbutton_position.is_none() => {
                self.unbutton_position = ButtonPosition::try_from(value).ok();
            }
            "researchhotkey" if self.research_hotkey.is_none() => {
                self.research_hotkey = Hotkey::try_from(value).ok();
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
            "modifier" => {
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

    pub(crate) fn section_id_from(line: &str) -> Option<String> {
        let without_brackets = line.strip_prefix('[')?.strip_suffix(']')?;
        let section_id = without_brackets.trim();
        if section_id.is_empty() {
            None
        } else {
            Some(section_id.to_string())
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
                let hotkey = accumulator.hotkey.unwrap_or(Hotkey::VirtualKey(0));
                let system_binding = SystemBinding {
                    hotkey,
                    class,
                    modifier: accumulator.system_modifier,
                };
                Self::System(system_binding)
            }
            SectionKind::Ability => {
                let ability_binding = AbilityBinding {
                    hotkey: accumulator.hotkey,
                    unhotkey: accumulator.unhotkey,
                    button_position: accumulator.button_position,
                    unbutton_position: accumulator.unbutton_position,
                    research_hotkey: accumulator.research_hotkey,
                    research_button_position: accumulator.research_button_position,
                    tip: accumulator.tip,
                    research_tip: accumulator.research_tip,
                    un_tip: accumulator.un_tip,
                    ubertip: accumulator.ubertip,
                    research_ubertip: accumulator.research_ubertip,
                    un_ubertip: accumulator.un_ubertip,
                    icon: accumulator.icon,
                    un_icon: accumulator.un_icon,
                    modifier: accumulator.modifier,
                };
                Self::Ability(ability_binding)
            }
        }
    }
}
