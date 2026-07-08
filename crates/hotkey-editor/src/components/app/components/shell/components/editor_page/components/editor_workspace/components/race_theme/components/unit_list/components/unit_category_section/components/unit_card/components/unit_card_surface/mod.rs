pub mod components;
mod props;
mod style;

use components::unit_card_icon::{UnitCardIcon, UnitCardIconProps};
use components::unit_card_info::{UnitCardInfo, UnitCardInfoProps};
use dioxus::prelude::*;
pub use props::UnitCardSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCardSurface);

/// The unit card's selectable `button`: the portrait beside the name/id, wearing the
/// shared entity-card look and the generic `--race-color` accent. Click and
/// Space/Enter select the unit; the mount handler registers the button with the focus
/// coordinator. Presentational — its accent comes from the `--race-color` an ancestor
/// publishes, so the gallery can render it under any race theme and every state falls
/// out.
#[component]
pub fn UnitCardSurface(props: UnitCardSurfaceProps) -> Element {
    let icon = UnitCardIconProps::from(&props);
    let info = UnitCardInfoProps::from(&props);
    let is_selected = props.is_selected;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    let onmounted = props.onmounted;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-selected": is_selected,
            onclick,
            onkeydown,
            onmounted,
            UnitCardIcon { ..icon }
            UnitCardInfo { ..info }
        }
    }
}
