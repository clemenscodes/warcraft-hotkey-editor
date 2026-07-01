mod props;
mod style;
use super::resolve_move_name::ResolveMoveName;
use super::resolve_object_id::ResolveObjectId;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveFightNameBtnProps;
use style::CLASS;
assert_component!(ResolveFightNameBtn);
#[component]
pub fn ResolveFightNameBtn(props: ResolveFightNameBtnProps) -> Element {
    let name = props.name;
    let object_id = props.object_id;
    let has_unit = props.has_unit;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled: !has_unit,
            onclick,
            ResolveMoveName { text: name, is_link: has_unit }
            ResolveObjectId { text: object_id }
        }
    }
}
