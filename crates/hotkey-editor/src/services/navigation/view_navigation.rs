use super::default_unit::DefaultUnit;
use crate::services::collision_selection::CollisionSelection;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::editor_navigation::DecodedEditorNavigation;
use crate::services::navigation::navigation_command::{NavigationCommand, NavigationHistoryMode};
use crate::services::navigation::navigation_snapshot::NavigationSnapshot;
use crate::services::navigation::search_session::SearchSession;
use crate::services::resolve_selection::ResolveSelection;
use dioxus::prelude::*;
use warcraft_api::{Race, WarcraftObjectId, WarcraftObjectMeta};
use warcraft_api::{UnitMode, WarcraftApi};
use warcraft_keybinds::GridSlotId;

#[derive(Clone, Copy, PartialEq)]
pub struct EditorNavigationSignals {
    current_view: Signal<AppView>,
    active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    selected_unit_id: Signal<Option<WarcraftObjectId>>,
    search_query: Signal<String>,
}

impl EditorNavigationSignals {
    pub fn new(
        current_view: Signal<AppView>,
        active_race: Signal<Race>,
        unit_mode: Signal<UnitMode>,
        selected_unit_id: Signal<Option<WarcraftObjectId>>,
        search_query: Signal<String>,
    ) -> Self {
        Self {
            current_view,
            active_race,
            unit_mode,
            selected_unit_id,
            search_query,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ViewNavigationContext {
    current_view: Signal<AppView>,
    active_race: Signal<Race>,
    unit_mode: Signal<UnitMode>,
    selected_unit_id: Signal<Option<WarcraftObjectId>>,
    search_query: Signal<String>,
    collision_selection: CollisionSelection,
    resolve_selection: ResolveSelection,
    search_session: SearchSession,
    dispatch: Callback<NavigationCommand>,
}

impl ViewNavigationContext {
    pub fn new(
        editor: EditorNavigationSignals,
        collision_selection: CollisionSelection,
        resolve_selection: ResolveSelection,
        search_session: SearchSession,
        dispatch: Callback<NavigationCommand>,
    ) -> Self {
        let EditorNavigationSignals {
            current_view,
            active_race,
            unit_mode,
            selected_unit_id,
            search_query,
        } = editor;
        Self {
            current_view,
            active_race,
            unit_mode,
            selected_unit_id,
            search_query,
            collision_selection,
            resolve_selection,
            search_session,
            dispatch,
        }
    }

    pub fn current_view(&self) -> Signal<AppView> {
        self.current_view
    }

    pub fn active_race(&self) -> Signal<Race> {
        self.active_race
    }

    pub fn unit_mode(&self) -> Signal<UnitMode> {
        self.unit_mode
    }

    pub fn selected_unit_id(&self) -> Signal<Option<WarcraftObjectId>> {
        self.selected_unit_id
    }

    pub fn search_query(&self) -> Signal<String> {
        self.search_query
    }

    fn snapshot(&self, view: AppView) -> NavigationSnapshot {
        match view {
            AppView::Editor => {
                let race = *self.active_race.peek();
                let unit_mode = *self.unit_mode.peek();
                let selected_unit_id = *self.selected_unit_id.peek();
                let search_query = self.search_query.peek().clone();
                let navigation =
                    DecodedEditorNavigation::new(race, unit_mode, selected_unit_id, search_query);
                NavigationSnapshot::Editor(navigation)
            }
            AppView::Collisions { kind } => {
                let selected = self.collision_entry_signal(kind);
                let entry = selected.peek().clone();
                NavigationSnapshot::Collisions { kind, entry }
            }
            AppView::Resolve => {
                let selected = self.resolve_selection.selected_move_category();
                let entry = selected.peek().clone();
                NavigationSnapshot::Resolve { entry }
            }
        }
    }

    fn collision_entry_signal(&self, kind: CollisionKind) -> Signal<Option<String>> {
        match kind {
            CollisionKind::Positions => self.collision_selection.selected_island(),
            CollisionKind::Hotkeys => self.collision_selection.selected_hotkey_unit(),
            CollisionKind::UnitPositions => self.collision_selection.selected_unit_position(),
        }
    }

    fn push(&self, snapshot: NavigationSnapshot) {
        let command = NavigationCommand::new(snapshot, NavigationHistoryMode::Push);
        self.dispatch.call(command);
    }

    fn replace(&self, snapshot: NavigationSnapshot) {
        let command = NavigationCommand::new(snapshot, NavigationHistoryMode::Replace);
        self.dispatch.call(command);
    }

    pub fn apply(self, target: AppView) {
        if target == *self.current_view.peek() {
            return;
        }
        let snapshot = self.snapshot(target);
        self.push(snapshot);
    }

    pub fn select_race(self, race: Race, mut selected_slot: Signal<Option<GridSlotId>>) {
        selected_slot.set(None);
        let unit_mode = *self.unit_mode.peek();
        let default_unit = DefaultUnit::new(race, unit_mode);
        let next_unit = default_unit.resolve();
        let search_query = self.search_query.peek().clone();
        let navigation = DecodedEditorNavigation::new(race, unit_mode, next_unit, search_query);
        let snapshot = NavigationSnapshot::Editor(navigation);
        self.push(snapshot);
    }

    pub fn select_mode(self, mode: UnitMode, mut selected_slot: Signal<Option<GridSlotId>>) {
        selected_slot.set(None);
        let race = *self.active_race.peek();
        let default_unit = DefaultUnit::new(race, mode);
        let next_unit = default_unit.resolve();
        let search_query = self.search_query.peek().clone();
        let navigation = DecodedEditorNavigation::new(race, mode, next_unit, search_query);
        let snapshot = NavigationSnapshot::Editor(navigation);
        self.push(snapshot);
    }

    pub fn open_unit(self, unit_id: WarcraftObjectId) {
        let api = WarcraftApi::default();
        let object_option = api.object(unit_id);
        let mut race = *self.active_race.peek();
        let mut unit_mode = *self.unit_mode.peek();
        if let Some(object) = object_option {
            if let Some(object_race) = object.race() {
                race = object_race;
            }
            if let WarcraftObjectMeta::Unit(unit_meta) = object.meta() {
                unit_mode = if unit_meta.is_campaign() {
                    UnitMode::Campaign
                } else {
                    UnitMode::Melee
                };
            }
        }
        let search_query = self.search_query.peek().clone();
        let selected_unit = Some(unit_id);
        let navigation = DecodedEditorNavigation::new(race, unit_mode, selected_unit, search_query);
        let snapshot = NavigationSnapshot::Editor(navigation);
        self.push(snapshot);
    }

    pub fn select_unit(self, unit_id: WarcraftObjectId) {
        let race = *self.active_race.peek();
        let unit_mode = *self.unit_mode.peek();
        let search_query = self.search_query.peek().clone();
        let selected_unit = Some(unit_id);
        let navigation = DecodedEditorNavigation::new(race, unit_mode, selected_unit, search_query);
        let snapshot = NavigationSnapshot::Editor(navigation);
        self.push(snapshot);
    }

    pub fn select_collision_entry(self, entry: String) {
        let AppView::Collisions { kind } = *self.current_view.peek() else {
            return;
        };
        let selected_entry = Some(entry);
        let snapshot = NavigationSnapshot::Collisions {
            kind,
            entry: selected_entry,
        };
        self.replace(snapshot);
    }

    pub fn select_move_category(self, slug: String) {
        let selected_entry = Some(slug);
        let snapshot = NavigationSnapshot::Resolve {
            entry: selected_entry,
        };
        self.replace(snapshot);
    }

    pub fn set_search_query(self, value: String) {
        let race = *self.active_race.peek();
        let unit_mode = *self.unit_mode.peek();
        let selected_unit_id = *self.selected_unit_id.peek();
        let navigation = DecodedEditorNavigation::new(race, unit_mode, selected_unit_id, value);
        let snapshot = NavigationSnapshot::Editor(navigation);
        let mut session_active = self.search_session.active();
        let mut session_generation = self.search_session.generation();
        let session_was_open = *session_active.peek();
        if session_was_open {
            self.replace(snapshot);
        } else {
            self.push(snapshot);
            session_active.set(true);
        }
        let next_generation = session_generation.peek().wrapping_add(1);
        session_generation.set(next_generation);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(500).await;
            if *session_generation.peek() == next_generation {
                session_active.set(false);
            }
        });
    }

    pub fn restore_view(self, view: AppView) {
        let mut current_view = self.current_view;
        if *current_view.peek() != view {
            current_view.set(view);
        }
    }

    pub fn restore(self, view: AppView, navigation: &DecodedEditorNavigation) {
        self.restore_view(view);
        let mut active_race = self.active_race;
        if *active_race.peek() != navigation.race() {
            active_race.set(navigation.race());
        }
        let mut unit_mode = self.unit_mode;
        if *unit_mode.peek() != navigation.unit_mode() {
            unit_mode.set(navigation.unit_mode());
        }
        let mut selected_unit_id = self.selected_unit_id;
        if *selected_unit_id.peek() != navigation.selected_unit_id() {
            selected_unit_id.set(navigation.selected_unit_id());
        }
        let mut search_query = self.search_query;
        if *search_query.peek() != navigation.search_query() {
            search_query.set(navigation.search_query().to_owned());
        }
    }
}
