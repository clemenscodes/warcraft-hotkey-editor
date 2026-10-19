pub mod apply_grid_layout_command;
pub mod move_slot_command;
pub mod resolve_conflicts_command;
pub mod set_hotkey_command;
pub mod set_system_hotkey_command;
pub mod swap_system_bindings_command;

#[cfg(test)]
pub(crate) fn assert_command<TheCommand>()
where
    TheCommand: ddd::Command<warcraft_keybinds::CustomKeys>,
{
}
