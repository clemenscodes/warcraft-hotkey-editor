use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::shared::icons::ICON_RESOLVE;
use crate::components::shell::header::{TOOLBAR_BTN_CLASS, TOOLBAR_ICON_CLASS};
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct ResolveButtonProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) navigation: ViewNavigationContext,
}

/// Toolbar button that navigates to the Resolve page, where the cascade plan is
/// previewed and applied. (Replaces the old confirm dialog.)
#[component]
pub(crate) fn ResolveButton(props: ResolveButtonProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let navigation = props.navigation;
    let has_loaded_file = loaded_keys.read().is_some();

    let go_to_resolve = move |_| {
        navigation.apply(AppView::Resolve);
    };

    rsx! {
        button {
            class: TOOLBAR_BTN_CLASS,
            r#type: "button",
            aria_label: "Resolve conflicts",
            "data-action": "view-resolve",
            disabled: !has_loaded_file,
            onclick: go_to_resolve,
            span {
                class: TOOLBAR_ICON_CLASS,
                aria_hidden: "true",
                dangerous_inner_html: ICON_RESOLVE,
            }
        }
    }
}
