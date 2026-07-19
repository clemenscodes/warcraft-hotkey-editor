use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::context::use_unit_catalog;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use wasm_bindgen::JsCast;

// Cards are content-height now, so the pager renders a small window and pads the
// rest with two spacer divs whose heights come from the cards' MEASURED heights.
// The buffer renders a few cards past the viewport on each side, so a card
// entering from below is mounted — and therefore measured — while it is still off
// screen; its height then only grows the bottom spacer and never shifts what is
// on screen.
const CARD_WINDOW_BUFFER: usize = 3;
// First guess for a card whose real height is not measured yet. Only the
// off-screen spacer totals lean on it; the cards you have actually passed are
// measured, and those are what the window and the scroll position are computed
// from, so an imperfect estimate never misplaces the current card.
const CARD_HEIGHT_ESTIMATE: i32 = 700;
// The roster repeats this many times in the scroll content, so scrolling past the
// last unit slides the first one in with no jump — an endless loop both ways. Odd,
// so there is a clean middle copy; the pager opens there so it can loop backward
// as well as forward. Each slot renders unit `virtual_index % roster_len`.
const LOOP_CYCLES: usize = 9;

pub(super) struct MobileEditorPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) onscroll: EventHandler<ScrollEvent>,
    pub(super) onscrollend: EventHandler<ScrollEvent>,
    pub(super) top_spacer_px: i32,
    pub(super) bottom_spacer_px: i32,
    pub(super) window_unit_ids: Vec<WarcraftObjectId>,
}

fn card_height(heights: &[i32], index: usize) -> i32 {
    match heights.get(index) {
        Some(height) if *height > 0 => *height,
        _ => CARD_HEIGHT_ESTIMATE,
    }
}

fn sum_heights(heights: &[i32], start: usize, end: usize) -> i32 {
    (start..end).map(|index| card_height(heights, index)).sum()
}

// The height of one whole copy of the roster (at least 1, so it can divide).
fn copy_height(heights: &[i32], unit_count: usize) -> i32 {
    sum_heights(heights, 0, unit_count).max(1)
}

// Height above a virtual slot. Slot `virtual_index` renders unit
// `virtual_index % unit_count`; its offset is that many whole copies plus the
// units before it within its own copy.
fn virtual_offset(heights: &[i32], unit_count: usize, virtual_index: usize) -> i32 {
    if unit_count == 0 {
        return 0;
    }
    let copies = i32::try_from(virtual_index / unit_count).unwrap_or(0);
    let remainder = virtual_index % unit_count;
    copies * copy_height(heights, unit_count) + sum_heights(heights, 0, remainder)
}

// The virtual slot straddling the vertical middle of the viewport at this scroll
// position, from the heights measured so far.
fn centered_virtual(
    heights: &[i32],
    unit_count: usize,
    virtual_count: usize,
    scroll_top: i32,
    viewport: i32,
) -> usize {
    if unit_count == 0 {
        return 0;
    }
    let total = copy_height(heights, unit_count);
    let target = scroll_top + viewport / 2;
    let copy = (target / total).max(0);
    let within = target - copy * total;
    let mut accumulated = 0;
    let mut unit = unit_count - 1;
    for index in 0..unit_count {
        accumulated += card_height(heights, index);
        if accumulated > within {
            unit = index;
            break;
        }
    }
    let copy_usize = usize::try_from(copy).unwrap_or(0);
    (copy_usize * unit_count + unit).min(virtual_count.saturating_sub(1))
}

