use super::components::carrier_card::CarrierCardProps;
use super::props::CarriersDialogProps;

/// One card per unit that carries the ability.
pub(super) fn cards(props: &CarriersDialogProps) -> Vec<CarrierCardProps> {
    props
        .dialog_data
        .carriers
        .iter()
        .map(|carrier| CarrierCardProps {
            unit_id: carrier.unit_id.clone(),
            icon_url: carrier.icon_url.clone(),
            name: carrier.name.clone(),
            view_navigation: props.view_navigation,
        })
        .collect()
}
