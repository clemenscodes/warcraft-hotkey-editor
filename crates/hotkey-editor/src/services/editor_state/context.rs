use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::{SearchField, UnitKind, WarcraftObjectId};
use warcraft_keybinds::GridSlotId;

use crate::services::editor_state::EditorState;

/// Access the [`EditorState`] provided by the app shell. Call from a component or
/// hook body (it is a hook). The editor's UI state is a shell-wide concept the
/// editor page reads from context rather than as props, so this accessor lives
/// beside the type in `services/`, not colocated with any component.
pub(crate) fn use_editor_state() -> EditorState {
    use_context()
}

/// Create every editor-scoped signal, assemble the [`EditorState`], provide it as
/// context, and hand it back. All of these start empty/default — none is seeded from
/// the URL, since the editor's own view state is not part of the route. The one
/// input, `update_hotkeys_on_move`, is the persisted editor preference the shell
/// loads. The shell calls this once on boot; the editor page reads the result through
/// [`use_editor_state`].
pub(crate) fn use_editor_state_provider(update_hotkeys_on_move: Signal<bool>) -> EditorState {
    let selected_slot = use_signal::<Option<GridSlotId>>(|| None);
    let selected_hero_level = use_signal::<u32>(|| 1);
    let selected_from_research = use_signal::<bool>(|| false);
    let selected_from_uprooted = use_signal::<bool>(|| false);
    let hotkey_assign_request = use_signal::<bool>(|| false);
    let tier_overrides = use_signal::<HashMap<WarcraftObjectId, usize>>(HashMap::new);
    let search_field = use_signal(SearchField::default);
    let collapsed_categories = use_signal::<HashSet<UnitKind>>(HashSet::new);
    let active_category = use_signal::<UnitKind>(|| UnitKind::Soldier);
    let show_abilityless_units = use_signal::<bool>(|| false);
    let expand_variants = use_signal::<bool>(|| false);
    let editor_state = EditorState {
        selected_slot,
        selected_hero_level,
        selected_from_research,
        selected_from_uprooted,
        hotkey_assign_request,
        tier_overrides,
        search_field,
        collapsed_categories,
        active_category,
        show_abilityless_units,
        expand_variants,
        update_hotkeys_on_move,
    };
    use_context_provider(|| editor_state);
    editor_state
}
