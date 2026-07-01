pub mod components;
mod hooks;
mod props;
mod style;

use crate::assert_component;
use components::header_actions::HeaderActions;
use components::header_brand::HeaderBrand;
use components::header_layout_slot::HeaderLayoutSlot;
use dioxus::prelude::*;
use hooks::{HeaderView, use_header};
pub use props::HeaderProps;
use style::CLASS;
assert_component!(Header);

/// The app's top chrome: brand on the left, the global grid-layout button
/// centered, and the action cluster on the right. It switches between a compact
/// (burger) layout below 1280px and the full three-column layout at and above it.
#[component]
pub fn Header(props: HeaderProps) -> Element {
    let HeaderView { brand, actions } = use_header(&props);
    rsx! {
        header { class: CLASS,
            HeaderBrand { ..brand }
            HeaderLayoutSlot {}
            HeaderActions { ..actions }
        }
    }
}
