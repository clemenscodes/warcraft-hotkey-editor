use super::components::carriers_grid::CarriersGridProps;
use super::components::carriers_grid::components::carrier_card::CarrierCardProps;
use dioxus::prelude::*;

/// The carriers dialog's scroll region input: the carrier cards to lay out in a grid.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogBodyProps {
    pub cards: Vec<CarrierCardProps>,
}

impl From<&CarriersDialogBodyProps> for CarriersGridProps {
    fn from(props: &CarriersDialogBodyProps) -> Self {
        let cards = props.cards.clone();
        Self { cards }
    }
}
