use crate::services::storage::local_storage::LocalStorage;
use warcraft_keybinds::GridLayout;

const CUSTOM_KEYS_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.custom-keys");
const GRID_LAYOUT_STORAGE: LocalStorage = LocalStorage::new("warcraft-hotkey-editor.grid-layout");

const UPDATE_HOTKEYS_ON_MOVE_STORAGE: LocalStorage =
    LocalStorage::new("warcraft-hotkey-editor.update-hotkeys-on-move");

const ONBOARDING_SEEN_STORAGE: LocalStorage =
    LocalStorage::new("warcraft-hotkey-editor.onboarding-seen");

const ONBOARDING_SEEN_VALUE: &str = "true";

pub(crate) struct CustomKeysPersistence;

impl CustomKeysPersistence {
    pub(crate) fn load_text() -> Option<String> {
        CUSTOM_KEYS_STORAGE.get()
    }

    pub(crate) fn save_text(text: &str) {
        CUSTOM_KEYS_STORAGE.set(text);
    }

    pub(crate) fn load_grid_layout() -> Option<GridLayout> {
        let raw_value = GRID_LAYOUT_STORAGE.get()?;
        GridLayout::try_from(raw_value.as_str()).ok()
    }

    pub(crate) fn save_grid_layout(layout: GridLayout) {
        let contents = layout.to_storage_string();
        GRID_LAYOUT_STORAGE.set(&contents);
    }

    pub(crate) fn load_update_hotkeys_on_move() -> bool {
        let stored = UPDATE_HOTKEYS_ON_MOVE_STORAGE.get();
        Self::update_hotkeys_on_move_from_stored(stored)
    }

    pub(crate) fn save_update_hotkeys_on_move(enabled: bool) {
        let value = if enabled { "true" } else { "false" };
        UPDATE_HOTKEYS_ON_MOVE_STORAGE.set(value);
    }

    fn update_hotkeys_on_move_from_stored(stored: Option<String>) -> bool {
        let stored_value = stored.as_deref();
        stored_value != Some("false")
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct OnboardingPersistence;

impl OnboardingPersistence {
    pub(crate) fn has_been_seen() -> bool {
        let stored = ONBOARDING_SEEN_STORAGE.get();
        Self::seen_from_stored(stored)
    }

    pub(crate) fn mark_seen() {
        ONBOARDING_SEEN_STORAGE.set(ONBOARDING_SEEN_VALUE);
    }

    fn seen_from_stored(stored: Option<String>) -> bool {
        let stored_value = stored.as_deref();
        stored_value == Some(ONBOARDING_SEEN_VALUE)
    }
}

#[cfg(test)]
mod onboarding_tests {
    use super::OnboardingPersistence;

    #[test]
    fn absent_value_is_not_seen() {
        let stored = None;
        let result = OnboardingPersistence::seen_from_stored(stored);
        assert!(!result);
    }

    #[test]
    fn exact_true_value_is_seen() {
        let stored = Some(String::from("true"));
        let result = OnboardingPersistence::seen_from_stored(stored);
        assert!(result);
    }

    #[test]
    fn other_values_are_not_seen() {
        let stored = Some(String::from("false"));
        let result = OnboardingPersistence::seen_from_stored(stored);
        assert!(!result);
    }
}

#[cfg(test)]
mod update_hotkeys_on_move_tests {
    use super::CustomKeysPersistence;

    #[test]
    fn absent_value_defaults_to_enabled() {
        let stored = None;
        let result = CustomKeysPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(result);
    }

    #[test]
    fn explicit_false_is_disabled() {
        let stored = Some(String::from("false"));
        let result = CustomKeysPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(!result);
    }

    #[test]
    fn explicit_true_is_enabled() {
        let stored = Some(String::from("true"));
        let result = CustomKeysPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(result);
    }
}
