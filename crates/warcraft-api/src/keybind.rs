use std::fmt;

/// Set of runtime contexts in which a system hotkey is active.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextSet {
    in_player: bool,
    in_observer: bool,
    in_replay: bool,
}

impl ContextSet {
    pub const ALWAYS: Self = Self {
        in_player: true,
        in_observer: true,
        in_replay: true,
    };

    pub const PLAYER_ONLY: Self = Self {
        in_player: true,
        in_observer: false,
        in_replay: false,
    };

    pub const OBSERVER_ONLY: Self = Self {
        in_player: false,
        in_observer: true,
        in_replay: false,
    };

    pub const REPLAY_ONLY: Self = Self {
        in_player: false,
        in_observer: false,
        in_replay: true,
    };

    pub fn overlaps(self, other: Self) -> bool {
        (self.in_player && other.in_player)
            || (self.in_observer && other.in_observer)
            || (self.in_replay && other.in_replay)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKeybindClass {
    Menu,
    ControlGroup,
    Game,
    Camera,
    Observer,
    Replay,
}

impl SystemKeybindClass {
    pub fn ini_field(self) -> &'static str {
        match self {
            Self::Game => "GameCommand=1",
            Self::ControlGroup => "CtrlGroupCommand=1",
            Self::Menu => "MenuCommand=1",
            Self::Camera => "CameraCommand=1",
            Self::Observer => "ObserverCommand=1",
            Self::Replay => "ReplayCommand=1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKeybindModifier {
    None,
    Alt,
    Ctrl,
    CtrlOrAlt,
    Shift,
}

impl SystemKeybindModifier {
    pub fn ini_str(self) -> Option<&'static str> {
        match self {
            Self::None => Option::None,
            Self::Alt => Some("Alt"),
            Self::Ctrl => Some("Ctrl"),
            Self::CtrlOrAlt => Some("Ctrl_or_Alt"),
            Self::Shift => Some("Shift"),
        }
    }
}

impl fmt::Display for SystemKeybindModifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::None => "",
            Self::Alt => "Alt + ",
            Self::Ctrl => "Ctrl + ",
            Self::CtrlOrAlt => "Ctrl/Alt + ",
            Self::Shift => "Shift + ",
        })
    }
}

impl TryFrom<&str> for SystemKeybindModifier {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, ()> {
        match text.trim().to_ascii_lowercase().as_str() {
            "alt" => Ok(Self::Alt),
            "ctrl" => Ok(Self::Ctrl),
            "ctrl_or_alt" => Ok(Self::CtrlOrAlt),
            "shift" => Ok(Self::Shift),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SystemKeybind {
    section_id: &'static str,
    comment: &'static str,
    default_hotkey: u32,
    default_modifier: SystemKeybindModifier,
    class: SystemKeybindClass,
}

impl SystemKeybind {
    pub const fn new(
        section_id: &'static str,
        comment: &'static str,
        default_hotkey: u32,
        default_modifier: SystemKeybindModifier,
        class: SystemKeybindClass,
    ) -> Self {
        Self {
            section_id,
            comment,
            default_hotkey,
            default_modifier,
            class,
        }
    }

    pub fn section_id(&self) -> &'static str {
        self.section_id
    }

    pub fn comment(&self) -> &'static str {
        self.comment
    }

    pub fn default_hotkey(&self) -> u32 {
        self.default_hotkey
    }

    pub fn default_modifier(&self) -> SystemKeybindModifier {
        self.default_modifier
    }

    pub fn class(&self) -> SystemKeybindClass {
        self.class
    }

    pub fn context_set(&self) -> ContextSet {
        match self.section_id {
            "itm1" | "itm2" | "itm3" | "itm4" | "itm5" | "itm6" => ContextSet::PLAYER_ONLY,
            "her1" | "her2" | "her3" => ContextSet::PLAYER_ONLY,
            "Ctr1" | "Ctr2" | "Ctr3" | "Ctr4" | "Ctr5" | "Ctr6" | "Ctr7" | "Ctr8" | "Ctr9"
            | "Ctr0" => ContextSet::PLAYER_ONLY,
            "sbgp" | "sidw" | "mpng" | "TFmv" | "Ally" | "QSav" | "MSav" | "MLod" => {
                ContextSet::PLAYER_ONLY
            }
            "THer" | "TSta" | "TPUQ" | "TSel" | "TMap" | "TToD" | "TAll" | "SPQM" | "SULM" => {
                ContextSet::OBSERVER_ONLY
            }
            "TRpl" | "TPPl" | "InGS" | "DeGS" | "TFoW" | "TAuC" => ContextSet::REPLAY_ONLY,
            _ => ContextSet::ALWAYS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_keybind_modifier_try_from_known_strings() {
        assert_eq!(
            SystemKeybindModifier::try_from("Alt"),
            Ok(SystemKeybindModifier::Alt)
        );
        assert_eq!(
            SystemKeybindModifier::try_from("ctrl"),
            Ok(SystemKeybindModifier::Ctrl)
        );
        assert_eq!(
            SystemKeybindModifier::try_from("Ctrl_or_Alt"),
            Ok(SystemKeybindModifier::CtrlOrAlt)
        );
        assert_eq!(
            SystemKeybindModifier::try_from("shift"),
            Ok(SystemKeybindModifier::Shift)
        );
    }

    #[test]
    fn system_keybind_modifier_try_from_unknown_is_error() {
        assert!(SystemKeybindModifier::try_from("Meta").is_err());
        assert!(SystemKeybindModifier::try_from("").is_err());
    }

    #[test]
    fn system_keybind_modifier_ini_str_none_produces_no_string() {
        assert!(SystemKeybindModifier::None.ini_str().is_none());
    }

    #[test]
    fn system_keybind_modifier_ini_str_matches_expected_tokens() {
        assert_eq!(SystemKeybindModifier::Alt.ini_str(), Some("Alt"));
        assert_eq!(SystemKeybindModifier::Ctrl.ini_str(), Some("Ctrl"));
        assert_eq!(
            SystemKeybindModifier::CtrlOrAlt.ini_str(),
            Some("Ctrl_or_Alt")
        );
        assert_eq!(SystemKeybindModifier::Shift.ini_str(), Some("Shift"));
    }

    #[test]
    fn system_keybind_class_ini_field_contains_equals_one() {
        for class in [
            SystemKeybindClass::Game,
            SystemKeybindClass::ControlGroup,
            SystemKeybindClass::Menu,
            SystemKeybindClass::Camera,
            SystemKeybindClass::Observer,
            SystemKeybindClass::Replay,
        ] {
            assert!(
                class.ini_field().contains("=1"),
                "ini_field for {class:?} should contain '=1'"
            );
        }
    }

    #[test]
    fn system_keybind_accessors_return_stored_values() {
        let keybind = SystemKeybind::new(
            "QuickSave",
            "Quick Save",
            0x53,
            SystemKeybindModifier::Ctrl,
            SystemKeybindClass::Game,
        );
        assert_eq!(keybind.section_id(), "QuickSave");
        assert_eq!(keybind.comment(), "Quick Save");
        assert_eq!(keybind.default_hotkey(), 0x53);
        assert!(matches!(
            keybind.default_modifier(),
            SystemKeybindModifier::Ctrl
        ));
        assert!(matches!(keybind.class(), SystemKeybindClass::Game));
    }
}
