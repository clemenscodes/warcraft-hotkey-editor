mod model;
mod view;

pub use view::SelectedUnitCardSurfaceView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_icon::UnitCardIcon;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_info::UnitCardInfo;
use dioxus::prelude::*;
use model::SelectedUnitCardSurfaceModel;
use style::CLASS;
use tw_macro::assert_component;

/// The selected unit card surface: the card button in its selected look, composing the shared
/// portrait and text column. Presentational — the dispatcher renders it.
#[component]
pub fn SelectedUnitCardSurface(props: SelectedUnitCardSurfaceModel) -> Element {
    let icon_path = props.icon_path.clone();
    let display_name = props.display_name.clone();
    let icon_display_name = display_name.clone();
    let unit_id = props.unit_id;
    let is_selected = true;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            onkeydown,
            UnitCardIcon {
                icon_path,
                display_name: icon_display_name,
            }
            UnitCardInfo {
                display_name,
                unit_id,
                is_selected,
            }
        }
    }
}

assert_component!(SelectedUnitCardSurface);
