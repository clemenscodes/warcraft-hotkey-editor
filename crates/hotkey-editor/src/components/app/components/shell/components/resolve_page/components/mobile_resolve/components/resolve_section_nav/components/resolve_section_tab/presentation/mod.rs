use super::model::ResolveSectionTabModel;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveCategory;
use dioxus::prelude::*;

/// Resolves the move category a tab colours itself for.
///
/// The nav hands each tab only its section title, the exact string
/// MoveCategory::section_title produced. Mapping it back with
/// MoveCategory::from_title keeps that title vocabulary in one place, so the tab
/// never carries a second copy of the reason strings. A title the mapping does
/// not know falls back to Fight, which cannot happen for the four fixed
/// sections.
pub(super) fn use_resolve_section_tab(props: &ResolveSectionTabModel) -> MoveCategory {
    let label = props.label.clone();
    let category_memo = use_memo(move || {
        let resolved = MoveCategory::from_title(&label);
        resolved.unwrap_or(MoveCategory::Fight)
    });
    category_memo()
}
