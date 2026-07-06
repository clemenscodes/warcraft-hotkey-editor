//! The application-layer commands the [`super::service::CustomKeysService`]
//! dispatches. Each is a named [`ddd::Command`] over the `CustomKeys` aggregate:
//! a first-class, self-contained intention to change it, executed through the
//! service's write-through `commit` boundary (so every mutation re-normalizes
//! and persists). Commands are `ApplicationLayer` — they live here in the
//! renderer crate, not in the pure-domain `warcraft-keybinds` crate.

pub mod apply_grid_layout_command;
pub mod move_slot_command;
pub mod set_hotkey_command;
pub mod set_system_hotkey_command;
pub mod swap_system_bindings_command;

#[cfg(test)]
pub(crate) fn assert_command<TheCommand>()
where
    TheCommand: ddd::Command<warcraft_keybinds::CustomKeys>,
{
}
