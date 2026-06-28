use dioxus::prelude::{Signal, WritableExt};
use warcraft_keybinds::{CustomKeys, GridLayout};

pub(crate) use warcraft_keybinds::MoveRequest;

pub(crate) struct Positions;

impl Positions {
    pub(crate) fn move_or_swap(
        custom_keys_signal: &mut Signal<Option<CustomKeys>>,
        request: MoveRequest<'_>,
    ) {
        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeys::from(""));
        file.move_slot(&request);
    }

    pub(crate) fn apply_grid_to_all_known_objects(
        custom_keys_signal: &mut Signal<Option<CustomKeys>>,
        layout: GridLayout,
    ) -> usize {
        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeys::from(""));
        file.apply_grid_to_all_bindings(layout)
    }
}
