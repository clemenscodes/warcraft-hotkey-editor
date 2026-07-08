mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebar;
use dioxus::prelude::*;
use props::HotkeysContentPresentation;
pub use props::HotkeysContentProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeysContent);

/// The shared-hotkey two-pane content: the clashing-units sidebar column beside the
/// fluid hotkey unit detail pane.
#[component]
pub fn HotkeysContent(props: HotkeysContentProps) -> Element {
    let HotkeysContentPresentation {
        collision_kind,
        count,
    } = HotkeysContentPresentation::from(&props);
    let sidebar = props.sidebar;
    let detail = props.detail;
    rsx! {
        div {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "{count}",
            UnitCardsSidebar { ..sidebar }
            HotkeyUnitDetail { ..detail }
        }
    }
}
