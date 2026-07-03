mod props;

use crate::components::shell::header::components::header_actions::components::header_toolbar::components::shared::toolbar_button::{
    ToolbarButton, ToolbarButtonProps,
};
use dioxus::prelude::*;
pub use props::ResolveButtonProps;

/// Toolbar button that navigates to the Resolve page, where the cascade plan is
/// previewed and applied. Presentational: it renders from `disabled` and `onclick`
/// alone and fetches nothing, so the gallery can showcase it with plain values.
/// `ResolveButtonHost` supplies both from context.
#[component]
pub fn ResolveButton(props: ResolveButtonProps) -> Element {
    let button = ToolbarButtonProps::from(&props);
    rsx! {
        ToolbarButton { ..button }
    }
}
