use super::view::DraggableMarkerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DraggableMarkerModel {
    pub active: bool,
}

impl From<&DraggableMarkerView> for DraggableMarkerModel {
    fn from(view: &DraggableMarkerView) -> Self {
        let DraggableMarkerView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Layered for DraggableMarkerModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for DraggableMarkerModel {
    type View = DraggableMarkerView;
}
