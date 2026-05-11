pub(super) fn tile_class(
    has_occupant: bool,
    is_selected: bool,
    drag_in_progress: bool,
    is_command: bool,
    is_being_dragged: bool,
    is_drop_target: bool,
    is_off_state_blocked: bool,
) -> String {
    let base = match (has_occupant, is_selected, drag_in_progress, is_command) {
        (true, true, _, _) => "grid-tile has-ability selected",
        (true, false, _, true) => "grid-tile has-ability is-command",
        (true, false, _, false) => "grid-tile has-ability",
        (false, _, true, _) if is_off_state_blocked => "grid-tile blocked-drop-target",
        (false, _, true, _) => "grid-tile drop-target",
        (false, _, false, _) => "grid-tile",
    };
    let mut class = base.to_string();
    if is_being_dragged {
        class.push_str(" dragging-source");
    }
    if is_drop_target {
        class.push_str(" drag-over");
    }
    class
}
