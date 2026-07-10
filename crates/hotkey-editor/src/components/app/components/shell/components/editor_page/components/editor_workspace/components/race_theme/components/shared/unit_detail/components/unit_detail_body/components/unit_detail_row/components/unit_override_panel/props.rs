use super::components::unit_tile_override::UnitTileOverrideProps;
use crate::components::app::components::shell::components::shared::grid_heading::GridHeadingProps;
use dioxus::prelude::*;

/// The right column holding the hotkey override: the "Hotkey override" heading over the
/// override card. On phones it becomes a sticky bottom sheet, widened and shifted out of
/// the card's padding.
#[derive(Props, Clone, PartialEq)]
pub struct UnitOverridePanelProps {
    pub heading: GridHeadingProps,
    pub tile_override: UnitTileOverrideProps,
}

impl From<&UnitOverridePanelProps> for GridHeadingProps {
    fn from(props: &UnitOverridePanelProps) -> Self {
        props.heading.clone()
    }
}

impl From<&UnitOverridePanelProps> for UnitTileOverrideProps {
    fn from(props: &UnitOverridePanelProps) -> Self {
        props.tile_override.clone()
    }
}
