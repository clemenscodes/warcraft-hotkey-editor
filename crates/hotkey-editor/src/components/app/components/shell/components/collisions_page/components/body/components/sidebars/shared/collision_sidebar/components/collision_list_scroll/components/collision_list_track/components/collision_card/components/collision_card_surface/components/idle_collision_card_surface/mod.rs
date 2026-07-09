mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::components::collision_card_surface::components::shared::collision_card_meta::{CollisionCardMeta, CollisionCardMetaProps};
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::collision_sidebar::components::collision_list_scroll::components::collision_list_track::components::collision_card::components::collision_card_surface::components::shared::collision_card_visual::{CollisionCardVisual, CollisionCardVisualProps};
use dioxus::prelude::*;
pub use props::IdleCollisionCardSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IdleCollisionCardSurface);

/// The idle collision card surface: the card button in its idle look, composing the
/// shared visual and meta line. Presentational — the dispatcher renders it.
#[component]
pub fn IdleCollisionCardSurface(props: IdleCollisionCardSurfaceProps) -> Element {
    let visual = CollisionCardVisualProps::from(&props);
    let meta = CollisionCardMetaProps::from(&props);
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            CollisionCardVisual { ..visual }
            CollisionCardMeta { ..meta }
        }
    }
}
