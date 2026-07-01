use super::components::header_actions::HeaderActionsProps;
use super::components::header_brand::HeaderBrandProps;
use super::props::HeaderProps;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The two child prop bundles the header places, shaped once: the brand (with its
/// click handler already wired) and the action cluster. The centered layout slot
/// takes no props (it reads overlay state from context), so it is not built here.
pub struct HeaderView {
    pub brand: HeaderBrandProps,
    pub actions: HeaderActionsProps,
}

/// The composed hook: reads the app-wide navigation context, wires the brand's
/// click handler from it, and builds the action cluster's prop bundle. Navigation
/// and overlay state are app-wide context the header does not own — the header
/// only forwards the editor state its action cluster needs.
pub fn use_header(props: &HeaderProps) -> HeaderView {
    let loaded_keys = props.loaded_keys;
    let upload_status = props.upload_status;
    let grid_layout = props.grid_layout;
    let navigation = use_context::<ViewNavigationContext>();
    let on_home = EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Editor));
    let brand = HeaderBrandProps { onclick: on_home };
    let actions = HeaderActionsProps {
        loaded_keys,
        upload_status,
        grid_layout,
    };
    HeaderView { brand, actions }
}
