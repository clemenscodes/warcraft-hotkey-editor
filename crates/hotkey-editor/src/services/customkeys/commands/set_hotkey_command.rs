use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::HotkeyTarget;
use warcraft_keybinds::HotkeyToken;

pub struct SetHotkeyCommand {
    target: HotkeyTarget,
    token: Option<HotkeyToken>,
}

impl SetHotkeyCommand {
    pub fn new(target: HotkeyTarget, token: Option<HotkeyToken>) -> Self {
        Self { target, token }
    }
}

impl Layered for SetHotkeyCommand {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for SetHotkeyCommand {
    type Outcome = ();

    fn execute(self, keys: &mut CustomKeys) {
        keys.set_hotkey(self.target, self.token);
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::SetHotkeyCommand;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn set_hotkey_is_a_command() {
        assert_command::<SetHotkeyCommand>();
    }
}
