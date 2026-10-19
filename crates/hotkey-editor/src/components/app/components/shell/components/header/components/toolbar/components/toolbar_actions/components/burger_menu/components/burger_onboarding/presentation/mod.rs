use crate::persistence::onboarding_persistence::OnboardingPersistence;
use crate::services::viewport::use_is_pager_viewport;
use dioxus::prelude::*;

pub(super) struct BurgerOnboardingPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

// Below the desktop band the toolbar's help button is hidden and the burger
// drawer is mounted only while the menu is open, so neither can show the guide
// on a first visit. This opens it there once, on its own, outside the drawer.
pub(super) fn use_burger_onboarding() -> BurgerOnboardingPresentation {
    let pager_viewport = use_is_pager_viewport();
    let mut open_signal =
        use_signal::<bool>(move || pager_viewport && !OnboardingPersistence::has_been_seen());
    let open = open_signal();
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    BurgerOnboardingPresentation {
        open,
        on_open_change,
    }
}
