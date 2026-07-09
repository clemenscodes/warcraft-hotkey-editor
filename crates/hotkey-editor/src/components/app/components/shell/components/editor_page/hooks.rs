use super::props::EditorPageProps;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::{use_synced_route, use_view_navigation};
use crate::services::navigation::editor_nav::DecodedEditorNav;
use crate::services::navigation::nav_snapshot::NavSnapshot;
use dioxus::prelude::*;

/// Reconcile the editor route into the shell's navigation signals. The reconcile is the
/// read side of the URL contract — decoding `?race=&mode=&unit=&search_query=` and
/// writing it into the navigation signals whenever the route changes (deep-link,
/// back/forward) — while the shell's push effect handles the write side. The workspace
/// and its children source every editor signal from context themselves, so the page
/// shapes no child props.
pub(super) fn use_editor_page(props: &EditorPageProps) {
    let navigation = use_view_navigation();
    let mut synced_route = use_synced_route();
    let decoded = DecodedEditorNav::decode(
        props.race.as_deref(),
        props.mode.as_deref(),
        props.unit.as_deref(),
        props.search_query.as_deref(),
    );
    use_effect(use_reactive!(|decoded| {
        navigation.restore(AppView::Editor, &decoded);
        let snapshot = NavSnapshot::Editor(decoded.clone());
        synced_route.set(snapshot);
    }));
}
