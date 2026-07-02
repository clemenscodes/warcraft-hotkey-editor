use super::props::BreadcrumbsProps;
use crate::components::views::collisions_page::components::breadcrumbs::components::breadcrumb::BreadcrumbProps;
use crate::services::navigation::app_view::CollisionKind;

/// The three breadcrumb tabs, in display order, with their active flags resolved.
pub(super) struct BreadcrumbsModel {
    pub(super) positions: BreadcrumbProps,
    pub(super) unit_positions: BreadcrumbProps,
    pub(super) hotkeys: BreadcrumbProps,
}

impl From<&BreadcrumbsProps> for BreadcrumbsModel {
    fn from(props: &BreadcrumbsProps) -> Self {
        let view_navigation = props.view_navigation;
        let positions = BreadcrumbProps {
            label: "Cross Collisions".to_owned(),
            count: props.position_count,
            target_kind: CollisionKind::Positions,
            data_breadcrumb: "positions",
            active: matches!(props.kind, CollisionKind::Positions),
            view_navigation,
        };
        let unit_positions = BreadcrumbProps {
            label: "Intra Collisions".to_owned(),
            count: props.unit_position_count,
            target_kind: CollisionKind::UnitPositions,
            data_breadcrumb: "unit-positions",
            active: matches!(props.kind, CollisionKind::UnitPositions),
            view_navigation,
        };
        let hotkeys = BreadcrumbProps {
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
