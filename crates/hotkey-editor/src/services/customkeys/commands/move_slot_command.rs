use ddd::ApplicationLayer;
use ddd::Command;
use ddd::Layered;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::MoveRequest;

pub struct MoveSlotCommand<'a> {
    request: MoveRequest<'a>,
}

impl<'a> MoveSlotCommand<'a> {
    pub fn new(request: MoveRequest<'a>) -> Self {
        Self { request }
    }
}

impl Layered for MoveSlotCommand<'_> {
    type Layer = ApplicationLayer;
}

impl Command<CustomKeys> for MoveSlotCommand<'_> {
    type Outcome = ();

    fn execute(self, keys: &mut CustomKeys) {
        keys.move_slot(&self.request);
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::MoveSlotCommand;
    use crate::services::customkeys::commands::assert_command;

    #[test]
    fn move_slot_is_a_command() {
        assert_command::<MoveSlotCommand<'_>>();
    }
}
