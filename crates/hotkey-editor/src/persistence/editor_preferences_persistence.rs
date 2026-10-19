use browser_kit::storage::LocalStorage;

const UPDATE_HOTKEYS_ON_MOVE_STORAGE: LocalStorage =
    LocalStorage::new("warcraft-hotkey-editor.update-hotkeys-on-move");

/// Whether a move also rewrites the hotkey, on by default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct UpdateHotkeysOnMovePersistence;

impl UpdateHotkeysOnMovePersistence {
    pub(crate) fn load() -> bool {
        let stored = UPDATE_HOTKEYS_ON_MOVE_STORAGE.get();
        Self::from_stored(stored)
    }

    pub(crate) fn save(enabled: bool) {
        let value = if enabled { "true" } else { "false" };
        UPDATE_HOTKEYS_ON_MOVE_STORAGE.set(value);
    }

    fn from_stored(stored: Option<String>) -> bool {
        let stored_value = stored.as_deref();
        stored_value != Some("false")
    }
}

#[cfg(test)]
mod update_hotkeys_on_move_tests {
    use super::UpdateHotkeysOnMovePersistence;

    #[test]
    fn absent_value_defaults_to_enabled() {
        let stored = None;
        let result = UpdateHotkeysOnMovePersistence::from_stored(stored);
        assert!(result);
    }

    #[test]
    fn explicit_false_is_disabled() {
        let stored = Some(String::from("false"));
        let result = UpdateHotkeysOnMovePersistence::from_stored(stored);
        assert!(!result);
    }

    #[test]
    fn explicit_true_is_enabled() {
        let stored = Some(String::from("true"));
        let result = UpdateHotkeysOnMovePersistence::from_stored(stored);
        assert!(result);
    }
}
