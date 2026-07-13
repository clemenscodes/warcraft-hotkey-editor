use ddd::ApplicationLayer;
use ddd::ApplicationService;
use ddd::Layered;
use ddd::QueryHandler;
use ddd::Repository;
use ddd::Service;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::HotkeyTarget;
use warcraft_keybinds::HotkeyToken;
use warcraft_keybinds::ImportOutcome;
use warcraft_keybinds::KeyCode;
use warcraft_keybinds::MoveRequest;
use warcraft_keybinds::WarcraftObjectId;

use crate::persistence::custom_keys_persistence;
use crate::repository::custom_keys_repository::CustomKeysRepository;
use crate::services::customkeys::commands::apply_grid_layout_command::ApplyGridLayoutCommand;
use crate::services::customkeys::commands::move_slot_command::MoveSlotCommand;
use crate::services::customkeys::commands::resolve_conflicts_command::ResolveConflictsCommand;
use crate::services::customkeys::commands::set_hotkey_command::SetHotkeyCommand;
use crate::services::customkeys::commands::set_system_hotkey_command::SetSystemHotkeyCommand;
use crate::services::customkeys::commands::swap_system_bindings_command::SwapSystemBindingsCommand;
use crate::services::customkeys::queries::collision_summary_query::CollisionSummaryQuery;
use crate::services::customkeys::queries::cross_unit_collisions_query::CrossUnitCollisionsQuery;
use crate::services::customkeys::queries::resolve_preview_query::ResolvePreviewQuery;
use crate::services::customkeys::queries::slot_binding_query::SlotBindingQuery;
use crate::services::customkeys::queries::slot_binding_query::SlotBindingView;
use crate::services::customkeys::queries::unit_collisions_query::UnitCollisionsQuery;
use warcraft_keybinds::CascadePlan;
use warcraft_keybinds::CollisionSummary;
use warcraft_keybinds::CrossUnitCollisionReport;
use warcraft_keybinds::UnitCollisionReport;

/// The application-layer service that owns the live [`CustomKeys`] aggregate and
/// is the only sanctioned way for the renderer to mutate it. Every command runs
/// through [`Service::commit`], which write-throughs to the repository before
/// updating the live signal, so localStorage never trails the edit.
#[derive(Clone, Copy)]
pub struct CustomKeysService {
    keys: Signal<Option<CustomKeys>>,
}

impl CustomKeysService {
    pub fn new(keys: Signal<Option<CustomKeys>>) -> Self {
        Self { keys }
    }

    /// A read-only, reactive view of the live aggregate for the renderer. Reads
    /// subscribe the caller; there is no write access — mutations go only through
    /// the command methods, so the wall holds for reads as well as writes.
    pub fn keys(&self) -> ReadSignal<Option<CustomKeys>> {
        self.keys.into()
    }

    pub fn apply_grid_layout(&self, layout: GridLayout) -> usize {
        let command = ApplyGridLayoutCommand::new(layout);
        self.dispatch(command)
    }

    pub fn override_hotkey(&self, target: HotkeyTarget, token: Option<HotkeyToken>) {
        let command = SetHotkeyCommand::new(target, token);
        self.dispatch(command);
    }

    pub fn move_slot(&self, request: &MoveRequest<'_>) {
        let owned_request = *request;
        let command = MoveSlotCommand::new(owned_request);
        self.dispatch(command);
    }

    /// Set a single system keybind's hotkey, re-normalizing and persisting through the
    /// commit boundary. The renderer never writes the aggregate directly.
    pub fn set_system_hotkey(&self, section_id: WarcraftObjectId, code: KeyCode) {
        let command = SetSystemHotkeyCommand::new(section_id, code);
        self.dispatch(command);
    }

    /// Exchange two system keybinds' hotkeys (the inventory drag-to-swap gesture),
    /// re-normalizing and persisting through the commit boundary.
    pub fn swap_system_bindings(&self, source_id: WarcraftObjectId, target_id: WarcraftObjectId) {
        let command = SwapSystemBindingsCommand::new(source_id, target_id);
        self.dispatch(command);
    }

    /// Run the position cascade that resolves every outstanding collision, re-normalizing
    /// and persisting through the commit boundary. Returns the resulting [`CascadePlan`] so
    /// the resolve page can report how many slots moved and how many could not be placed.
    /// The renderer never runs the cascade against the aggregate itself.
    pub fn resolve_conflicts(&self) -> CascadePlan {
        let command = ResolveConflictsCommand;
        self.dispatch(command)
    }

    /// The resolved binding + conflict picture for one system keybind section (the
    /// read side). Reactive: reading it subscribes the caller, so a slot re-renders
    /// when the keys change. The renderer asks here instead of building the binding
    /// map or resolving collisions itself.
    pub fn slot_binding(&self, section_id: WarcraftObjectId) -> SlotBindingView {
        let query = SlotBindingQuery::new(section_id);
        self.handle(query)
    }

    /// The cross-unit position-collision report (the read side). Reactive: reading it
    /// subscribes the caller. The collisions page asks here instead of running the
    /// report itself at render time.
    pub fn cross_unit_collisions(&self) -> CrossUnitCollisionReport {
        let query = CrossUnitCollisionsQuery;
        self.handle(query)
    }

