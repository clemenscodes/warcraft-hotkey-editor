use warcraft_api::SystemHotkeysCategory;

/// The hero-selection editor's shaped content: the intro caption the domain supplies for
/// the Hero Selection category.
pub(super) struct HeroSelectionHotkeysViewModel {
    pub(super) caption: &'static str,
}

/// Sources the Hero Selection category's intro caption from the domain, so the renderer
/// never hardcodes the copy.
pub(super) fn use_hero_selection_hotkeys_view() -> HeroSelectionHotkeysViewModel {
    let caption = SystemHotkeysCategory::HeroSelection
        .caption()
        .unwrap_or_default();
    HeroSelectionHotkeysViewModel { caption }
}
