use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

pub(super) fn use_brand() -> EventHandler<MouseEvent> {
    let navigation = use_view_navigation();
    EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Editor))
}
