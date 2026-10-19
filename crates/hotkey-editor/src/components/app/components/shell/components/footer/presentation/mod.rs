use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;

/// Whether the mobile footer should be tucked away right now. The pager drives it
/// by scroll direction; the footer only reads it.
pub(super) fn use_footer_scrolled_away() -> bool {
    *use_editor_state().footer_hidden().read()
}
