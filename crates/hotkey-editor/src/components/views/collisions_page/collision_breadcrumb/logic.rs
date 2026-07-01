use super::props::CollisionBreadcrumbProps;
use crate::services::navigation::app_view::AppView;
use dioxus::prelude::*;

/// The breadcrumb's derived interaction: where it navigates and its aria state.
pub(super) struct CollisionBreadcrumbModel {
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) aria_current: &'static str,
}

impl From<&CollisionBreadcrumbProps> for CollisionBreadcrumbModel {
    fn from(props: &CollisionBreadcrumbProps) -> Self {
        let view_navigation = props.view_navigation;
        let target_kind = props.target_kind;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            let target = AppView::Collisions { kind: target_kind };
            view_navigation.apply(target);
        });
        let aria_current = if props.active { "page" } else { "false" };
        Self {
            onclick,
            aria_current,
        }
    }
}
