use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;

pub struct ApplyGridLayoutCommand {
    layout: GridLayout,
}

impl ApplyGridLayoutCommand {
    pub fn new(layout: GridLayout) -> Self {
        Self { layout }
    }
}

impl Layered for ApplyGridLayoutCommand {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for ApplyGridLayoutCommand {
    type Outcome = usize;

    fn execute(self, keys: &mut CustomKeys) -> usize {
        keys.apply_grid_to_all_bindings(self.layout)
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::ApplyGridLayoutCommand;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn apply_grid_layout_is_a_command() {
        assert_command::<ApplyGridLayoutCommand>();
    }
}
