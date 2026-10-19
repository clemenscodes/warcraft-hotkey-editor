use super::model::TemplatePagerModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::components::templates_dialog_body::components::shared::template_card::TemplateCardView;
use crate::components::app::components::shell::components::shared::snap_pager::{
    PagerCallbacks, SnapPager,
};
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;

// The template card hosts, the cards the pager snaps onto.
const CARD_SELECTOR: &str = ".template-pager-card-host";

pub(super) struct TemplatePagerPresentation {
    pub(super) cards: Vec<TemplateCardView>,
    pub(super) onmounted: EventHandler<MountedEvent>,
}

impl ddd::Layered for TemplatePagerPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for TemplatePagerPresentation {
    type Model = TemplatePagerModel;
}

/// Pages through the templates like every other pager. Nothing settles on a
/// template the way a unit becomes the selection, and there are only a handful,
/// so all of them stay mounted and nothing is reported back.
pub(super) fn use_template_pager(props: &TemplatePagerModel) -> TemplatePagerPresentation {
    let cards = props.cards.clone();
    // Created on mount, once the pager exists; dropping it removes its listeners.
    let snap_pager = use_hook(|| Rc::new(RefCell::new(None::<SnapPager>)));
    // The callbacks can hold this cell themselves, so the pager is taken out on
    // unmount instead of waiting for the cell to be dropped.
    let dropped_snap_pager = snap_pager.clone();
    use_drop(move || {
        dropped_snap_pager.borrow_mut().take();
    });
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let callbacks = PagerCallbacks::new(|_card_index| {}, |_moved_distance| {}, |_edge| {});
        let pager = SnapPager::attach(element, CARD_SELECTOR, callbacks);
        *snap_pager.borrow_mut() = Some(pager);
    });
    TemplatePagerPresentation { cards, onmounted }
}
