use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

/// The carriers dialog's scroll region input: the carriers to lay out in a grid.
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogBodyProps {
    pub carriers: Vec<CarrierUnitView>,
}
