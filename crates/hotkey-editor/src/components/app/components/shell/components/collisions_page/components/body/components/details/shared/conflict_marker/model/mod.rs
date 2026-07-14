use super::state::ConflictMarker;
use super::view::ConflictMarkerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictMarkerModel {
    pub marker: ConflictMarker,
    #[props(default)]
    pub is_top: bool,
}

impl From<&ConflictMarkerView> for ConflictMarkerModel {
    fn from(view: &ConflictMarkerView) -> Self {
        let ConflictMarkerView { marker, is_top } = view.clone();
        Self { marker, is_top }
    }
}

impl ddd::Model for ConflictMarkerModel {
    type View = ConflictMarkerView;
}
