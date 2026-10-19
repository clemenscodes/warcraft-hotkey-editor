use dioxus::prelude::*;

use crate::services::search_dialog::SearchDialogDismiss;

/// The dismiss a picked search result uses to close the dialog, if one is in
/// scope. It is absent outside the search dialog, so the shared unit card
/// renders unchanged in the desktop unit list.
pub(crate) fn use_search_dialog_dismiss() -> Option<SearchDialogDismiss> {
    try_consume_context::<SearchDialogDismiss>()
}

pub(crate) fn use_search_dialog_dismiss_provider(on_open_change: Callback<bool>) {
    let search_dialog_dismiss = SearchDialogDismiss::new(on_open_change);
    use_context_provider(|| search_dialog_dismiss);
}
