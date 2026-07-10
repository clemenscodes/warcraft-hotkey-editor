use crate::services::carriers::CarrierUnitView;
use dioxus::prelude::*;

/// The carriers dialog's bordered box inputs: the title and close handler its header row
/// carries, and the carriers laid out in the scrolling grid below it. Wrapped in the
/// library `DialogContent` (which carries no project class — this panel's own classed
/// `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogPanelProps {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub carriers: Vec<CarrierUnitView>,
}
