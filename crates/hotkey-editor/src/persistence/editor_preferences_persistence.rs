use browser_kit::storage::LocalStorage;

const UPDATE_HOTKEYS_ON_MOVE_STORAGE: LocalStorage =
    LocalStorage::new("warcraft-hotkey-editor.update-hotkeys-on-move");

pub(crate) struct EditorPreferencesPersistence;

impl EditorPreferencesPersistence {
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

#[cfg(test)]
mod update_hotkeys_on_move_tests {
    use super::EditorPreferencesPersistence;

    #[test]
    fn absent_value_defaults_to_enabled() {
        let stored = None;
        let result = EditorPreferencesPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(result);
    }

    #[test]
    fn explicit_false_is_disabled() {
        let stored = Some(String::from("false"));
        let result = EditorPreferencesPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(!result);
    }

    #[test]
    fn explicit_true_is_enabled() {
        let stored = Some(String::from("true"));
        let result = EditorPreferencesPersistence::update_hotkeys_on_move_from_stored(stored);
        assert!(result);
    }
}
