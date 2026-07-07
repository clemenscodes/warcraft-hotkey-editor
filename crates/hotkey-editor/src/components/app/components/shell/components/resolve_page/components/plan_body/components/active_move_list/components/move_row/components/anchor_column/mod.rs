mod props;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_column::FightColumn;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlate;
use dioxus::prelude::*;
pub use props::{AnchorColumnProps, AnchorParts};

/// The rival column of a move card, shown only when the move has an anchor
/// (Fight/Swap): the rival's name plate over its icon. Renders nothing otherwise.
use tw_macro::assert_component;
assert_component!(AnchorColumn);
#[component]
pub fn AnchorColumn(props: AnchorColumnProps) -> Element {
    let Some(anchor) = props.anchor else {
        return rsx! {};
    };
    let name_plate = anchor.name_plate;
    let ability = anchor.ability;
    rsx! {
        FightColumn {
            FightNamePlate { ..name_plate }
            AbilityIcon { ..ability }
        }
    }
}
