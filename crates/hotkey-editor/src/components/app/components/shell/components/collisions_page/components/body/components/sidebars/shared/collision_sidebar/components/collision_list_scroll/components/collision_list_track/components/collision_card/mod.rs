pub mod components;
mod props;
pub mod state;
mod style;

use components::collision_card_surface::CollisionCardSurface;
use dioxus::prelude::*;
use props::CollisionCardProps;
pub use state::{CollisionCardContent, CollisionCardData};
use style::CLASS;
use tw_macro::assert_component;

/// The selectable card shared by every collision sidebar: its own button surface plus
/// a leading visual (unit portrait or island mini grid) and a meta line (name and id,
/// or the coordinate) with the collision count. The unit/island shape is chosen by its
/// [`CollisionCardContent`], so the three sidebars all render this one card. A thin
/// identity wrapper (its `collision-card` root class) that nests its own
/// `CollisionCardSurface` button for the look and the fixed gold accent.
#[component]
pub fn CollisionCard(props: CollisionCardProps) -> Element {
    let is_selected = props.is_selected;
    let onclick = props.onclick;
    let count = props.count;
    let content = props.content;
    rsx! {
        div {
            class: CLASS,
            CollisionCardSurface { is_selected, onclick, count, content }
        }
    }
}

assert_component!(CollisionCard);
