use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::shared::icons::ICON_RESOLVE;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub struct ResolveButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
}

/// Toolbar button that navigates to the Resolve page, where the cascade plan is
/// previewed and applied. (Replaces the old confirm dialog.)
#[component]
pub fn ResolveButton(props: ResolveButtonProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let navigation = use_context::<ViewNavigationContext>();
    let has_loaded_file = loaded_keys.read().is_some();
    let resolve_disabled = !has_loaded_file;

    let go_to_resolve = move |_| {
        navigation.apply(AppView::Resolve);
    };

    rsx! {
        ToolbarButton {
            icon: ICON_RESOLVE,
            aria_label: "Resolve conflicts",
            "data-action": "view-resolve",
            disabled: resolve_disabled,
            onclick: go_to_resolve,
        }
    }
}
