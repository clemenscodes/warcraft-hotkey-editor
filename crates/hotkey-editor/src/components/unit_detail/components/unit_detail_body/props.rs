use super::components::unit_detail_row::UnitDetailRowProps;
use dioxus::prelude::*;

/// The body of the card below the stats: the grids-and-override row.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailBodyProps {
    pub row: UnitDetailRowProps,
}
