use ddd::ApplicationLayer;
use ddd::ApplicationService;
use ddd::Layered;
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

use crate::repository::custom_keys_repository::CustomKeysRepository;
use crate::services::customkeys::commands::apply_grid_layout::ApplyGridLayout;
use crate::services::customkeys::commands::move_slot::MoveSlot;
use crate::services::customkeys::commands::set_hotkey::SetHotkey;
use crate::services::customkeys::commands::set_system_hotkey::SetSystemHotkey;
use crate::services::customkeys::commands::swap_system_bindings::SwapSystemBindings;

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
        let command = ApplyGridLayout::new(layout);
        self.dispatch(command)
    }

    pub fn override_hotkey(&self, target: HotkeyTarget, token: Option<HotkeyToken>) {
        let command = SetHotkey::new(target, token);
        self.dispatch(command);
    }

    pub fn move_slot(&self, request: &MoveRequest<'_>) {
        let owned_request = *request;
        let command = MoveSlot::new(owned_request);
        self.dispatch(command);
    }

    /// Set a single system keybind's hotkey, re-normalizing and persisting through the
    /// commit boundary. The renderer never writes the aggregate directly.
    pub fn set_system_hotkey(&self, section_id: &str, code: KeyCode) {
        let owned_section_id = section_id.to_string();
        let command = SetSystemHotkey::new(owned_section_id, code);
        self.dispatch(command);
    }

    /// Exchange two system keybinds' hotkeys (the inventory drag-to-swap gesture),
    /// re-normalizing and persisting through the commit boundary.
    pub fn swap_system_bindings(&self, source_id: &str, target_id: &str) {
        let owned_source_id = source_id.to_string();
        let owned_target_id = target_id.to_string();
        let command = SwapSystemBindings::new(owned_source_id, owned_target_id);
        self.dispatch(command);
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
