use super::super::super::logic::UnitPositionDetailData;
use super::components::unit_position_conflict_grid::components::unit_position_conflict_card::UnitPositionConflictCardProps;
use super::components::unit_position_conflict_grid::UnitPositionConflictGridProps;
use super::components::unit_position_detail_header::UnitPositionDetailHeaderProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_detail_unit::ConflictDetailUnitProps;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The populated position-collision detail pane: the selected unit's header data and its
/// position-conflict cards.
#[derive(Props, Clone, PartialEq)]
pub struct FilledUnitPositionDetailProps {
    pub unit: ConflictDetailUnitProps,
    pub name: String,
    pub unit_id: WarcraftObjectId,
    pub count: usize,
    pub cards: Vec<UnitPositionConflictCardProps>,
}

impl From<&UnitPositionDetailData> for FilledUnitPositionDetailProps {
    fn from(data: &UnitPositionDetailData) -> Self {
        let unit = data.unit.clone();
        let name = data.name.clone();
        let unit_id = data.unit_id;
        let count = data.count;
        let cards = data.cards.clone();
        Self {
            unit,
            name,
            unit_id,
            count,
            cards,
        }
    }
}

impl From<&FilledUnitPositionDetailProps> for UnitPositionDetailHeaderProps {
    fn from(props: &FilledUnitPositionDetailProps) -> Self {
        let unit = props.unit.clone();
        let name = props.name.clone();
        let unit_id = props.unit_id;
        let count = props.count;
        Self {
            unit,
            name,
            unit_id,
            count,
        }
    }
}

impl From<&FilledUnitPositionDetailProps> for UnitPositionConflictGridProps {
    fn from(props: &FilledUnitPositionDetailProps) -> Self {
        let cards = props.cards.clone();
        Self { cards }
    }
}
