use dioxus::prelude::*;

use crate::services::resolve_selection::ResolveSelection;

pub(crate) fn use_resolve_selection() -> ResolveSelection {
    use_context()
}

pub(crate) fn use_resolve_selection_provider(
    initial_move_category: Option<String>,
) -> ResolveSelection {
    let selected_move_category = use_signal::<Option<String>>(move || initial_move_category);
    let resolve_selection = ResolveSelection::new(selected_move_category);
    use_context_provider(|| resolve_selection);
    resolve_selection
}
