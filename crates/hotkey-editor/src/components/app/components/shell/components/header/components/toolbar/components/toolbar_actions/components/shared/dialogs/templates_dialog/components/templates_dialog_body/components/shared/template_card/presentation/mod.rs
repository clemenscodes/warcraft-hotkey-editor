use super::model::TemplateCardModel;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use wasm_bindgen::JsCast;

// The grids a template previews: its command card and its research menu.
pub(super) const PREVIEW_GRID_COUNT: usize = 2;

pub(super) struct TemplateCardPresentation {
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onscroll: EventHandler<ScrollEvent>,
    pub(super) active_grid_index: usize,
}

impl ddd::Layered for TemplateCardPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for TemplateCardPresentation {
    type Model = TemplateCardModel;
}

pub(super) fn use_template_card(props: &TemplateCardModel) -> TemplateCardPresentation {
    let on_apply = props.on_apply;
    let onclick = EventHandler::new(move |_event: MouseEvent| on_apply.call(()));
    // The preview grid the sideways carousel shows, so the dots below it follow
    // a swipe the same way a unit card's do.
    let active_grid_index_signal = use_signal::<usize>(|| 0);
    let onscroll = use_hook(|| {
        let mut active_grid_index_writer = active_grid_index_signal;
        EventHandler::new(move |event: ScrollEvent| {
            let Some(web_event) = event.data().try_as_web_event() else {
                return;
            };
            let Some(scroll_target) = web_event.target() else {
                return;
            };
            let Ok(carousel_element) = scroll_target.dyn_into::<web_sys::Element>() else {
                return;
            };
            let carousel_client_width = carousel_element.client_width();
            if carousel_client_width <= 0 {
                return;
            }
            let carousel_scroll_left = carousel_element.scroll_left();
            let half_pane_width = carousel_client_width / 2;
            let raw_index = (carousel_scroll_left + half_pane_width) / carousel_client_width;
            let next_index = usize::try_from(raw_index).unwrap_or(0);
            if *active_grid_index_writer.peek() != next_index {
                active_grid_index_writer.set(next_index);
            }
        })
    });
    let active_grid_index = active_grid_index_signal();
    TemplateCardPresentation {
        onclick,
        onscroll,
        active_grid_index,
    }
}
