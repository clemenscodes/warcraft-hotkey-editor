use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

const CROSS_COLLISIONS: &str = "Cross Collisions";
const INTRA_COLLISIONS: &str = "Intra Collisions";
const HOTKEY_COLLISIONS: &str = "Hotkey Collisions";

pub(crate) struct CollisionBreadcrumbsInputs {
    pub(crate) active_kind: CollisionKind,
    pub(crate) position_count: usize,
    pub(crate) unit_position_count: usize,
    pub(crate) hotkey_count: usize,
    pub(crate) view_navigation: ViewNavigationContext,
}

impl From<CollisionBreadcrumbsInputs> for Vec<BreadcrumbView> {
    fn from(inputs: CollisionBreadcrumbsInputs) -> Self {
        let CollisionBreadcrumbsInputs {
            active_kind,
            position_count,
            unit_position_count,
            hotkey_count,
            view_navigation,
        } = inputs;

        let positions_input = CollisionBreadcrumb {
            kind: CollisionKind::Positions,
            label: CROSS_COLLISIONS,
            count: position_count,
            active_kind,
            view_navigation,
        };
        let positions = BreadcrumbView::from(positions_input);

        let unit_positions_input = CollisionBreadcrumb {
            kind: CollisionKind::UnitPositions,
            label: INTRA_COLLISIONS,
            count: unit_position_count,
            active_kind,
            view_navigation,
        };
        let unit_positions = BreadcrumbView::from(unit_positions_input);

        let hotkeys_input = CollisionBreadcrumb {
            kind: CollisionKind::Hotkeys,
            label: HOTKEY_COLLISIONS,
            count: hotkey_count,
            active_kind,
            view_navigation,
        };
        let hotkeys = BreadcrumbView::from(hotkeys_input);

        vec![positions, unit_positions, hotkeys]
    }
}

struct CollisionBreadcrumb {
    kind: CollisionKind,
    label: &'static str,
    count: usize,
    active_kind: CollisionKind,
    view_navigation: ViewNavigationContext,
}

impl From<CollisionBreadcrumb> for BreadcrumbView {
    fn from(input: CollisionBreadcrumb) -> Self {
        let CollisionBreadcrumb {
            kind,
            label,
            count,
            active_kind,
            view_navigation,
        } = input;
        let active = active_kind == kind;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            let target = AppView::Collisions { kind };
            view_navigation.apply(target);
        });
        let label = label.to_owned();
        Self {
            label,
            count,
            active,
            onclick,
        }
    }
}
