use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::WarcraftObjectId;

pub struct SwapSystemBindingsCommand {
    source_id: WarcraftObjectId,
    target_id: WarcraftObjectId,
}

impl SwapSystemBindingsCommand {
    pub fn new(source_id: WarcraftObjectId, target_id: WarcraftObjectId) -> Self {
        Self {
            source_id,
            target_id,
        }
    }
}

impl Layered for SwapSystemBindingsCommand {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for SwapSystemBindingsCommand {
    type Outcome = ();

    fn execute(self, keys: &mut CustomKeys) {
        keys.swap_system_bindings(self.source_id, self.target_id);
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::SwapSystemBindingsCommand;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn swap_system_bindings_is_a_command() {
        assert_command::<SwapSystemBindingsCommand>();
    }
}
