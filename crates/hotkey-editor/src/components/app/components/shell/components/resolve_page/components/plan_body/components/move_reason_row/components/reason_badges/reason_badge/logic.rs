use super::components::human_reason_badge::HumanReasonBadgeProps;
use super::components::orc_reason_badge::OrcReasonBadgeProps;
use super::components::success_reason_badge::SuccessReasonBadgeProps;
use super::components::undead_reason_badge::UndeadReasonBadgeProps;
use super::props::ReasonBadgeProps;

impl From<&ReasonBadgeProps> for OrcReasonBadgeProps {
    fn from(props: &ReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}

impl From<&ReasonBadgeProps> for HumanReasonBadgeProps {
    fn from(props: &ReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}

impl From<&ReasonBadgeProps> for UndeadReasonBadgeProps {
    fn from(props: &ReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}

impl From<&ReasonBadgeProps> for SuccessReasonBadgeProps {
    fn from(props: &ReasonBadgeProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
