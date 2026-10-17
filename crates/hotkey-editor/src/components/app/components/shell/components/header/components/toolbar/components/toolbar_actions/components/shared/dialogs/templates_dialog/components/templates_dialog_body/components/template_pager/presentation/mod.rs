use super::model::TemplatePagerModel;
use crate::components::app::components::shell::components::shared::spring_pager::{
    CardResizeWatch, PagerAnchor, PagerMotion, PagerWindow, PassiveInputListeners,
};
use crate::services::drag_state::context::use_drag_state;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::components::templates_dialog_body::components::shared::template_card::TemplateCardView;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;

// The template card hosts, the elements the pager's motion measures and moves.
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

/// Pages through the templates the way the editor pages through units: one
/// template per swipe, along the same spring. There are only a handful, so all
/// of them stay mounted and the window never moves.
pub(super) fn use_template_pager(props: &TemplatePagerModel) -> TemplatePagerPresentation {
    let cards = props.cards.clone();
    let template_count = cards.len();
    let center_slot = isize::try_from(template_count / 2).unwrap_or(0);
    let window =
        use_signal(|| PagerWindow::centered_on(center_slot, template_count, template_count));
    // Nothing settles on a template the way a unit becomes the selection, so the
    // rest the motion reports is not acted on.
    let rested_anchor = use_signal(|| None::<PagerAnchor>);
    let dragging_slot = use_drag_state().dragging_slot();
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));
    let hook_element_ref = element_ref.clone();
    let motion = use_hook(move || {
        PagerMotion::new(
            hook_element_ref,
            window,
            rested_anchor,
            dragging_slot,
            None,
            CARD_SELECTOR,
        )
    });
    // Created on mount, once the pager exists; dropping them disconnects.
    let resize_watch = use_hook(|| Rc::new(RefCell::new(None::<CardResizeWatch>)));
    let input_listeners = use_hook(|| Rc::new(RefCell::new(None::<PassiveInputListeners>)));
    let mounted_element_ref = element_ref.clone();
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        *mounted_element_ref.borrow_mut() = Some(element.clone());
        let resized_motion = motion.clone();
        let created_watch = CardResizeWatch::new(move || {
            resized_motion.reapply();
        });
        if let Some(watch) = created_watch.as_ref() {
            watch.observe_cards(&element, CARD_SELECTOR);
        }
        *resize_watch.borrow_mut() = created_watch;
        let mut listeners = PassiveInputListeners::new(&element);
        let pointer_down_motion = motion.clone();
        listeners.listen("pointerdown", move |event| {
            pointer_down_motion.pointer_down(&event)
        });
        let pointer_move_motion = motion.clone();
        listeners.listen("pointermove", move |event| {
            pointer_move_motion.pointer_move(&event)
        });
        let pointer_up_motion = motion.clone();
        listeners.listen("pointerup", move |event| {
            pointer_up_motion.pointer_up(&event)
        });
        let pointer_cancel_motion = motion.clone();
        listeners.listen("pointercancel", move |event| {
            pointer_cancel_motion.pointer_cancel(&event)
        });
        let wheel_motion = motion.clone();
        listeners.listen("wheel", move |event| wheel_motion.wheel(&event));
        let key_motion = motion.clone();
        listeners.listen("keydown", move |event| key_motion.key_down(&event));
        let transition_motion = motion.clone();
        listeners.listen("transitionend", move |event| {
            transition_motion.transition_end(&event)
        });
        *input_listeners.borrow_mut() = Some(listeners);
        // The first template rests at the top. If the cards are not laid out yet,
        // the resize watch applies the held anchor once they are.
        let first_slot = 0;
        let first_template = PagerAnchor::aligned(first_slot);
        motion.hold(first_template);
    });
    TemplatePagerPresentation { cards, onmounted }
}
