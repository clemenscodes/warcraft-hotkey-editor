use super::components::carrier_card::CarrierCardProps;
use super::props::CarriersDialogProps;

/// One card per unit that carries the shared ability.
pub(super) fn cards(props: &CarriersDialogProps) -> Vec<CarrierCardProps> {
    props
        .dialog_data
        .carriers()
        .iter()
        .map(|carrier| CarrierCardProps {
            unit_id: carrier.unit_id().to_owned(),
            icon_url: carrier.icon_url().map(str::to_owned),
            name: carrier.name().to_owned(),
            view_navigation: props.view_navigation,
        })
        .collect()
}
