use super::components::above_center_tooltip::AboveCenterTooltipProps;
use super::components::above_left_tooltip::AboveLeftTooltipProps;
use super::components::above_right_tooltip::AboveRightTooltipProps;
use super::components::below_center_tooltip::BelowCenterTooltipProps;
use super::components::below_left_tooltip::BelowLeftTooltipProps;
use super::components::below_right_tooltip::BelowRightTooltipProps;
use super::props::TooltipProps;

/// Each positioned bubble shows only the text; its placement and anchor are baked
/// into its own component, so the dispatcher hands every look the same message.

impl From<&TooltipProps> for AboveLeftTooltipProps {
    fn from(props: &TooltipProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}

impl From<&TooltipProps> for AboveCenterTooltipProps {
    fn from(props: &TooltipProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}

impl From<&TooltipProps> for AboveRightTooltipProps {
    fn from(props: &TooltipProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}

impl From<&TooltipProps> for BelowLeftTooltipProps {
    fn from(props: &TooltipProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}

impl From<&TooltipProps> for BelowCenterTooltipProps {
    fn from(props: &TooltipProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}

impl From<&TooltipProps> for BelowRightTooltipProps {
    fn from(props: &TooltipProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}
