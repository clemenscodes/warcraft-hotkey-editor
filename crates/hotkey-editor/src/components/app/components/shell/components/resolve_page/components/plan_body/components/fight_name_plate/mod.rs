mod props;
mod view;

pub use view::FightNamePlateView;
mod style;
use super::shared::move_name::MoveName;
use super::shared::object_id::ObjectId;
use dioxus::prelude::*;
use props::FightNamePlateProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn FightNamePlate(props: FightNamePlateProps) -> Element {
    let name = props.name;
    let object_id = props.object_id;
    let object_id_value = object_id.value();
    rsx! {
        div {
            class: CLASS,
            MoveName { text: name, is_link: false }
            ObjectId { text: object_id_value }
        }
    }
}

assert_component!(FightNamePlate);
