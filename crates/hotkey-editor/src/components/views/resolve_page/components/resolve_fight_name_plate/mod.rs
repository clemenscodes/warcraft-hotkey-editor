mod props;
mod style;
use super::resolve_move_name::ResolveMoveName;
use super::resolve_object_id::ResolveObjectId;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveFightNamePlateProps;
use style::CLASS;
assert_component!(ResolveFightNamePlate);
#[component]
pub fn ResolveFightNamePlate(props: ResolveFightNamePlateProps) -> Element {
    let name = props.name;
    let object_id = props.object_id;
    rsx! {
        div {
            class: CLASS,
            ResolveMoveName { text: name, is_link: false }
            ResolveObjectId { text: object_id }
        }
    }
}
