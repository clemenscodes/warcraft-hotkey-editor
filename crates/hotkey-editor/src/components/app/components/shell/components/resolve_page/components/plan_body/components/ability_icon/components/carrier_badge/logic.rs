use super::components::regular_carrier_badge::RegularCarrierBadgeProps;
use super::components::winner_carrier_badge::WinnerCarrierBadgeProps;
use super::props::CarrierBadgeProps;

impl From<&CarrierBadgeProps> for WinnerCarrierBadgeProps {
    fn from(props: &CarrierBadgeProps) -> Self {
        let count = props.count;
        Self { count }
    }
}

impl From<&CarrierBadgeProps> for RegularCarrierBadgeProps {
    fn from(props: &CarrierBadgeProps) -> Self {
        let count = props.count;
        Self { count }
    }
}
