use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::HotkeyTarget;
use warcraft_keybinds::HotkeyToken;

/// Override (or clear) the hotkey of one ability/research/off-state target.
pub struct SetHotkey {
    target: HotkeyTarget,
    token: Option<HotkeyToken>,
}

impl SetHotkey {
    pub fn new(target: HotkeyTarget, token: Option<HotkeyToken>) -> Self {
        Self { target, token }
    }
}

impl Layered for SetHotkey {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for SetHotkey {
    type Outcome = ();

    fn execute(self, keys: &mut CustomKeys) {
        keys.set_hotkey(self.target, self.token);
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::SetHotkey;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn set_hotkey_is_a_command() {
        assert_command::<SetHotkey>();
    }
}
