use crate::services::carriers::CarrierUnitView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The carrier card's inputs: the carrier view plus the navigation read from context by
/// the component's hook.
pub(super) struct CarrierCardInputs {
    pub(super) carrier: CarrierUnitView,
    pub(super) view_navigation: ViewNavigationContext,
}

/// The shaped carrier card: the icon source and alt text, the deep-link click handler,
/// and the name and id the card places directly.
pub(super) struct CarrierCardPresentation {
    pub(super) icon_src: Option<String>,
    pub(super) icon_alt: String,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
}

impl From<CarrierCardInputs> for CarrierCardPresentation {
    fn from(inputs: CarrierCardInputs) -> Self {
        let carrier = inputs.carrier;
        let view_navigation = inputs.view_navigation;
        let unit_id = carrier.unit_id();
        let name = carrier.name().to_owned();
        let icon_src = carrier.icon_url().map(str::to_owned);
        let icon_alt = name.clone();
        let onclick =
            EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(unit_id));
        Self {
            icon_src,
            icon_alt,
            onclick,
            name,
            unit_id,
        }
    }
}
use super::model::CarrierCardModel;
use crate::services::navigation::context::use_view_navigation;

/// Reads the navigation from context and shapes the carrier card: its icon, name, id,
/// and the open-unit handler the card's button fires.
pub(super) fn use_carrier_card(props: &CarrierCardModel) -> CarrierCardPresentation {
    let view_navigation = use_view_navigation();
    let carrier = props.carrier.clone();
    let inputs = CarrierCardInputs {
        carrier,
        view_navigation,
    };
    CarrierCardPresentation::from(inputs)
}

impl ddd::Presentation for CarrierCardPresentation {
    type Model = CarrierCardModel;
}
