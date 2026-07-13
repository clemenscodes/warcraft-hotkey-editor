use dioxus::prelude::*;

use crate::services::resolve_selection::ResolveSelection;

/// Access the [`ResolveSelection`] provided by the app shell. Call from a component
/// or hook body (it is a hook). The selected move-category is shell-wide (it feeds
/// the URL sync), so this accessor lives beside the type in `services/`, not
/// colocated with any component.
pub(crate) fn use_resolve_selection() -> ResolveSelection {
    use_context()
}

/// Create the selected-move-category signal (seeded from the entry the resolve URL
/// carried), assemble the [`ResolveSelection`], provide it as context, and hand it
/// back. The shell calls this once on boot; the resolve page reads the result through
/// [`use_resolve_selection`].
pub(crate) fn use_resolve_selection_provider(
    initial_move_category: Option<String>,
) -> ResolveSelection {
    let selected_move_category = use_signal::<Option<String>>(move || initial_move_category);
    let resolve_selection = ResolveSelection::new(selected_move_category);
    use_context_provider(|| resolve_selection);
    resolve_selection
}
