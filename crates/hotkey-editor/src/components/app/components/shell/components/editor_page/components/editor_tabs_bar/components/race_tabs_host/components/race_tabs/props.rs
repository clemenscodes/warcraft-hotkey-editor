use super::view::RaceTabsView;
use dioxus::prelude::*;
use warcraft_api::Race;

/// The race tabs' only concern: which race is active (to mark the current tab), and a
/// select handler to dispatch when a tab is chosen. Everything the selection *cascades*
/// — the default unit, the slot reset — is the domain's job behind `select_race`, not a
/// bundle of signals threaded through the tabs.
#[derive(Props, Clone, Copy, PartialEq)]
pub struct RaceTabsProps {
    pub active_race: Signal<Race>,
    pub on_select: EventHandler<Race>,
}

/// One race tab's finished binding: whether it is the active tab, its display label, and
/// the pointer/keyboard handlers `RaceTabs` baked from the active race and the select
/// handler. This is plain data — no navigation signal — so every per-race wrapper and
/// the shared dispatcher take it without any context threading. The tab's look (its race
/// colours) is the wrapper's own `style.rs`, never part of this binding.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabBinding {
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&RaceTabsView> for RaceTabsProps {
    fn from(view: &RaceTabsView) -> Self {
        let RaceTabsView {
            active_race,
            on_select,
        } = view.clone();
        Self {
            active_race,
            on_select,
        }
    }
}

impl ddd::Props for RaceTabsProps {
    type View = RaceTabsView;
}
