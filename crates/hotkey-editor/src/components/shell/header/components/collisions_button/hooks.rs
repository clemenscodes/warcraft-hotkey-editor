use dioxus::prelude::*;
use warcraft_keybinds::CollisionSummary;

use super::logic::CollisionsButtonPresentation;
use super::props::CollisionsButtonProps;

/// Primitive hook: the one place this component reaches `warcraft-keybinds`. It
/// reads the live config and grid layout and asks the domain to count the
/// collisions, recomputing only when either signal changes.
fn use_collision_summary(props: &CollisionsButtonProps) -> CollisionSummary {
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let summary = use_memo(move || {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return CollisionSummary::default();
        };
        let layout = *grid_layout.read();
        CollisionSummary::compute(file, layout)
    });
    summary()
}

/// Composed hook: wires the domain summary into the button's full presentation,
/// so the body sees a single already-shaped value.
pub fn use_collisions_button(props: &CollisionsButtonProps) -> CollisionsButtonPresentation {
    let summary = use_collision_summary(props);
    CollisionsButtonPresentation::build(summary, props)
}
