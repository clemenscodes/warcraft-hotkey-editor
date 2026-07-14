use warcraft_api::SystemHotkeysCategory;

pub(super) struct HeroSelectionHotkeysViewModel {
    pub(super) caption: &'static str,
}

pub(super) fn use_hero_selection_hotkeys_view() -> HeroSelectionHotkeysViewModel {
    let caption = SystemHotkeysCategory::HeroSelection
        .caption()
        .unwrap_or_default();
    HeroSelectionHotkeysViewModel { caption }
}
