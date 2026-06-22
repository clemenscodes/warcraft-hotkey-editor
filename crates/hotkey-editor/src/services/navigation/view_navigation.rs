use dioxus::prelude::*;
use warcraft_api::{Race, WarcraftObjectMeta};
use warcraft_database::{ObjectLookup, UnitMode};

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
        UrlNavigationState::push_view_to_url(race, mode, unit_id_ref, query_str, target, None);
    }

    /// Deep-link into the editor focused on `unit_id`.  Resolves the
    /// unit's race and mode from the database when possible so the
    /// editor opens on the right race/mode tab, selects the unit, and
    /// switches to the editor view.  When the unit cannot be resolved
    /// it falls back to selecting the id alone — the editor detail still
    /// shows the unit regardless.
    pub(crate) fn open_unit(self, unit_id: &str) {
        let object_option = ObjectLookup::by_id(unit_id);
        if let Some(object) = object_option {
            if let Some(race) = object.race() {
                let mut active_race = self.active_race;
                active_race.set(race);
            }
            if let WarcraftObjectMeta::Unit(unit_meta) = object.meta() {
                let resolved_mode = if unit_meta.is_campaign() {
                    UnitMode::Campaign
                } else {
                    UnitMode::Melee
                };
                let mut unit_mode = self.unit_mode;
                unit_mode.set(resolved_mode);
            }
        }
        let owned_unit_id = unit_id.to_owned();
        let mut selected_unit_id = self.selected_unit_id;
        selected_unit_id.set(Some(owned_unit_id));
        self.apply(AppView::Editor);
    }
}
