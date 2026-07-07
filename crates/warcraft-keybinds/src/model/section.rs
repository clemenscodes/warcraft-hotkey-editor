use super::AbilityBinding;
use super::AbilityModifier;
use super::CommandBinding;
use super::GridCoordinate;
use super::Hotkey;
use super::SystemBinding;
use super::WarcraftKeybinding;
use super::ability_binding::{AbilitySlotData, ResearchSlotData};
use warcraft_api::{
    SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId, WarcraftObjectKind,
};
use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};

/// The type of a CustomKeys.txt section, determined from the game database.
#[derive(Debug, Clone, Copy)]
pub(crate) enum SectionKind {
    Ability,
    Command,
    System(SystemKeybindClass),
}

/// Resolved section identity: the canonical database ID and the binding kind.
/// Returned by `SectionResolution::from_section_id`; replaces a raw tuple.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SectionResolution {
    canonical_id: WarcraftObjectId,
    kind: SectionKind,
}

impl SectionResolution {
    pub(crate) fn canonical_id(&self) -> WarcraftObjectId {
        self.canonical_id
    }

    pub(crate) fn kind(&self) -> SectionKind {
        self.kind
    }

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Clone, Debug)]
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
                let hotkey = accumulator.hotkey;
                let button_position = accumulator.button_position;
                let unbutton_position = accumulator.unbutton_position;
                let tip = accumulator.tip;
                let un_tip = accumulator.un_tip;
                let command_binding = CommandBinding::from_parts(
                    hotkey,
                    button_position,
                    unbutton_position,
                    tip,
                    un_tip,
                );
                Self::Command(command_binding)
            }
            SectionKind::System(class) => {
                let missing_hotkey = Hotkey::VirtualKey(0);
                let hotkey = accumulator.hotkey.unwrap_or(missing_hotkey);
                let modifier = accumulator.system_modifier;
                let system_binding = SystemBinding::new(hotkey, class, modifier);
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
                let modifier = accumulator.modifier;
                let ability_binding =
                    AbilityBinding::from_parts(primary_slot, alt_slot, research_slot, modifier);
                Self::Ability(ability_binding)
            }
        }
    }
}
