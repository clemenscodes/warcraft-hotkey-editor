use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;

use crate::services::navigation::app_view::AppView;
use crate::services::navigation::url_state::UrlNavigationState;

/// Bundles every signal the header needs to write when the user
/// switches views (clicks the brand to go home, or clicks the
/// Collisions icon to land on `?view=collisions`).  Exposed as a
/// `Copy` struct so onclick closures can capture it cheaply and call
/// `apply` to dispatch the navigation.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct ViewNavigationContext {
    pub(crate) current_view: Signal<AppView>,
    pub(crate) active_race: Signal<Race>,
    pub(crate) unit_mode: Signal<UnitMode>,
    pub(crate) selected_unit_id: Signal<Option<String>>,
    pub(crate) search_query: Signal<String>,
}

impl ViewNavigationContext {
    /// Switch to `target` and push a history entry so browser
    /// back/forward navigates between views.  No-op when `target`
    /// already matches the current view.
    pub(crate) fn apply(self, target: AppView) {
        let mut current_view = self.current_view;
        if target == *current_view.read() {
            return;
        }
        current_view.set(target);
        let race = *self.active_race.read();
        let mode = *self.unit_mode.read();
        let unit_id_option = self.selected_unit_id.read().clone();
        let query = self.search_query.read().clone();
        let unit_id_ref = unit_id_option.as_deref();
        let query_str = query.as_str();
        UrlNavigationState::push_view_to_url(race, mode, unit_id_ref, query_str, target);
    }
}
