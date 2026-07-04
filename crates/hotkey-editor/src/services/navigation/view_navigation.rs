use crate::services::navigation::app_view::AppView;
use crate::services::navigation::editor_nav::DecodedEditorNav;
use dioxus::prelude::*;
use warcraft_api::{Race, WarcraftObjectMeta};
use warcraft_database::{ObjectLookup, UnitMode};

/// Bundles every signal the header needs to write when the user
/// switches views (clicks the brand to go home, or clicks the
/// Collisions icon to land on `?view=collisions`).  Exposed as a
/// `Copy` struct so onclick closures can capture it cheaply and call
/// `apply` to dispatch the navigation.
#[derive(Clone, Copy, PartialEq)]
pub struct ViewNavigationContext {
    pub current_view: Signal<AppView>,
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub search_query: Signal<String>,
}

impl ViewNavigationContext {
    /// Switch to `target`. No-op when `target` already matches the current view.
    /// Setting the view signal is all that is needed: the shell's URL-sync effect
    /// observes the change and pushes the matching route through the router (so
    /// browser back/forward navigates between views).
    pub fn apply(self, target: AppView) {
        let mut current_view = self.current_view;
        if target == *current_view.read() {
            return;
        }
        current_view.set(target);
    }

    /// Reconcile the current view alone, without touching the editor selection. The
    /// collisions and resolve pages call this: their route carries no race/mode/unit/
    /// search (that is the editor's state, which persists untouched in these signals
    /// while another page is shown), so they only announce which page is now active.
    pub fn restore_view(self, view: AppView) {
        let mut current_view = self.current_view;
        if *current_view.peek() != view {
            current_view.set(view);
        }
    }

    /// Reconcile the editor route into these navigation signals: set the current view
    /// and the decoded editor race/mode/unit/search. Each field is set only when it
    /// actually changes, so restoring a route the signals already match (the common
    /// case, since the shell's own push wrote it) triggers no needless re-render. This
    /// is the read side of the shell's URL contract for the editor page — the mounted
    /// page owns it, so the current view always reflects the live route.
    pub fn restore(self, view: AppView, nav: &DecodedEditorNav) {
        self.restore_view(view);
        let mut active_race = self.active_race;
        if *active_race.peek() != nav.race {
            active_race.set(nav.race);
        }
        let mut unit_mode = self.unit_mode;
        if *unit_mode.peek() != nav.unit_mode {
            unit_mode.set(nav.unit_mode);
        }
        let mut selected_unit_id = self.selected_unit_id;
        if *selected_unit_id.peek() != nav.selected_unit_id {
            selected_unit_id.set(nav.selected_unit_id.clone());
        }
        let mut search_query = self.search_query;
        if *search_query.peek() != nav.search_query {
            search_query.set(nav.search_query.clone());
        }
    }

    /// Deep-link into the editor focused on `unit_id`.  Resolves the
    /// unit's race and mode from the database when possible so the
    /// editor opens on the right race/mode tab, selects the unit, and
    /// switches to the editor view.  When the unit cannot be resolved
    /// it falls back to selecting the id alone — the editor detail still
    /// shows the unit regardless.
    pub fn open_unit(self, unit_id: &str) {
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
