use super::props::TooltipProps;
use super::state::{TooltipAnchor, TooltipPlacement};

/// A tooltip's shaped presentation: its text, the `data-placement` / `data-anchor`
/// tokens its style keys on, and whether it should render at all. Built by `From`
/// so the body only places these.
pub(super) struct TooltipPresentation {
    pub(super) text: String,
    pub(super) placement: &'static str,
    pub(super) anchor: &'static str,
    pub(super) is_empty: bool,
}

impl From<&TooltipProps> for TooltipPresentation {
    fn from(props: &TooltipProps) -> Self {
        let text = props.text.clone();
        let is_empty = text.is_empty();
        let placement = match props.placement {
            TooltipPlacement::Below => "below",
            TooltipPlacement::Above => "above",
        };
        let anchor = match props.anchor {
            TooltipAnchor::Center => "center",
            TooltipAnchor::Left => "left",
            TooltipAnchor::Right => "right",
        };
        Self {
            text,
            placement,
            anchor,
            is_empty,
        }
    }
}
