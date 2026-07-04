use super::super::super::DragFollowerOverlayProps;
use super::logic::FollowerPresentation;
use dioxus::prelude::*;
use warcraft_api::RaceLabels;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerGhostProps {
    pub race_attribute: &'static str,
    /// The follower's presentation when this grid owns the in-progress drag, or
    /// `None` when there is nothing to show.
    pub presentation: Option<FollowerPresentation>,
}

impl From<&DragFollowerOverlayProps> for DragFollowerGhostProps {
    /// The ghost needs the grid's race attribute plus, when this grid owns the
    /// drag, the dragged tile's presentation.
    fn from(props: &DragFollowerOverlayProps) -> Self {
        let race_attribute = RaceLabels::data_attribute(props.race);
        let active = if props.visible {
            props.drag_follower.read().clone()
        } else {
            None
        };
        let presentation = active.as_ref().map(FollowerPresentation::from);
        Self {
            race_attribute,
            presentation,
        }
    }
}
