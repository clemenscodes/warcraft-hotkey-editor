use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CascadePlan;
use warcraft_keybinds::CustomKeys;

/// Run the position cascade that resolves every outstanding collision. Its outcome
/// is the [`CascadePlan`] describing which slots moved and which could not be placed.
pub struct ResolveConflictsCommand;

impl Layered for ResolveConflictsCommand {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for ResolveConflictsCommand {
    type Outcome = CascadePlan;

    fn execute(self, keys: &mut CustomKeys) -> CascadePlan {
        keys.resolve_conflicts()
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::ResolveConflictsCommand;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn resolve_conflicts_is_a_command() {
        assert_command::<ResolveConflictsCommand>();
    }
}
