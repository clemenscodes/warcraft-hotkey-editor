use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::KeyCode;
use warcraft_keybinds::WarcraftObjectId;

/// Set one system keybind's hotkey, addressed by the section's `WarcraftObjectId`
/// (a keybind section is one of a fixed, database-defined set of ids).
pub struct SetSystemHotkeyCommand {
    section_id: WarcraftObjectId,
    code: KeyCode,
}

impl SetSystemHotkeyCommand {
    pub fn new(section_id: WarcraftObjectId, code: KeyCode) -> Self {
        Self { section_id, code }
    }
}

impl Layered for SetSystemHotkeyCommand {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for SetSystemHotkeyCommand {
    type Outcome = ();

    fn execute(self, keys: &mut CustomKeys) {
        keys.set_system_hotkey(self.section_id, self.code);
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::SetSystemHotkeyCommand;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn set_system_hotkey_is_a_command() {
        assert_command::<SetSystemHotkeyCommand>();
    }
}
