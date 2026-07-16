use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use super::model::HotkeyOverrideCardModel;

/// The font size the card starts at, as a fraction of its query container.
const BASE_FONT_CQI: f64 = 3.8;
/// The floor the card refuses to shrink past, so text never becomes unreadable.
const MINIMUM_FONT_CQI: f64 = 2.2;
/// How much one shrink step takes off.
const SHRINK_STEP_CQI: f64 = 0.15;

pub(super) struct HotkeyOverrideCardFit {
    pub(super) font_style: String,
    pub(super) onmounted: EventHandler<MountedEvent>,
}

/// Reports whether anything inside the card is being cut off.
///
/// Asking the card alone is not enough. It lays its parts out as a flex column,
/// so when the text grows the parts shrink instead of pushing past the card, and
/// the card can keep reporting that everything fits while the text is already
/// clipped several levels down. The whole subtree has to be asked.
fn subtree_overflows(element: &web_sys::Element) -> bool {
    if is_clipped(element) {
        return true;
    }
    let Ok(descendants) = element.query_selector_all("*") else {
        return false;
    };
    let descendant_count = descendants.length();
    for descendant_index in 0..descendant_count {
        let Some(node) = descendants.get(descendant_index) else {
            continue;
        };
        let Ok(descendant) = node.dyn_into::<web_sys::Element>() else {
            continue;
        };
        if is_clipped(&descendant) {
            return true;
        }
    }
    false
}

fn is_clipped(element: &web_sys::Element) -> bool {
    // A pixel of slack: sub-pixel layout rounding otherwise reads as an overflow
    // and would shrink the font forever.
    const ROUNDING_SLACK_PX: i32 = 1;
    let scroll_height = element.scroll_height();
    let client_height = element.client_height();
    scroll_height > client_height + ROUNDING_SLACK_PX
}

/// Shrinks the card's font until its content fits the fixed box it is given.
///
/// The card is a fixed-height page inside the mobile pager, but an ability's
/// text is not fixed at all, so a single font size either wastes the box or
/// overflows it. This measures the rendered card and steps the font down until
/// nothing is clipped, publishing the result as a custom property the mobile
/// band reads. Desktop never overflows, since the card hugs its content there,
/// so the search settles on the first pass and the property goes unused.
pub(super) fn use_hotkey_override_card_fit(
    props: &HotkeyOverrideCardModel,
) -> HotkeyOverrideCardFit {
    let object_id = props.object_id;
    let mut font_cqi = use_signal::<f64>(|| BASE_FONT_CQI);
    // Bumped whenever the shown ability changes. `font_cqi` alone cannot carry
    // that: switching from one ability to another that both sit at the base size
    // writes the same value, which is not a change and would never re-arm the
    // measuring effect below.
    let mut fit_generation = use_signal::<u32>(|| 0);
    // Set once the card's element exists. The measuring effect runs after render
    // but the element only arrives with `onmounted`, so the effect has to be able
    // to wait for it. A plain `RefCell` cannot wake it; a signal can.
    let mut mounted_revision = use_signal::<u32>(|| 0);
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));

    let mounted_element_ref = element_ref.clone();
    let onmounted = use_hook(|| {
        EventHandler::new(move |event: MountedEvent| {
            let Some(element) = event.data().try_as_web_event() else {
                return;
            };
            *mounted_element_ref.borrow_mut() = Some(element);
            let next_revision = mounted_revision.peek().wrapping_add(1);
            mounted_revision.set(next_revision);
        })
    });

    // A different ability means different text, so the search restarts from the
    // top rather than inheriting the previous ability's shrink.
    use_effect(use_reactive!(|object_id| {
        let _ = object_id;
        if *font_cqi.peek() != BASE_FONT_CQI {
            font_cqi.set(BASE_FONT_CQI);
        }
        let next_generation = fit_generation.peek().wrapping_add(1);
        fit_generation.set(next_generation);
    }));

    // Re-runs on three things: the card mounting, the ability changing, and each
    // shrink. That last one is what makes this converge without a measure loop,
    // because every pass is measured against the freshly laid-out DOM. It
    // terminates since a pass either fits or takes a step off, and the floor
    // stops it.
    let measured_element_ref = element_ref.clone();
    use_effect(move || {
        let _ = mounted_revision.read();
        let _ = fit_generation.read();
        let current_font_cqi = *font_cqi.read();
        let borrowed = measured_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let client_height = element.client_height();
        if client_height <= 0 {
            return;
        }
        if current_font_cqi <= MINIMUM_FONT_CQI {
            return;
        }
        let overflows = subtree_overflows(element);
        if !overflows {
            return;
        }
        let shrunk = current_font_cqi - SHRINK_STEP_CQI;
        let next_font_cqi = shrunk.max(MINIMUM_FONT_CQI);
        font_cqi.set(next_font_cqi);
    });

    let current_font_cqi = *font_cqi.read();
    let font_style = format!("--override-font-size:{current_font_cqi}cqi");
    HotkeyOverrideCardFit {
        font_style,
        onmounted,
    }
}
