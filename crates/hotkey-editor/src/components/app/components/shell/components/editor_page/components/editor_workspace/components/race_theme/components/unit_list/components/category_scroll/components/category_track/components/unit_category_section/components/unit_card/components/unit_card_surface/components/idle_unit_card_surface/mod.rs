mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_icon::{UnitCardIcon, UnitCardIconProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_info::{UnitCardInfo, UnitCardInfoProps};
use dioxus::prelude::*;
pub use props::IdleUnitCardSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IdleUnitCardSurface);

/// The idle unit card surface: the card button in its idle look, composing the shared
/// portrait and text column. Presentational — the dispatcher renders it.
#[component]
pub fn IdleUnitCardSurface(props: IdleUnitCardSurfaceProps) -> Element {
    let icon = UnitCardIconProps::from(&props);
    let info = UnitCardInfoProps::from(&props);
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    let onmounted = props.onmounted;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            onkeydown,
            onmounted,
            UnitCardIcon { ..icon }
            UnitCardInfo { ..info }
        }
    }
}
