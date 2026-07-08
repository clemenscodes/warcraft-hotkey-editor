pub mod components;
mod props;
mod style;
use components::conflict_detail_unit_icon::{ConflictDetailUnitIcon, ConflictDetailUnitIconProps};
use dioxus::prelude::*;
pub use props::ConflictDetailUnitProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictDetailUnit);
/// The clickable unit portrait in the detail header (deep-links into the editor).
#[component]
pub fn ConflictDetailUnit(props: ConflictDetailUnitProps) -> Element {
    let onclick = props.onclick;
    let name = props.name;
    let icon = ConflictDetailUnitIconProps {
        src: props.icon_url,
        alt: name,
    };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            ConflictDetailUnitIcon { ..icon }
        }
    }
}
