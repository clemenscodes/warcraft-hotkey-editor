use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;

/// Reassign every binding's hotkey from a grid layout. Its outcome is the number
/// of bindings whose hotkey actually changed.
pub struct ApplyGridLayout {
    layout: GridLayout,
}

impl ApplyGridLayout {
    pub fn new(layout: GridLayout) -> Self {
        Self { layout }
    }
}

impl Layered for ApplyGridLayout {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for ApplyGridLayout {
    type Outcome = usize;

    fn execute(self, keys: &mut CustomKeys) -> usize {
        keys.apply_grid_to_all_bindings(self.layout)
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::ApplyGridLayout;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn apply_grid_layout_is_a_command() {
        assert_command::<ApplyGridLayout>();
    }
}
