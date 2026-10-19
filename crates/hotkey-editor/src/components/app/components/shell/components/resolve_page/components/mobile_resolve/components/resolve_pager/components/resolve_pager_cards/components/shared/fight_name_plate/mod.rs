mod model;
mod view;

pub use view::FightNamePlateView;
mod style;
use crate::components::app::components::shell::components::resolve_page::components::mobile_resolve::components::resolve_pager::components::resolve_pager_cards::components::shared::move_name::MoveName;
use crate::components::app::components::shell::components::resolve_page::components::mobile_resolve::components::resolve_pager::components::resolve_pager_cards::components::shared::object_id::ObjectId;
use dioxus::prelude::*;
use model::FightNamePlateModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn FightNamePlate(props: FightNamePlateModel) -> Element {
    let name = props.name;
    let object_id = props.object_id;
    let object_id_value = object_id.value();
    rsx! {
        div {
            class: CLASS,
            MoveName {
                text: name,
                is_link: false,
            }
            ObjectId {
                text: object_id_value,
            }
        }
    }
}

assert_component!(FightNamePlate);
