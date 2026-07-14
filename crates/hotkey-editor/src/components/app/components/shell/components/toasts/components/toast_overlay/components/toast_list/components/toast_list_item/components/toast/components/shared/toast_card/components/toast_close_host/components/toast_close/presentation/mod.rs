use super::model::ToastCloseModel;
use dioxus::prelude::*;

pub struct ToastClosePresentation {
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&ToastCloseModel> for ToastClosePresentation {
    fn from(model: &ToastCloseModel) -> Self {
        let id = model.id;
        let on_remove = model.on_remove;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            on_remove.call(id);
        });
        Self { onclick }
    }
}

impl ddd::Presentation for ToastClosePresentation {
    type Model = ToastCloseModel;
}
