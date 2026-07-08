use crate::components::app::components::shell::components::collisions_page::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetailProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebarProps;
use crate::components::app::components::shell::components::collisions_page::logic::HotkeyConflictView;
use crate::services::navigation::app_view::CollisionKind;
use dioxus::prelude::*;

/// The shared-hotkey two-pane content: the clashing-units sidebar beside the hotkey
/// unit detail pane, tagged with the fixed `hotkeys` kind slug and the conflict count
/// for the e2e hooks.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeysContentProps {
    pub count: usize,
    pub sidebar: UnitCardsSidebarProps<HotkeyConflictView>,
    pub detail: HotkeyUnitDetailProps,
}

/// The kind slug and count resolved for the two-pane wrapper's data attributes.
pub(super) struct HotkeysContentPresentation {
    pub collision_kind: &'static str,
    pub count: usize,
}

impl From<&HotkeysContentProps> for HotkeysContentPresentation {
    fn from(props: &HotkeysContentProps) -> Self {
        let kind = CollisionKind::Hotkeys;
        let collision_kind = kind.kind_param();
        let count = props.count;
        Self {
            collision_kind,
            count,
        }
    }
}
