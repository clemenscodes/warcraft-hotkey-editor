mod props;
mod view;

pub use view::HotkeysContentView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetail;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::unit_cards_sidebar::UnitCardsSidebar;
use dioxus::prelude::*;
use props::HotkeysContentProps;
use style::CLASS;
use tw_macro::assert_component;

/// The shared-hotkey two-pane content: the clashing-units sidebar column beside the
/// fluid hotkey unit detail pane.
#[component]
pub fn HotkeysContent(props: HotkeysContentProps) -> Element {
    let sidebar_units = props.units.clone();
    let detail_units = props.units;
    rsx! {
        div {
            class: CLASS,
            UnitCardsSidebar { units: sidebar_units }
            HotkeyUnitDetail { units: detail_units }
        }
    }
}

assert_component!(HotkeysContent);
