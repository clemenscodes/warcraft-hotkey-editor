use browser_kit::storage::LocalStorage;

const ONBOARDING_SEEN_STORAGE: LocalStorage =
    LocalStorage::new("warcraft-hotkey-editor.onboarding-seen");

const ONBOARDING_SEEN_VALUE: &str = "true";

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
