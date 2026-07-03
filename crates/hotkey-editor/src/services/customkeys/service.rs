use ddd::Adapter;
use ddd::ApplicationLayer;
use ddd::ApplicationService;
use ddd::InfrastructureLayer;
use ddd::Layered;
use ddd::Repository;
use ddd::Service;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::HotkeyTarget;
use warcraft_keybinds::HotkeyToken;
use warcraft_keybinds::MoveRequest;

use crate::services::customkeys::persistence::CustomKeysPersistence;

/// Infrastructure adapter that persists the [`CustomKeys`] aggregate to
/// localStorage. Its `save` funnels through [`CustomKeys::normalized_text`], so
/// only normalized text can ever reach storage (architecture rule R2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomKeysRepository;

impl Layered for CustomKeysRepository {
    type Layer = InfrastructureLayer;
}

impl Adapter for CustomKeysRepository {}

impl Repository<CustomKeys> for CustomKeysRepository {
    fn load(&self) -> Option<CustomKeys> {
        let stored_text = CustomKeysPersistence::load_text()?;
        let parsed_keys = CustomKeys::from_text(stored_text.as_str());
        Some(parsed_keys)
    }

    fn save(&self, aggregate: &CustomKeys) {
        let canonical_text = aggregate.to_string();
        CustomKeysPersistence::save_text(&canonical_text);
    }
}

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
        self.commit(|keys| keys.apply_grid_to_all_bindings(layout))
    }

    pub fn override_hotkey(&self, target: HotkeyTarget, token: Option<HotkeyToken>) {
        self.commit(|keys| keys.set_hotkey(target, token));
    }

    pub fn move_slot(&self, request: &MoveRequest<'_>) {
        self.commit(|keys| keys.move_slot(request));
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
        let read_guard = self.keys.read();
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
