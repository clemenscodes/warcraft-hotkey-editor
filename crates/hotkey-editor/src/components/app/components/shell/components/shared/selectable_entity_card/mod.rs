mod props;
pub mod state;
mod style;

use dioxus::prelude::*;
pub use props::SelectableEntityCardProps;
pub use state::CardAccent;
use tw_macro::assert_component;
assert_component!(SelectableEntityCard);

/// The selectable card surface shared by the editor unit list and the collision
/// sidebars: the `button` shell that carries the resting look, the hover and
/// keyboard-focus treatment, the accent per selected state, and the mobile/tablet
/// carousel interior. It is presentational — the accent, selected flag, and handlers
/// arrive as props, and the leading visual plus meta line arrive as `children`, so
/// each card is a thin wrapper that only picks the accent and nests its content.
#[component]
pub fn SelectableEntityCard(props: SelectableEntityCardProps) -> Element {
    let is_selected = props.is_selected;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    let onmounted = props.onmounted;
    let children = props.children;
    let class = style::class(props.accent);
    rsx! {
        button {
            class,
            r#type: "button",
            "data-selected": is_selected,
            onclick,
            onkeydown: move |event: KeyboardEvent| {
                if let Some(handler) = onkeydown {
                    handler.call(event);
                }
            },
            onmounted: move |event: Event<MountedData>| {
                if let Some(handler) = onmounted {
                    handler.call(event);
                }
            },
            {children}
        }
    }
}
