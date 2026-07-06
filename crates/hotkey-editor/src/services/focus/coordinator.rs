use dioxus::prelude::*;
use std::rc::Rc;

/// The regions a keyboard activation can hand focus on to. Which concrete element is
/// "the active race tab" or "the selected unit card" is decided by application state
/// (the active-race and selected-unit signals), never re-derived from the DOM.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum FocusTarget {
    RaceTabs,
    UnitCard,
    OverrideKey,
}

/// A keyboard-only, state-driven focus hand-off. A source (a keydown handler) records
/// where focus should land next; the element that application state marks as that
/// region's target registers its own mounted handle; a single driver effect focuses
/// that handle once both are present. This replaces the old
/// `document.querySelector("[data-active='true']")` round-trip: the app already knows
/// which element is active, so it never asks the DOM.
#[derive(Clone, Copy)]
pub(crate) struct FocusCoordinator {
    intent: Signal<Option<FocusTarget>>,
    race_tabs_handle: Signal<Option<Rc<MountedData>>>,
    unit_card_handle: Signal<Option<Rc<MountedData>>>,
    override_key_handle: Signal<Option<Rc<MountedData>>>,
}

impl FocusCoordinator {
    /// Build the coordinator's signals. Call once at the shell root and provide the
    /// result as context (mirrors `UndoHistory::use_history`).
    pub(crate) fn use_coordinator() -> Self {
        let intent = use_signal(|| None);
        let race_tabs_handle = use_signal(|| None);
        let unit_card_handle = use_signal(|| None);
        let override_key_handle = use_signal(|| None);
        Self {
            intent,
            race_tabs_handle,
            unit_card_handle,
            override_key_handle,
        }
    }

    /// A keydown handler records where focus should land after the activation renders.
    pub(crate) fn request(self, target: FocusTarget) {
        let mut intent = self.intent;
        intent.set(Some(target));
    }

    /// The active race tab registers (or clears) its own mounted handle.
    pub(crate) fn set_race_tabs_handle(self, handle: Option<Rc<MountedData>>) {
        let mut slot = self.race_tabs_handle;
        slot.set(handle);
    }

    /// The unit card the list marks as its focus target registers its mounted handle.
    pub(crate) fn set_unit_card_handle(self, handle: Option<Rc<MountedData>>) {
        let mut slot = self.unit_card_handle;
        slot.set(handle);
    }

    /// The override key cell registers its mounted handle while it is on screen.
    pub(crate) fn set_override_key_handle(self, handle: Option<Rc<MountedData>>) {
        let mut slot = self.override_key_handle;
        slot.set(handle);
    }

    /// The single driver: when an intent is pending and its target's handle is
    /// registered, focus that handle and clear the intent. It re-runs whenever the
    /// intent or a handle changes, so a target that mounts *after* the request (a race
    /// switch re-renders the unit list) still receives focus once it registers.
    pub(crate) fn drive(self) {
        use_effect(move || {
            let Some(target) = *self.intent.read() else {
                return;
            };
            let handle = match target {
                FocusTarget::RaceTabs => self.race_tabs_handle.read().clone(),
                FocusTarget::UnitCard => self.unit_card_handle.read().clone(),
                FocusTarget::OverrideKey => self.override_key_handle.read().clone(),
            };
            let Some(handle) = handle else {
                return;
            };
            let mut intent = self.intent;
            intent.set(None);
            spawn(async move {
                let _ = handle.set_focus(true).await;
            });
        });
    }
}
