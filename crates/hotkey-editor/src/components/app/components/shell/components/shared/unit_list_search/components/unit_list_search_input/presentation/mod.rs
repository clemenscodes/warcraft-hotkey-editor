mod dialog_autofocus;

use super::model::UnitListSearchInputModel;
use dialog_autofocus::DialogAutofocus;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub(super) struct UnitListSearchInputPresentation {
    pub(super) value: ReadSignal<String>,
    pub(super) placeholder: &'static str,
    pub(super) on_input: EventHandler<FormEvent>,
    pub(super) on_keydown: EventHandler<KeyboardEvent>,
    pub(super) onmounted: EventHandler<MountedEvent>,
}

impl ddd::Layered for UnitListSearchInputPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for UnitListSearchInputPresentation {
    type Model = UnitListSearchInputModel;
}

pub(super) fn use_unit_list_search_input(
    props: &UnitListSearchInputModel,
) -> UnitListSearchInputPresentation {
    let value = props.value;
    let placeholder = props.placeholder;
    let on_input = props.on_input;
    let on_keydown = props.on_keydown;
    let autofocus = props.autofocus;
    // Created on mount when the field autofocuses; dropping it removes the listener.
    let dialog_autofocus = use_hook(|| Rc::new(RefCell::new(None::<DialogAutofocus>)));
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        if !autofocus {
            return;
        }
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let Ok(field) = element.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        *dialog_autofocus.borrow_mut() = DialogAutofocus::new(field);
    });
    UnitListSearchInputPresentation {
        value,
        placeholder,
        on_input,
        on_keydown,
        onmounted,
    }
}
