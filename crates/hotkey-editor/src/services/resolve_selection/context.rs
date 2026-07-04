use dioxus::prelude::*;

use crate::services::resolve_selection::ResolveSelection;

/// Access the [`ResolveSelection`] provided by the app shell. Call from a component
/// or hook body (it is a hook). The selected move-category is shell-wide (it feeds
/// the URL sync), so this accessor lives beside the type in `services/`, not
/// colocated with any component.
pub(crate) fn use_resolve_selection() -> ResolveSelection {
    use_context::<ResolveSelection>()
}
