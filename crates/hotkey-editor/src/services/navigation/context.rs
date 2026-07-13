use dioxus::prelude::*;
use warcraft_api::{Race, UnitMode, WarcraftObjectId};

use crate::services::collision_selection::CollisionSelection;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;
use crate::services::navigation::navigation_command::NavigationCommand;
use crate::services::navigation::search_session::SearchSession;
use crate::services::navigation::view_navigation::{
    EditorNavigationSignals, ViewNavigationContext,
};
use crate::services::resolve_selection::ResolveSelection;

/// Access the [`ViewNavigationContext`] provided at the app root. Call from a
/// component or hook body (it is a hook). Navigation is a global, crate-wide
/// concept — no single component owns it — so this accessor lives beside the
/// type in `services/`, not colocated with any component.
pub(crate) fn use_view_navigation() -> ViewNavigationContext {
    use_context()
}

/// Create the editor's own navigation signals (view, race, mode, unit, search, all
/// seeded from the entry URL) and a fresh, closed search-typing session, then assemble
/// the [`ViewNavigationContext`] over them plus the already-provided collision and
/// resolve selections and the shell's route `dispatch`, provide it as context, and
/// hand it back. The `dispatch` callback is the sole seam to the concrete `Route`, so
/// it is created in the shell and passed in — this layer stays route-agnostic. The
/// shell calls this once on boot; everything reads the result through
/// [`use_view_navigation`].
pub(crate) fn use_view_navigation_provider(
    initial_view: AppView,
    navigation: DecodedEditorNavigation,
    collision_selection: CollisionSelection,
    resolve_selection: ResolveSelection,
    dispatch: Callback<NavigationCommand>,
) -> ViewNavigationContext {
    let initial_race = navigation.race();
    let initial_mode = navigation.unit_mode();
    let initial_unit_id = navigation.selected_unit_id();
    let initial_search = navigation.search_query().to_owned();
    let current_view = use_signal::<AppView>(move || initial_view);
    let active_race = use_signal::<Race>(move || initial_race);
    let unit_mode = use_signal::<UnitMode>(move || initial_mode);
    let selected_unit_id = use_signal::<Option<WarcraftObjectId>>(move || initial_unit_id);
    let search_query = use_signal::<String>(move || initial_search);
    let search_session_active = use_signal::<bool>(|| false);
    let search_session_generation = use_signal::<u32>(|| 0);
    let search_session = SearchSession::new(search_session_active, search_session_generation);
    let editor_navigation = EditorNavigationSignals::new(
        current_view,
        active_race,
        unit_mode,
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
