use super::components::carrier_card_icon::CarrierCardIconProps;
use super::props::CarrierCardProps;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The shaped carrier card: the icon props, the deep-link click handler, and the name
/// and id text the card places directly.
pub(super) struct CarrierCardModel {
    pub(super) icon: CarrierCardIconProps,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) name: String,
    pub(super) unit_id: WarcraftObjectId,
}

impl From<&CarrierCardProps> for CarrierCardModel {
    fn from(props: &CarrierCardProps) -> Self {
        let name = props.name.clone();
        let unit_id = props.unit_id;
        let view_navigation = props.view_navigation;
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
