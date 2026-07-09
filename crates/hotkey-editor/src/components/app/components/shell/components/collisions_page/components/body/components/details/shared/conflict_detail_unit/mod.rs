pub mod components;
mod props;
mod style;
use crate::services::navigation::context::use_view_navigation;
use components::conflict_detail_unit_icon::{ConflictDetailUnitIcon, ConflictDetailUnitIconProps};
use dioxus::prelude::*;
pub use props::ConflictDetailUnitProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictDetailUnit);
/// The clickable unit portrait in the detail header. It deep-links into the editor
/// focused on its unit through the navigation read from context.
#[component]
pub fn ConflictDetailUnit(props: ConflictDetailUnitProps) -> Element {
    let unit_id = props.unit_id;
    let name = props.name;
    let icon = ConflictDetailUnitIconProps {
        src: props.icon_url,
        alt: name,
    };
    let view_navigation = use_view_navigation();
    let onclick = move |_event: MouseEvent| view_navigation.open_unit(unit_id);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            ConflictDetailUnitIcon { ..icon }
        }
    }
}
