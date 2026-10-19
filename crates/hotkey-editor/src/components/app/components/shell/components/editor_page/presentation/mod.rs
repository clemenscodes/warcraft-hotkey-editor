use super::model::EditorPageModel;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;
use dioxus::prelude::*;

pub(super) fn use_editor_page(props: &EditorPageModel) {
    let navigation = use_view_navigation();
    let decoded = DecodedEditorNavigation::decode(
        props.race.as_deref(),
        props.mode.as_deref(),
        props.unit.as_deref(),
        props.search_query.as_deref(),
    );
    use_effect(use_reactive!(|decoded| {
        navigation.restore(AppView::Editor, &decoded);
    }));
}
