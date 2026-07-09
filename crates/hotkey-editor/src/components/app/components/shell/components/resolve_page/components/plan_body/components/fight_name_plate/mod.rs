mod props;
mod style;
use super::shared::move_name::MoveName;
use super::shared::object_id::ObjectId;
use dioxus::prelude::*;
pub use props::FightNamePlateProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FightNamePlate);
#[component]
pub fn FightNamePlate(props: FightNamePlateProps) -> Element {
    let name = props.name;
    let object_id = props.object_id;
    rsx! {
        div {
            class: CLASS,
            MoveName { text: name, is_link: false }
            ObjectId { text: object_id.value() }
        }
    }
}
