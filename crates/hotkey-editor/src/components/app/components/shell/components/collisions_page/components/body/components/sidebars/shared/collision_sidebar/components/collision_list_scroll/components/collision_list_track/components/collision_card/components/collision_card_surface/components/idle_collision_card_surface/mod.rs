mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::components::collision_card_surface::components::shared::collision_card_meta::CollisionCardMeta;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::components::collision_card_surface::components::shared::collision_card_visual::CollisionCardVisual;
use dioxus::prelude::*;
use props::IdleCollisionCardSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;

/// The idle collision card surface: the card button in its idle look, composing the
/// shared visual and meta line. Presentational — the dispatcher renders it.
#[component]
pub fn IdleCollisionCardSurface(props: IdleCollisionCardSurfaceProps) -> Element {
    let onclick = props.onclick;
    let count = props.count;
    let content = props.content;
    let visual_content = content.clone();
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            CollisionCardVisual { content: visual_content }
            CollisionCardMeta { content, count }
        }
    }
}

assert_component!(IdleCollisionCardSurface);
