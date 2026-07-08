pub mod components;
mod props;
mod style;

use components::collision_card_meta::{CollisionCardMeta, CollisionCardMetaProps};
use components::collision_card_visual::{CollisionCardVisual, CollisionCardVisualProps};
use dioxus::prelude::*;
pub use props::CollisionCardSurfaceProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CollisionCardSurface);

/// The collision card's selectable `button`: the leading visual beside the meta line
/// and count, wearing the shared entity-card look and the fixed collision-gold accent.
/// Presentational — the selected flag and click handler arrive as props, so the
/// gallery renders it directly and every state falls out.
#[component]
pub fn CollisionCardSurface(props: CollisionCardSurfaceProps) -> Element {
    let visual = CollisionCardVisualProps::from(&props);
    let meta = CollisionCardMetaProps::from(&props);
    let is_selected = props.is_selected;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-selected": is_selected,
            onclick,
            CollisionCardVisual { ..visual }
            CollisionCardMeta { ..meta }
        }
    }
}
