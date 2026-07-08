mod props;
mod state;
mod style;
use dioxus::prelude::*;
pub use props::PanelCardProps;
pub use state::PanelCardVariant;
use style::class;
use tw_macro::assert_component;
assert_component!(PanelCard);

/// The shared card surface behind a plan move card and a collision conflict card.
/// It owns the bordered, tinted panel look; each caller nests its own identity
/// wrapper around it and picks a variant.
#[component]
pub fn PanelCard(props: PanelCardProps) -> Element {
    let class = class(props.variant);
    let children = props.children;
    rsx! {
        div {
            class,
            {children}
        }
    }
}
