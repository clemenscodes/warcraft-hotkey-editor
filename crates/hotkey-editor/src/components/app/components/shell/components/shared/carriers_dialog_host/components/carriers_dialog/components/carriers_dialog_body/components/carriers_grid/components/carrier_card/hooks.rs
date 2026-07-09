use super::logic::{CarrierCardInputs, CarrierCardModel};
use super::props::CarrierCardProps;
use crate::services::navigation::context::use_view_navigation;

/// Reads the navigation from context and shapes the carrier card: its icon, name, id,
/// and the open-unit handler the card's button fires.
pub(super) fn use_carrier_card(props: &CarrierCardProps) -> CarrierCardModel {
    let view_navigation = use_view_navigation();
    let inputs = CarrierCardInputs {
        props: props.clone(),
        view_navigation,
    };
    CarrierCardModel::from(inputs)
}
