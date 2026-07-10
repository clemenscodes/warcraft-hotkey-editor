mod props;
mod style;
use tw_macro::assert_component;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::shared::move_name::MoveName;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::shared::object_id::ObjectId;
use dioxus::prelude::*;
pub use props::FightNameButtonProps;
use style::CLASS;
assert_component!(FightNameButton);
#[component]
pub fn FightNameButton(props: FightNameButtonProps) -> Element {
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
            MoveName { text: name, is_link: has_unit }
            ObjectId { text: object_id.value() }
        }
    }
}
