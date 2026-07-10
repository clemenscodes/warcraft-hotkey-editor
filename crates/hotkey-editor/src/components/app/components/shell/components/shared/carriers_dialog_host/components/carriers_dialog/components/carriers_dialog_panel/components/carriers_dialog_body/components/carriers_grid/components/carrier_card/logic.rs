use super::components::carrier_card_icon::CarrierCardIconProps;
use super::props::CarrierCardProps;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The carrier card's inputs: its own data plus the navigation read from context by the
/// component's hook.
pub(super) struct CarrierCardInputs {
    pub(super) props: CarrierCardProps,
    pub(super) view_navigation: ViewNavigationContext,
}

/// The shaped carrier card: the icon props, the deep-link click handler, and the name
/// and id text the card places directly.
pub(super) struct CarrierCardModel {
    pub(super) icon: CarrierCardIconProps,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
}

impl From<CarrierCardInputs> for CarrierCardModel {
    fn from(inputs: CarrierCardInputs) -> Self {
        let props = inputs.props;
        let view_navigation = inputs.view_navigation;
        let name = props.name.clone();
        let unit_id = props.unit_id;
        let icon = CarrierCardIconProps {
            src: props.icon_url.clone(),
            alt: name.clone(),
        };
        let onclick =
            EventHandler::new(move |_event: MouseEvent| view_navigation.open_unit(unit_id));
        Self {
            icon,
            onclick,
            name,
            unit_id,
        }
    }
}
