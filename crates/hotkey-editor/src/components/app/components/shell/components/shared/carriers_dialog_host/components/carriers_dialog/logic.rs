use super::components::carriers_grid::components::carrier_card::CarrierCardProps;
use super::props::CarriersDialogProps;

/// One card per resolved carrier of the ability.
pub(super) fn cards(props: &CarriersDialogProps) -> Vec<CarrierCardProps> {
    props
        .carriers
        .iter()
        .map(|carrier| CarrierCardProps {
            unit_id: carrier.unit_id().to_owned(),
            icon_url: carrier.icon_url().map(str::to_owned),
            name: carrier.name().to_owned(),
            view_navigation: props.view_navigation,
        })
        .collect()
}
