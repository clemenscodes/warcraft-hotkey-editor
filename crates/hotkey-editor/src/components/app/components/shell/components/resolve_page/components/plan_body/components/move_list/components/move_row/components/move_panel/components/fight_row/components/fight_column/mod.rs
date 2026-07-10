pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use components::fight_name_button::FightNameButton;
use dioxus::prelude::*;
pub use props::FightColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FightColumn);

/// The mover's fighter column: its name button over its ability icon.
#[component]
pub fn FightColumn(props: FightColumnProps) -> Element {
    let mover_name_btn = props.mover_name_btn;
    let mover_ability = props.mover_ability;
    rsx! {
        div {
            class: CLASS,
            FightNameButton { ..mover_name_btn }
            AbilityIcon { ..mover_ability }
        }
    }
}
