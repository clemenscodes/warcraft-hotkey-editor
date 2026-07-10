mod props;
mod view;

pub use view::FightNameButtonView;
mod style;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::shared::move_name::MoveName;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::shared::object_id::ObjectId;
use dioxus::prelude::*;
use props::FightNameButtonProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn FightNameButton(props: FightNameButtonProps) -> Element {
    let name = props.name;
    let object_id = props.object_id;
    let object_id_value = object_id.value();
    let has_unit = props.has_unit;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled: !has_unit,
            onclick,
            MoveName { text: name, is_link: has_unit }
            ObjectId { text: object_id_value }
        }
    }
}

assert_component!(FightNameButton);
