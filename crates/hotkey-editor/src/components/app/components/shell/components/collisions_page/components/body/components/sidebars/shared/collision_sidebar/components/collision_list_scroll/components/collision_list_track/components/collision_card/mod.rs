pub mod components;
mod model;
mod view;

pub use view::CollisionCardView;
pub mod state;
mod style;

use components::collision_card_button::CollisionCardButton;
use dioxus::prelude::*;
use model::CollisionCardModel;
pub use state::{CollisionCardContent, CollisionCardData};
use style::CLASS;
use tw_macro::assert_component;

/// The selectable card shared by every collision sidebar: its own button surface plus
/// a leading visual (unit portrait or island mini grid) and a meta line (name and id,
/// or the coordinate) with the collision count. The unit/island shape is chosen by its
/// [`CollisionCardContent`], so the three sidebars all render this one card. A thin
/// identity wrapper (its `collision-card` root class) that nests its own
/// `CollisionCardButton` button for the look and the fixed gold accent.
#[component]
pub fn CollisionCard(props: CollisionCardModel) -> Element {
    let is_selected = props.is_selected;
    let onclick = props.onclick;
    let count = props.count;
    let content = props.content;
    rsx! {
        div {
            class: CLASS,
            CollisionCardButton {
                is_selected,
                onclick,
                count,
                content,
            }
        }
    }
}

assert_component!(CollisionCard);
