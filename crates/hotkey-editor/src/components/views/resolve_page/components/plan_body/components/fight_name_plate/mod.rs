mod props;
mod style;
use super::move_name::MoveName;
use super::object_id::ObjectId;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::FightNamePlateProps;
use style::CLASS;
assert_component!(FightNamePlate);
#[component]
pub fn FightNamePlate(props: FightNamePlateProps) -> Element {
    let name = props.name;
    let object_id = props.object_id;
    rsx! {
        div {
            class: CLASS,
            MoveName { text: name, is_link: false }
            ObjectId { text: object_id }
        }
    }
}
