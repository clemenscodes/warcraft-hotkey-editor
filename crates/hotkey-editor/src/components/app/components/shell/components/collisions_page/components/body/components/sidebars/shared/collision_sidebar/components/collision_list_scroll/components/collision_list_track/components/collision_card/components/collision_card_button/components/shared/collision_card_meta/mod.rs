mod model;
mod view;

pub use view::CollisionCardMetaView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::collision_count::CollisionCount;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::components::collision_card_button::components::shared::collision_card_primary::CollisionCardPrimary;
use dioxus::prelude::*;
use model::CollisionCardMetaModel;
use style::CLASS;
use tw_macro::assert_component;

/// The text column of a collision card: the primary meta line above the collision count.
#[component]
pub fn CollisionCardMeta(props: CollisionCardMetaModel) -> Element {
    let count = props.count;
    let content = props.content;
    rsx! {
        div {
            class: CLASS,
            CollisionCardPrimary { content }
            CollisionCount { count }
        }
    }
}

assert_component!(CollisionCardMeta);
