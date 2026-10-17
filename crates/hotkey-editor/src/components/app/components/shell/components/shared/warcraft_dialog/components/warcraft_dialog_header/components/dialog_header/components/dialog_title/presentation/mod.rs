mod title_fit;

use super::model::DialogTitleModel;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;
use title_fit::TitleFit;
use wasm_bindgen::JsCast;

pub(super) struct DialogTitlePresentation {
    pub(super) title: String,
    pub(super) onmounted: EventHandler<MountedEvent>,
}

impl ddd::Layered for DialogTitlePresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for DialogTitlePresentation {
    type Model = DialogTitleModel;
}

pub(super) fn use_dialog_title(props: &DialogTitleModel) -> DialogTitlePresentation {
    let title = props.title.clone();
    // Created on mount, once the title exists; dropping it disconnects.
    let title_fit = use_hook(|| Rc::new(RefCell::new(None::<TitleFit>)));
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let Ok(title_element) = element.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        *title_fit.borrow_mut() = TitleFit::new(title_element);
    });
    DialogTitlePresentation { title, onmounted }
}