pub(super) fn use_mobile_editor() -> MobileEditorPresentation {
    // The pager walks the whole roster — every unit of every race, in canonical
    // race order, melee before campaign — not the active race's filtered listing.
    // A swipe carries from the last unit of one race into the first of the next,
    // and `open_unit` (in `onscrollend`) switches the race tab to match.
    let catalog = use_unit_catalog();
    let unit_ids_memo = use_memo(move || {
        let roster = catalog.roster();
        let ids: Rc<[WarcraftObjectId]> = roster
            .entries()
            .iter()
            .map(|entry| entry.unit_id())
            .collect();
        ids
    });

    let viewport_px = use_signal::<i32>(|| 0);
    // The unit the pager has committed to (drives navigation). Advances only when
    // a swipe settles, in `onscrollend`.
    let active_index = use_signal::<usize>(|| 0);
    // Which cards are mounted, kept in step with the live scroll position so the
    // card swiping in from below is already rendered before it arrives.
    let window_center = use_signal::<usize>(|| 0);
    // Set once the opening card has been centred, so the first layout scrolls
    // there exactly once.
    let positioned = use_signal::<bool>(|| false);
    // Set while the pager scrolls itself to follow a navigation, so that scroll is
    // not mistaken for a swipe. See `onscrollend`.
    let following_navigation = use_signal::<bool>(|| false);
    // Measured card heights, indexed by unit; 0 means not measured yet.
    let measured_heights = use_signal::<Vec<i32>>(Vec::new);
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));
    // Written during render, read by the measuring effect — a plain cell so it
    // does not itself trigger a re-render.
    let window_start_cell = use_hook(|| Rc::new(Cell::<usize>::new(0)));
    // Last scroll position, so `onscroll` can tell which way the finger is going
    // and hide or reveal the footer accordingly.
    let previous_scroll_cell = use_hook(|| Rc::new(Cell::<i32>::new(0)));
    // Movement accumulated since the last direction change, so a brief snap-back
    // bounce during a downward swipe cannot be mistaken for an upward swipe.
    let scroll_intent_cell = use_hook(|| Rc::new(Cell::<i32>::new(0)));

    let unit_ids = unit_ids_memo();
    let unit_count = unit_ids.len();
    let virtual_count = unit_count.saturating_mul(LOOP_CYCLES);

    let mounted_element_ref = element_ref.clone();
    let mut mounted_viewport_px = viewport_px;
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let measured_height = element.client_height();
        *mounted_element_ref.borrow_mut() = Some(element);
        if *mounted_viewport_px.peek() != measured_height {
            mounted_viewport_px.set(measured_height);
        }
    });

    let editor = use_editor_state();
    let mut selected_slot = editor.selected_slot();
    let mut selected_from_research = editor.selected_from_research();
    let mut selected_from_uprooted = editor.selected_from_uprooted();
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id();
    let scroll_selected_unit_id = selected_unit_id;

    let scroll_search_dialog_open = editor.search_dialog_open();
    let scroll_unit_ids = unit_ids.clone();
    let scroll_element_ref = element_ref.clone();
    let scroll_measured_heights = measured_heights;
    let mut scroll_viewport_px = viewport_px;
    let mut scroll_active_index = active_index;
    let mut scroll_following_navigation = following_navigation;
    let onscrollend = EventHandler::new(move |_event: ScrollEvent| {
        let borrowed = scroll_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let measured_height = element.client_height();
        if measured_height <= 0 {
            return;
        }
        let scroll_top = element.scroll_top();
        // The centred slot is a virtual index across the repeated copies; the unit
        // it commits to is that index folded back into the single roster.
        let index = {
            let heights = scroll_measured_heights.peek();
            let virtual_index = centered_virtual(
                &heights,
                unit_count,
                virtual_count,
                scroll_top,
                measured_height,
            );
            if unit_count == 0 {
                0
            } else {
                virtual_index % unit_count
            }
        };
        if *scroll_viewport_px.peek() != measured_height {
            scroll_viewport_px.set(measured_height);
        }
        // The pager scrolls itself when a unit is chosen elsewhere — the search
        // dialog, back/forward. That programmatic scroll can settle several times
        // before it lands, because each card that mounts and is measured re
        // anchors the scroll and fires its own scrollend. Swallow every settle
        // until the centred card is the target the pager is heading for, so an
        // intermediate card is never mistaken for a swipe and committed, which
        // would fight the very navigation being followed. Cards are taller than
        // half the viewport, so the aligned target is always the centred card and
        // this latch clears the moment the pager arrives.
        if *scroll_following_navigation.peek() {
            if index == *scroll_active_index.peek() {
                scroll_following_navigation.set(false);
            }
            return;
        }
        // The search dialog owns the navigation while it is open. The pager
        // underneath still measures and re anchors its cards, and each of those
        // scroll corrections settles here. Committing the centred card would
        // re publish its unit and re add its mode, overwriting the unit and mode
        // the dialog navigated to, so the pager stays silent until the dialog
        // closes.
        if *scroll_search_dialog_open.peek() {
            return;
        }
        if *scroll_active_index.peek() != index {
            scroll_active_index.set(index);
            // A slot id is the ability itself, not the ability on this unit, so a
            // surviving selection would light up on every card that shares it.
            selected_slot.set(None);
            selected_from_research.set(false);
            selected_from_uprooted.set(false);
            // `open_unit` derives the race and the unit mode from the unit and
            // pushes a navigation snapshot, so swiping keeps the race theme, the
            // `unit` query parameter and the history entry in step with the card.
            // The guard keeps back and forward alive: navigating scrolls the
            // pager, firing this handler, and re-pushing the unit we were told to
            // show would bury the entry we came from.
            if let Some(unit_id) = scroll_unit_ids.get(index).copied() {
                let already_current = *scroll_selected_unit_id.peek() == Some(unit_id);
                if !already_current {
                    navigation.open_unit(unit_id);
                }
            }
        }
    });

    // Runs on every scroll frame, including mid-snap, so the mounted window tracks
    // where the scroll is heading rather than where it last settled — the card
    // below is already in the DOM before it slides into view. It only moves the
    // render window and never commits navigation.
    let window_scroll_element_ref = element_ref.clone();
    let window_scroll_measured_heights = measured_heights;
    let mut window_scroll_center = window_center;
    let window_scroll_previous = previous_scroll_cell.clone();
    let window_scroll_intent = scroll_intent_cell.clone();
    let mut window_scroll_footer_hidden = editor.footer_hidden();
    let onscroll = EventHandler::new(move |_event: ScrollEvent| {
        let borrowed = window_scroll_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let measured_height = element.client_height();
        if measured_height <= 0 {
            return;
        }
        let scroll_top = element.scroll_top();
        let virtual_index = {
            let heights = window_scroll_measured_heights.peek();
            centered_virtual(
                &heights,
                unit_count,
                virtual_count,
                scroll_top,
                measured_height,
            )
        };
        if *window_scroll_center.peek() != virtual_index {
            window_scroll_center.set(virtual_index);
        }
        // Scrolling the cards down hides the footer, up reveals it. Movement is
        // accumulated since the last direction change so a brief snap-back bounce
        // during a downward swipe never reaches the reveal threshold, while a
        // sustained upward swipe does. Hiding reacts quickly, revealing needs a
        // clear intent; near the very top the footer always shows so it is never
        // stuck away when there is nothing below.
        let previous = window_scroll_previous.get();
        window_scroll_previous.set(scroll_top);
        let scroll_delta = scroll_top - previous;
        let mut scroll_intent = window_scroll_intent.get();
        if (scroll_delta > 0 && scroll_intent < 0) || (scroll_delta < 0 && scroll_intent > 0) {
            scroll_intent = 0;
        }
        scroll_intent += scroll_delta;
        let want_hidden = if scroll_top <= measured_height / 2 {
            scroll_intent = 0;
            false
        } else if scroll_intent > 8 {
            scroll_intent = 0;
            true
        } else if scroll_intent < -64 {
            scroll_intent = 0;
            false
        } else {
            *window_scroll_footer_hidden.peek()
        };
        window_scroll_intent.set(scroll_intent);
        if *window_scroll_footer_hidden.peek() != want_hidden {
            window_scroll_footer_hidden.set(want_hidden);
        }
    });

    // Measure the cards currently mounted and record their real heights, so the
    // spacers and the scroll-to-unit maths reflect the true layout rather than the
    // estimate. Runs after each render the window shifts, reads the DOM, and only
    // writes back a height that actually changed, so it settles in one pass. A
    // card is measured while it is still below the fold (the look-ahead buffer),
    // so recording it only grows the bottom spacer and never shifts the view.
    let measure_element_ref = element_ref.clone();
    let measure_window_start = window_start_cell.clone();
    let mut measure_heights = measured_heights;
    let measure_window_center = window_center;
    let measure_viewport_px = viewport_px;
    let measure_previous_scroll = previous_scroll_cell.clone();
    use_effect(move || {
        // Re-run whenever the mounted window shifts, and once the viewport is
        // first measured (mount) or changes (resize) — a resize changes every
        // card's height, so they must be measured again.
        let _ = measure_window_center.read();
        let _ = measure_viewport_px.read();
        let borrowed = measure_element_ref.borrow();
        let Some(container) = borrowed.as_ref() else {
            return;
        };
        let Ok(nodes) = container.query_selector_all(".pager-card-host") else {
            return;
        };
        let start = measure_window_start.get();
        let old_heights = measure_heights.peek().clone();
        let mut heights = old_heights.clone();
        if heights.len() < unit_count {
            heights.resize(unit_count, 0);
        }
        let mut changed = false;
        if unit_count > 0 {
            for offset in 0..nodes.length() {
                let Some(node) = nodes.item(offset) else {
                    continue;
                };
                let Some(card) = node.dyn_ref::<web_sys::Element>() else {
                    continue;
                };
                let height = card.client_height();
                // Node `offset` renders virtual slot `start + offset`, which is
                // unit `(start + offset) % unit_count`.
                let index = (start + offset as usize) % unit_count;
                if index < heights.len() && height > 0 && heights[index] != height {
                    heights[index] = height;
                    changed = true;
                }
            }
        }
        if changed {
            // Measuring a card changes the whole-copy height, and because the
            // window sits several roster copies below the top that swing is
            // multiplied by the copy count — so the top spacer moves by many times
            // the one card that changed. Left alone the view would leap 10-20
            // cards. Shift the scroll by the same swing so the card under the
            // user's eyes stays exactly where it is, and record the corrected
            // position so the footer logic does not read the correction as a swipe.
            let old_top = virtual_offset(&old_heights, unit_count, start);
            let new_top = virtual_offset(&heights, unit_count, start);
            let anchor_adjust = new_top - old_top;
            measure_heights.set(heights);
            if anchor_adjust != 0 {
                let corrected = container.scroll_top() + anchor_adjust;
                container.set_scroll_top(corrected);
                measure_previous_scroll.set(corrected);
            }
        }
    });

    // The other direction: a unit chosen anywhere but the pager itself — most of
    // all back and forward — brings its card on screen. Positions on first layout
    // too, so the opening card is centred.
    let effect_unit_ids_memo = unit_ids_memo;
    let effect_element_ref = element_ref.clone();
    let effect_window_start = window_start_cell.clone();
    let mut effect_active_index = active_index;
    let mut effect_window_center = window_center;
    let mut effect_positioned = positioned;
    let mut effect_following_navigation = following_navigation;
    use_effect(move || {
        let target_unit_ids = effect_unit_ids_memo();
        if target_unit_ids.is_empty() {
            return;
        }
        let selected = *selected_unit_id.read();
        let matched_index = selected.and_then(|target_unit_id| {
            target_unit_ids
                .iter()
                .position(|unit_id| *unit_id == target_unit_id)
        });
        let target_index = matched_index.unwrap_or(0);
        let already_here = *effect_positioned.peek() && *effect_active_index.peek() == target_index;
        if already_here {
            return;
        }
        let borrowed = effect_element_ref.borrow();
        let Some(element) = borrowed.as_ref() else {
            return;
        };
        let measured_height = element.client_height();
        if measured_height <= 0 {
            return;
        }
        // Place the pager in the middle copy at the selected unit, so it can loop
        // both backward and forward from the opening card.
        let middle_virtual = unit_count * (LOOP_CYCLES / 2) + target_index;
        effect_active_index.set(target_index);
        effect_window_center.set(middle_virtual);
        effect_positioned.set(true);
        effect_following_navigation.set(true);
        // Scrolling waits a tick for the window this index change re-renders, then
        // aligns to the target card's REAL position rather than an estimated
        // offset — the spacers around it are still estimates for the cards nobody
        // has scrolled past yet, so any sum of estimates would overshoot. Reading
        // the card's own rect sidesteps that entirely.
        let scroll_element = element.clone();
        let spawn_window_start = effect_window_start.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(0).await;
            let container_top = scroll_element.get_bounding_client_rect().top();
            let Ok(nodes) = scroll_element.query_selector_all(".pager-card-host") else {
                return;
            };
            let start = spawn_window_start.get();
            let offset_in_window = middle_virtual.saturating_sub(start);
            let Ok(node_index) = u32::try_from(offset_in_window) else {
                return;
            };
            let Some(node) = nodes.item(node_index) else {
                return;
            };
            let Some(card) = node.dyn_ref::<web_sys::Element>() else {
                return;
            };
            let card_top = card.get_bounding_client_rect().top();
            let delta = (card_top - container_top).round();
            let Ok(delta_px) = i32::try_from(delta as i64) else {
                return;
            };
            let target_scroll_top = scroll_element.scroll_top() + delta_px;
            scroll_element.set_scroll_top(target_scroll_top);
        });
    });

    let current_viewport_px = *viewport_px.read();
    let virtual_last = virtual_count.saturating_sub(1);
    // The window is a slice of the VIRTUAL, looping index space; each slot renders
    // `unit_ids[virtual_index % unit_count]`, so the roster repeats and scrolling
    // past the last unit slides the first one in with no jump.
    let current_window_center = (*window_center.read()).min(virtual_last);
    let heights_read = measured_heights.read();

    let window_start;
    let window_end;
    if current_viewport_px <= 0 {
        window_start = 0;
        window_end = virtual_count.min(2 * CARD_WINDOW_BUFFER + 1);
    } else {
        window_start = current_window_center.saturating_sub(CARD_WINDOW_BUFFER);
        window_end = (current_window_center + CARD_WINDOW_BUFFER + 1).min(virtual_count);
    }
    window_start_cell.set(window_start);

    let window_unit_ids: Vec<WarcraftObjectId> = if unit_count == 0 {
        Vec::new()
    } else {
        (window_start..window_end)
            .map(|virtual_index| unit_ids[virtual_index % unit_count])
            .collect()
    };
    let top_spacer_px = virtual_offset(&heights_read, unit_count, window_start);
    let bottom_spacer_px = virtual_offset(&heights_read, unit_count, virtual_count)
        - virtual_offset(&heights_read, unit_count, window_end);

    MobileEditorPresentation {
        onmounted,
        onscroll,
        onscrollend,
        top_spacer_px,
        bottom_spacer_px,
        window_unit_ids,
    }
}
