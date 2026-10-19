use super::model::EditorPageModel;
use crate::components::app::route::Route;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;
use dioxus::prelude::*;

pub(super) fn use_editor_page(props: &EditorPageModel) {
    let navigation = use_view_navigation();
    let rendered_route = Route::Editor {
        race: props.race.clone(),
        mode: props.mode.clone(),
        unit: props.unit.clone(),
        search_query: props.search_query.clone(),
    };
    let decoded = DecodedEditorNavigation::decode(
        props.race.as_deref(),
        props.mode.as_deref(),
        props.unit.as_deref(),
        props.search_query.as_deref(),
    );
    // The effect runs after the render, so a quick click may already have
    // navigated on and set the signals to its own route. Only the route this
    // render was for is restored, and only while it is still the current one;
    // restoring it later would undo that click.
    use_effect(use_reactive!(|decoded, rendered_route| {
        let still_current = router().current::<Route>() == rendered_route;
        if still_current {
            navigation.restore(AppView::Editor, &decoded);
        }
    }));
}
