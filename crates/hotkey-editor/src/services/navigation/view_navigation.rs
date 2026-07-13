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

/// The five signals that make up the editor's own navigation state: which page is
/// active, and the editor's race/mode/unit/search selection. Grouped so the shell hands
/// them to [`ViewNavigationContext`] as one value rather than five loose arguments.
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

/// The navigation write/read surface for the whole app.
///
/// **Route is the single source of truth; these signals are a pure read-cache.** The
/// data flows one way: a user gesture builds a typed [`NavigationSnapshot`], wraps it in
/// a [`NavigationCommand`] (with push or replace decided at the gesture), and hands it to
/// the shell-supplied `dispatch` callback, which is the only place that names the
/// concrete `Route`. The router then navigates, each page reconciles the new route back
/// into these signals ([`restore`](Self::restore) / [`restore_view`](Self::restore_view)),
/// and the UI re-renders. There is no signals→route effect and therefore no echo guard:
/// the only writer of the route is an explicit gesture, and reconciling a route never
/// writes one back.
///
/// Exposed as a `Copy` struct so onclick closures capture it cheaply.
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

    /// Build the snapshot the address bar should show for `view`, from the current
    /// signal values (peeked, since this runs inside a gesture and never subscribes).
    /// The editor carries its race/mode/unit/search; a collisions view carries the
    /// active kind's currently-selected entry; the resolve view its selected category.
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

    /// The collision-selection signal that names the selected entry for `kind`.
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

    /// Switch to `target`, pushing a new history entry. No-op when `target` already
    /// matches the current view (so re-clicking the active page adds no history). The
    /// target route reconciles back into the signals when the page mounts.
    pub fn apply(self, target: AppView) {
        if target == *self.current_view.peek() {
            return;
        }
        let snapshot = self.snapshot(target);
        self.push(snapshot);
    }

    /// Select `race`: land on that race's default unit for the current mode and clear the
    /// slot selection, then push the editor route. Which unit is the default is a domain
    /// decision, resolved through [`DefaultUnit`], never computed in the renderer. The
    /// slot reset is pure UI state (not in the URL), so it is set directly; the race and
    /// unit ride into the route and reconcile back into their signals.
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

    /// Select `mode`: land on the current race's default unit for that mode, clear the
    /// slot selection, and push the editor route. Symmetric with [`select_race`](Self::select_race).
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

    /// Deep-link into the editor focused on `unit_id`. Resolves the unit's race and mode
    /// from the database so the editor opens on the right tabs, keeps the current search,
    /// and pushes the editor route.
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

    /// Select `unit_id` within the current race and mode: push the editor route carrying
    /// it, keeping the current race, mode, and search. Which unit is shown is route state,
    /// so it rides into the URL and reconciles back into the signal; the slot reset and
    /// category switch that accompany a pick are pure UI state and stay at the call site.
    /// Symmetric with [`select_race`](Self::select_race) / [`select_mode`](Self::select_mode).
    pub fn select_unit(self, unit_id: WarcraftObjectId) {
        let race = *self.active_race.peek();
        let unit_mode = *self.unit_mode.peek();
        let search_query = self.search_query.peek().clone();
        let selected_unit = Some(unit_id);
        let navigation = DecodedEditorNavigation::new(race, unit_mode, selected_unit, search_query);
        let snapshot = NavigationSnapshot::Editor(navigation);
        self.push(snapshot);
    }

    /// Pick a collisions-list entry: replace the current history entry (picking an entry
    /// must not spam history) with the active kind's route carrying `entry`. The active
    /// kind comes from the current view; on any non-collisions view this is a no-op.
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

    /// Pick a resolve move-category breadcrumb: replace the current history entry with the
    /// resolve route carrying `slug`.
    pub fn select_move_category(self, slug: String) {
        let selected_entry = Some(slug);
        let snapshot = NavigationSnapshot::Resolve {
            entry: selected_entry,
        };
        self.replace(snapshot);
    }

    /// Set the editor search query, running the debounced typing session: the first
    /// keystroke pushes a history entry and opens the session; every keystroke while the
    /// session is open replaces; the session closes 500 ms after the last keystroke. This
    /// is the write side of the search box — the query rides into the editor route and
    /// reconciles back into its signal.
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

    /// Reconcile the editor route into these navigation signals: set the current view and
    /// the decoded editor race/mode/unit/search. Each field is set only when it actually
    /// changes, so restoring a route the signals already match (the common case, since a
    /// gesture's own push wrote it) triggers no needless re-render. This is the read side
    /// of the URL contract for the editor page.
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
