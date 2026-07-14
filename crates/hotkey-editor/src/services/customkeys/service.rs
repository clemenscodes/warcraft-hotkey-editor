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

#[derive(Clone, Copy)]
pub struct CustomKeysService {
    keys: Signal<Option<CustomKeys>>,
}

impl CustomKeysService {
    pub fn new(keys: Signal<Option<CustomKeys>>) -> Self {
        Self { keys }
    }

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

    pub fn set_system_hotkey(&self, section_id: WarcraftObjectId, code: KeyCode) {
        let command = SetSystemHotkeyCommand::new(section_id, code);
        self.dispatch(command);
    }

    pub fn swap_system_bindings(&self, source_id: WarcraftObjectId, target_id: WarcraftObjectId) {
        let command = SwapSystemBindingsCommand::new(source_id, target_id);
        self.dispatch(command);
    }

    pub fn resolve_conflicts(&self) -> CascadePlan {
        let command = ResolveConflictsCommand;
        self.dispatch(command)
    }

    pub fn slot_binding(&self, section_id: WarcraftObjectId) -> SlotBindingView {
        let query = SlotBindingQuery::new(section_id);
        self.handle(query)
    }

    pub fn cross_unit_collisions(&self) -> CrossUnitCollisionReport {
        let query = CrossUnitCollisionsQuery;
        self.handle(query)
    }

    pub fn unit_collisions(&self, layout: GridLayout) -> UnitCollisionReport {
        let query = UnitCollisionsQuery::new(layout);
        self.handle(query)
    }

    pub fn resolve_preview(&self) -> CascadePlan {
        let query = ResolvePreviewQuery;
        self.handle(query)
    }

    pub fn collision_summary(&self, layout: GridLayout) -> CollisionSummary {
        let query = CollisionSummaryQuery::new(layout);
        self.handle(query)
    }

    pub fn exported_text(&self) -> String {
        let _subscribe = self.keys.read();
        let stored_text = custom_keys_persistence::load_text();
        stored_text.unwrap_or_default()
    }

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
