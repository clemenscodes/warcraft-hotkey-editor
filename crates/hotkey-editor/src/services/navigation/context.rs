use dioxus::prelude::*;
use warcraft_api::{Race, UnitModeSelection, WarcraftObjectId};

use crate::services::collision_selection::CollisionSelection;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;
use crate::services::navigation::navigation_command::NavigationCommand;
use crate::services::navigation::search_session::SearchSession;
use crate::services::navigation::view_navigation::{
    EditorNavigationSignals, ViewNavigationContext,
};
use crate::services::resolve_selection::ResolveSelection;

pub(crate) fn use_view_navigation() -> ViewNavigationContext {
    use_context()
}

pub(crate) fn use_view_navigation_provider(
    initial_view: AppView,
    navigation: DecodedEditorNavigation,
    collision_selection: CollisionSelection,
    resolve_selection: ResolveSelection,
    dispatch: Callback<NavigationCommand>,
) -> ViewNavigationContext {
    let initial_race = navigation.race();
    let initial_modes = navigation.unit_modes();
    let initial_unit_id = navigation.selected_unit_id();
    let initial_search = navigation.search_query().to_owned();
    let current_view = use_signal::<AppView>(move || initial_view);
    let active_race = use_signal::<Race>(move || initial_race);
    let unit_modes = use_signal::<UnitModeSelection>(move || initial_modes);
    let selected_unit_id = use_signal::<Option<WarcraftObjectId>>(move || initial_unit_id);
    let search_query = use_signal::<String>(move || initial_search);
    let search_session_active = use_signal::<bool>(|| false);
    let search_session_generation = use_signal::<u32>(|| 0);
    let search_session = SearchSession::new(search_session_active, search_session_generation);
    let editor_navigation = EditorNavigationSignals::new(
        current_view,
        active_race,
        unit_modes,
        selected_unit_id,
        search_query,
    );
    let view_navigation = ViewNavigationContext::new(
        editor_navigation,
        collision_selection,
        resolve_selection,
        search_session,
        dispatch,
    );
    use_context_provider(|| view_navigation);
    view_navigation
}
