use super::components::carrier_card::CarrierCardProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CarriersGridProps {
    pub cards: Vec<CarrierCardProps>,
}
