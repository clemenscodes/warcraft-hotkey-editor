use super::data;
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
        let active_kind = props.kind;

        let positions_kind = CollisionKind::Positions;
        let positions_slug = positions_kind.kind_param();
        let positions = BreadcrumbProps {
            label: data::CROSS_COLLISIONS.to_owned(),
            count: props.position_count,
            target_kind: positions_kind,
            data_breadcrumb: positions_slug,
            active: active_kind == positions_kind,
            view_navigation,
        };

        let unit_positions_kind = CollisionKind::UnitPositions;
        let unit_positions_slug = unit_positions_kind.kind_param();
        let unit_positions = BreadcrumbProps {
            label: data::INTRA_COLLISIONS.to_owned(),
            count: props.unit_position_count,
            target_kind: unit_positions_kind,
            data_breadcrumb: unit_positions_slug,
            active: active_kind == unit_positions_kind,
            view_navigation,
        };

        let hotkeys_kind = CollisionKind::Hotkeys;
        let hotkeys_slug = hotkeys_kind.kind_param();
        let hotkeys = BreadcrumbProps {
            label: data::HOTKEY_COLLISIONS.to_owned(),
            count: props.hotkey_count,
            target_kind: hotkeys_kind,
            data_breadcrumb: hotkeys_slug,
            active: active_kind == hotkeys_kind,
            view_navigation,
        };

        Self {
            positions,
            unit_positions,
            hotkeys,
        }
    }
}
