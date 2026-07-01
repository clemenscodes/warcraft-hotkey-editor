use super::props::CollisionBreadcrumbsProps;
use crate::components::views::collisions_page::collision_breadcrumb::CollisionBreadcrumbProps;
use crate::services::navigation::app_view::CollisionKind;

/// The three breadcrumb tabs, in display order, with their active flags resolved.
pub(super) struct CollisionBreadcrumbsModel {
    pub(super) positions: CollisionBreadcrumbProps,
    pub(super) unit_positions: CollisionBreadcrumbProps,
    pub(super) hotkeys: CollisionBreadcrumbProps,
}

impl From<&CollisionBreadcrumbsProps> for CollisionBreadcrumbsModel {
    fn from(props: &CollisionBreadcrumbsProps) -> Self {
        let view_navigation = props.view_navigation;
        let positions = CollisionBreadcrumbProps {
            label: "Cross Collisions".to_owned(),
            count: props.position_count,
            target_kind: CollisionKind::Positions,
            data_breadcrumb: "positions",
            active: matches!(props.kind, CollisionKind::Positions),
            view_navigation,
        };
        let unit_positions = CollisionBreadcrumbProps {
            label: "Intra Collisions".to_owned(),
            count: props.unit_position_count,
            target_kind: CollisionKind::UnitPositions,
            data_breadcrumb: "unit-positions",
            active: matches!(props.kind, CollisionKind::UnitPositions),
            view_navigation,
        };
        let hotkeys = CollisionBreadcrumbProps {
            label: "Hotkey Collisions".to_owned(),
            count: props.hotkey_count,
            target_kind: CollisionKind::Hotkeys,
            data_breadcrumb: "hotkeys",
            active: matches!(props.kind, CollisionKind::Hotkeys),
            view_navigation,
        };
        Self {
            positions,
            unit_positions,
            hotkeys,
        }
    }
}
