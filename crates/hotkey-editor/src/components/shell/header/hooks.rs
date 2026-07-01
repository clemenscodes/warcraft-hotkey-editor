use dioxus::prelude::*;

use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;

use super::components::header_actions::HeaderActionsProps;
use super::components::header_brand::HeaderBrandProps;
use super::components::header_layout_slot::HeaderLayoutSlotProps;
use super::props::HeaderProps;

/// The three child prop bundles the header places, shaped once: the brand (with
/// its click handler already wired), the centered layout slot, and the action
/// cluster. The body only names this and spreads it.
pub struct HeaderView {
    pub brand: HeaderBrandProps,
    pub layout_slot: HeaderLayoutSlotProps,
    pub actions: HeaderActionsProps,
}

/// The composed hook: derives the navigation context, wires the brand's click
/// handler, and builds each child prop bundle. The dialog open signals are owned
/// by the app root (like every other dialog) and threaded in as props.
pub fn use_header(props: &HeaderProps) -> HeaderView {
    let loaded_keys = props.loaded_keys;
    let upload_status = props.upload_status;
    let preview_open = props.preview_open;
    let grid_layout = props.grid_layout;
    let system_hotkeys_open = props.system_hotkeys_open;
    let help_open = props.help_open;
    let layout_dialog_open = props.layout_dialog_open;
    let templates_dialog_open = props.templates_dialog_open;
    let navigation = ViewNavigationContext::from(props);

    let on_home = EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Editor));
    let brand = HeaderBrandProps { onclick: on_home };
    let layout_slot = HeaderLayoutSlotProps { layout_dialog_open };
    let actions = HeaderActionsProps {
        loaded_keys,
        upload_status,
        preview_open,
        grid_layout,
        layout_dialog_open,
        templates_dialog_open,
        system_hotkeys_open,
        help_open,
        navigation,
    };

    HeaderView {
        brand,
        layout_slot,
        actions,
    }
}
