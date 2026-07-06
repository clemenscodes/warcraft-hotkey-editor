use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;

/// Exchange two system keybinds' hotkeys (the inventory drag-to-swap gesture).
/// Owns both section ids so the command is self-contained.
pub struct SwapSystemBindings {
    source_id: String,
    target_id: String,
}

impl SwapSystemBindings {
    pub fn new(source_id: String, target_id: String) -> Self {
        Self {
            source_id,
            target_id,
        }
    }
}

impl Layered for SwapSystemBindings {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for SwapSystemBindings {
    type Outcome = ();

    fn execute(self, keys: &mut CustomKeys) {
        keys.swap_system_bindings(&self.source_id, &self.target_id);
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::SwapSystemBindings;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn swap_system_bindings_is_a_command() {
        assert_command::<SwapSystemBindings>();
    }
}
