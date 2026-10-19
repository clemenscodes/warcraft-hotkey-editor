use super::model::LayoutTileModel;
use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;
use dioxus::prelude::*;

pub(super) struct LayoutTilePresentation {
    pub(super) label: String,
    pub(super) keycap_state: EditableKeycapState,
    pub(super) ondragstart: EventHandler<Event<DragData>>,
    pub(super) ondragend: EventHandler<Event<DragData>>,
    pub(super) ondragover: EventHandler<Event<DragData>>,
    pub(super) ondrop: EventHandler<Event<DragData>>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&LayoutTileModel> for LayoutTilePresentation {
    fn from(props: &LayoutTileModel) -> Self {
        let label = props.label.clone();
        let keycap_state = EditableKeycapState::from(props.state);
        let ondragstart = props.ondragstart;
        let ondragend = props.ondragend;
        let ondragover = props.ondragover;
        let ondrop = props.ondrop;
        let onclick = props.onclick;
        Self {
            label,
            keycap_state,
            ondragstart,
            ondragend,
            ondragover,
            ondrop,
            onclick,
        }
    }
}

impl ddd::Layered for LayoutTilePresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for LayoutTilePresentation {
    type Model = LayoutTileModel;
}
