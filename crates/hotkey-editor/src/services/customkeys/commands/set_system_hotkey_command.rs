use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::KeyCode;

/// Set one system keybind's hotkey. Owns the section id so the command is
/// self-contained (a `String` clone of the caller's `&str`).
pub struct SetSystemHotkey {
    section_id: String,
    code: KeyCode,
}

impl SetSystemHotkey {
    pub fn new(section_id: String, code: KeyCode) -> Self {
        Self { section_id, code }
    }
}

impl Layered for SetSystemHotkey {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for SetSystemHotkey {
    type Outcome = ();

    fn execute(self, keys: &mut CustomKeys) {
        keys.set_system_hotkey(&self.section_id, self.code);
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::SetSystemHotkey;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn set_system_hotkey_is_a_command() {
        assert_command::<SetSystemHotkey>();
    }
}
