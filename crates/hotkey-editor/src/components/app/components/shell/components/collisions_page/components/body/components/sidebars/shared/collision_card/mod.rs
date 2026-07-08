pub mod components;
mod props;
pub mod state;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::row_meta::RowMeta;
use crate::components::app::components::shell::components::shared::selectable_entity_card::{
    CardAccent, SelectableEntityCard, SelectableEntityCardProps,
};
use components::collision_card_primary::{CollisionCardPrimary, CollisionCardPrimaryProps};
use components::collision_card_visual::{CollisionCardVisual, CollisionCardVisualProps};
use dioxus::prelude::*;
pub use props::CollisionCardProps;
pub use state::CollisionCardContent;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CollisionCard);

/// The selectable card shared by every collision sidebar: the shared entity-card
/// surface plus a leading visual (unit portrait or island mini grid) and a meta line
/// (name and id, or the coordinate) with the collision count. The unit/island shape is
/// chosen by its [`CollisionCardContent`], so the three sidebars all render this one
/// card. A thin identity wrapper that carries the deep-link data attributes and wears
/// the fixed gold accent on the shared surface.
#[component]
pub fn CollisionCard(props: CollisionCardProps) -> Element {
    let is_selected = props.is_selected;
    let collision_key = props.collision_key;
    let onclick = props.onclick;
    let count = props.count;
    let content = props.content;
    let visual = CollisionCardVisualProps {
        content: content.clone(),
    };
    let primary = CollisionCardPrimaryProps { content };
    let children = rsx! {
        CollisionCardVisual { ..visual }
        RowMeta {
            CollisionCardPrimary { ..primary }
            CollisionCount { count }
        }
    };
    let card = SelectableEntityCardProps {
        accent: CardAccent::CollisionGold,
        is_selected,
        onclick,
        onkeydown: None,
        onmounted: None,
        children,
    };
    rsx! {
        div {
            class: CLASS,
            "data-selected": is_selected,
            "data-collision-key": collision_key,
            SelectableEntityCard { ..card }
        }
    }
}
