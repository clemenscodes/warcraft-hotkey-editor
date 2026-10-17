mod stats_spread;

use super::model::UnitDetailDialogBodyModel;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use stats_spread::StatsSpread;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub(super) struct UnitDetailDialogBodyPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
}

impl ddd::Layered for UnitDetailDialogBodyPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for UnitDetailDialogBodyPresentation {
    type Model = UnitDetailDialogBodyModel;
}

pub(super) fn use_unit_detail_dialog_body() -> UnitDetailDialogBodyPresentation {
    // Created on mount, once the body exists; dropping it disconnects.
    let stats_spread = use_hook(|| Rc::new(RefCell::new(None::<StatsSpread>)));
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let Ok(body) = element.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        *stats_spread.borrow_mut() = StatsSpread::new(body);
    });
    UnitDetailDialogBodyPresentation { onmounted }
}