    /// The per-unit collision report for a grid layout (the read side). Reactive on
    /// the aggregate; the layout is supplied by the caller from the grid-layout
    /// service.
    pub fn unit_collisions(&self, layout: GridLayout) -> UnitCollisionReport {
        let query = UnitCollisionsQuery::new(layout);
        self.handle(query)
    }

    /// The cascade plan a resolve would produce — a read-only preview (the read
    /// side). Reactive on the aggregate; the resolve page previews the plan here
    /// instead of running the cascade at render time.
    pub fn resolve_preview(&self) -> CascadePlan {
        let query = ResolvePreviewQuery;
        self.handle(query)
    }

    /// The collision-count summary the toolbar badge shows, for a grid layout (the
    /// read side). Reactive on the aggregate; the badge asks here instead of running
    /// the collision reports itself at render time.
    pub fn collision_summary(&self, layout: GridLayout) -> CollisionSummary {
        let query = CollisionSummaryQuery::new(layout);
        self.handle(query)
    }

    /// The exact stored `CustomKeys.txt` text (R5: export and preview ARE the stored
    /// localStorage text, nothing more — no re-serialize, no re-normalize; the stored
    /// text is already normalized per R2). Reads the aggregate first so a reactive
    /// caller re-reads on every mutation, then returns the authoritative stored text.
    pub fn exported_text(&self) -> String {
        let _subscribe = self.keys.read();
        let stored_text = custom_keys_persistence::load_text();
        stored_text.unwrap_or_default()
    }

    /// The sanctioned import command: overlays the uploaded text onto the baseline
    /// through the domain (rule R7, "imports replace, then normalize"), writes the
    /// normalized result through to storage, and returns the outcome so the caller
    /// can report how much was imported. An upload reaches the aggregate only here;
    /// the renderer never sets the keys signal itself.
    pub fn import_overlay(&self, overlay_text: &str) -> ImportOutcome {
        let outcome = CustomKeys::import_overlay(overlay_text);
        let imported_outcome = outcome.clone();
        let imported_keys = imported_outcome.into_keys();
        let repository = self.repository();
        repository.save(&imported_keys);
        self.replace(imported_keys);
        outcome
    }
}

impl Layered for CustomKeysService {
    type Layer = ApplicationLayer;
}

impl ApplicationService for CustomKeysService {}

impl QueryHandler<SlotBindingQuery> for CustomKeysService {
    /// Reads the live aggregate reactively (so the caller re-renders on change)
    /// and answers the query against that snapshot.
    fn handle(&self, query: SlotBindingQuery) -> SlotBindingView {
        let read_guard = self.keys.read();
        let custom_keys = read_guard.as_ref();
        query.answer(custom_keys)
    }
}

impl QueryHandler<CrossUnitCollisionsQuery> for CustomKeysService {
    fn handle(&self, query: CrossUnitCollisionsQuery) -> CrossUnitCollisionReport {
        let read_guard = self.keys.read();
        let custom_keys = read_guard.as_ref();
        query.answer(custom_keys)
    }
}

impl QueryHandler<UnitCollisionsQuery> for CustomKeysService {
    fn handle(&self, query: UnitCollisionsQuery) -> UnitCollisionReport {
        let read_guard = self.keys.read();
        let custom_keys = read_guard.as_ref();
        query.answer(custom_keys)
    }
}

impl QueryHandler<ResolvePreviewQuery> for CustomKeysService {
    fn handle(&self, query: ResolvePreviewQuery) -> CascadePlan {
        let read_guard = self.keys.read();
        let custom_keys = read_guard.as_ref();
        query.answer(custom_keys)
    }
}

impl QueryHandler<CollisionSummaryQuery> for CustomKeysService {
    fn handle(&self, query: CollisionSummaryQuery) -> CollisionSummary {
        let read_guard = self.keys.read();
        let custom_keys = read_guard.as_ref();
        query.answer(custom_keys)
    }
}

impl Service<CustomKeys> for CustomKeysService {
    type Repository = CustomKeysRepository;

    fn repository(&self) -> Self::Repository {
        CustomKeysRepository
    }

    fn snapshot(&self) -> CustomKeys {
        let read_guard = self.keys.peek();
        read_guard.clone().unwrap_or_default()
    }

    fn replace(&self, aggregate: CustomKeys) {
        let mut keys_signal = self.keys;
        keys_signal.set(Some(aggregate));
    }

    /// Overridden to restore the aggregate invariant: after the mutation runs the
    /// aggregate is re-normalized before it is persisted and stored back, so
    /// every state the renderer can observe is normalized. This is the single
    /// write-through boundary for `CustomKeys`.
    fn commit<Outcome>(&self, change: impl FnOnce(&mut CustomKeys) -> Outcome) -> Outcome {
        let mut aggregate = self.snapshot();
        let outcome = change(&mut aggregate);
        let normalized = aggregate.normalize();
        let repository = self.repository();
        repository.save(&normalized);
        self.replace(normalized);
        outcome
    }
}
