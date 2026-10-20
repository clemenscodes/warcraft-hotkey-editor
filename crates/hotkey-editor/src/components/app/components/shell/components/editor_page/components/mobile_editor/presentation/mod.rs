use super::components::pager_track::components::pager_page::PagerPageView;
use crate::components::app::components::shell::components::shared::footer_intent::FooterIntent;
use crate::components::app::components::shell::components::shared::snap_pager::{
    PagerAnchor, PagerCallbacks, PagerEdge, PagerWindow, RosterSlot, SnapPager,
};
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use crate::services::unit_catalog::context::use_unit_catalog;
use crate::services::viewport::use_laptop_viewport;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use pager_paging::PagerPaging;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use warcraft_api::{Race, WarcraftApi, WarcraftObjectId};

mod pager_paging;

// Pages mounted on each side of the page the pager opens or jumps on.
const CARD_WINDOW_BUFFER: usize = 3;
// Pages mounted at a time when the scrolling comes near an end of the mounted
// pages. It happens while the scrolling is still under way, so each mount is
// kept small: a page is a full card of grids, and mounting several at once
// is a long frame the scrolling stutters on.
const EXTENSION_STEP: usize = 2;
// The most pages mounted at once before a settle trims the window back to the
// pages around the one on screen. Every mounted page re-renders when the
// selection or the navigation changes, so the fewer the smoother.
const MAX_MOUNTED_PAGES: usize = 15;
// Pages kept on each side of the page on screen when a settle trims the window.
const TRIM_BUFFER: usize = 3;
// The pager's pages, the cards it scrolls through.
const CARD_SELECTOR: &str = ".pager-page";

pub(super) struct MobileEditorPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) window_pages: Vec<PagerPageView>,
}

pub(super) fn use_mobile_editor() -> MobileEditorPresentation {
    // The pager walks the whole roster — every unit of every race, in canonical
    // race order, melee before campaign — not the active race's filtered listing.
    // Scrolling carries from the last unit of one race into the first of the next,
    // and `open_unit` (in the settle) switches the race tab to match.
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
    // The race of every roster unit, in the same order, so the paging can start
    // each race on a page of its own.
    let unit_races_memo = use_memo(move || {
        let api = WarcraftApi::default();
        let races: Rc<[Race]> = unit_ids_memo()
            .iter()
            .map(|unit_id| {
                api.unit()
                    .get(*unit_id)
                    .and_then(|unit| unit.race())
                    .unwrap_or(Race::Neutral)
            })
            .collect();
        races
    });

    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let selected_from_research = editor.selected_from_research();
    let selected_from_uprooted = editor.selected_from_uprooted();
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id();
    let footer_hidden = editor.footer_hidden();
    let laptop_viewport = use_laptop_viewport();

    // The window opens straight on the selected unit, so the very first render
    // already mounts the pages around it and nothing else.
    let opening_unit_ids = unit_ids_memo.peek().clone();
    let opening_races = unit_races_memo.peek().clone();
    let opening_unit_id = *selected_unit_id.peek();
    let opening_paging = PagerPaging::new(*laptop_viewport.peek());
    let opening_slot = opening_unit_id
        .and_then(|unit_id| opening_unit_ids.iter().position(|id| *id == unit_id))
        .map(|unit_index| opening_paging.page_of(unit_index, &opening_races))
        .and_then(|page_index| isize::try_from(page_index).ok())
        .unwrap_or(0);
    let opening_roster_len = opening_paging.page_count(&opening_races);
    let window = use_signal(|| {
        PagerWindow::centered_on(opening_slot, CARD_WINDOW_BUFFER, opening_roster_len)
    });
    // Where a page must sit once the next render is committed. Set by a jump
    // (align the target) and by a re-centre (keep the page on screen still);
    // the effect below applies it against the real layout and clears it.
    let pending_anchor = use_signal(|| Some(PagerAnchor::aligned(opening_slot)));
    // Set once the pager element exists; the anchor effect waits for it, because
    // nothing can be aligned before the pager is in the DOM.
    let pager_mounted = use_signal(|| false);
    // The slot the scrolling last came to rest on, cleared by the settle below
    // once it has acted on it.
    let rested_slot = use_signal(|| None::<isize>);
    // Created on mount, once the pager element exists; dropping it removes its
    // listeners.
    let snap_pager = use_hook(|| Rc::new(RefCell::new(None::<SnapPager>)));
    // The callbacks can hold this cell themselves, so the pager is taken out on
    // unmount instead of waiting for the cell to be dropped.
    let dropped_snap_pager = snap_pager.clone();
    use_drop(move || {
        dropped_snap_pager.borrow_mut().take();
    });
    // The slot the pager last settled on or jumped to, so a navigation that only
    // echoes the pager's own settle is not mistaken for a jump.
    let settled_slot_cell = use_hook(|| Rc::new(Cell::new(Some(opening_slot))));
    // The paging the window was last laid out with, so a viewport crossing into
    // or out of the laptop band re-lays the window out on the new pages.
    let paging_cell = use_hook(|| Rc::new(Cell::new(opening_paging)));

    // Settles the pager on the page the scrolling has come to rest on: re-centres
    // the window around it and commits it as navigation.
    let search_dialog_open = editor.search_dialog_open();
    let settle_unit_ids_memo = unit_ids_memo;
    let mut settle_rested_slot = rested_slot;
    let mut settle_window = window;
    let mut settle_pending_anchor = pending_anchor;
    let settle_settled_slot = settled_slot_cell.clone();
    let settle_snap_pager = snap_pager.clone();
    use_effect(move || {
        let Some(settled_slot) = *settle_rested_slot.read() else {
            return;
        };
        settle_rested_slot.set(None);
        let current_window = *settle_window.peek();
        let unit_ids = settle_unit_ids_memo.peek().clone();
        let races = unit_races_memo.peek().clone();
        let paging = PagerPaging::new(*laptop_viewport.peek());
        let roster_len = paging.page_count(&races);
        settle_settled_slot.set(Some(settled_slot));
        // Trim the window back to the pages around the one on screen once it has
        // grown large. The page on screen keeps exactly the offset it has now;
        // the anchor effect places it again once the far pages are dropped.
        let first_slot = current_window.first_slot();
        let slots_above = settled_slot - first_slot;
        let Ok(mounted_above) = usize::try_from(slots_above) else {
            return;
        };
        let too_many_mounted = current_window.slot_count() > MAX_MOUNTED_PAGES;
        if too_many_mounted {
            let offset_from_top = settle_snap_pager
                .borrow()
                .as_ref()
                .and_then(|pager| pager.offset_from_top(mounted_above))
                .unwrap_or(0.0);
            let trimmed = PagerWindow::centered_on(settled_slot, TRIM_BUFFER, roster_len);
            settle_window.set(trimmed);
            let kept_page = PagerAnchor::new(settled_slot, offset_from_top);
            settle_pending_anchor.set(Some(kept_page));
        }
        // The search dialog owns the navigation while it is open, so a settle of
        // the pager underneath must not overwrite the unit it navigated to.
        if *search_dialog_open.peek() {
            return;
        }
        let Some(settled_page) = RosterSlot::unit_index(settled_slot, roster_len) else {
            return;
        };
        let settled_unit_index = paging.first_unit_of(settled_page, &races);
        let Some(settled_unit_id) = unit_ids.get(settled_unit_index).copied() else {
            return;
        };
        // Only scrolling onto a different page is navigation. Scrolling that
        // settles back onto the page it started on is already current, whichever
        // of its units is selected, so back and forward never get a duplicate entry.
        let selected_page = selected_unit_id
            .peek()
            .and_then(|unit_id| unit_ids.iter().position(|id| *id == unit_id))
            .map(|unit_index| paging.page_of(unit_index, &races));
        if selected_page == Some(settled_page) {
            return;
        }
        // A slot id is the ability itself, not the ability on this unit, so a
        // surviving selection would light up on every card that shares it. Each
        // is written only when it actually changes: every grid of every mounted
        // card recomputes its tiles on a write, even one that leaves it equal.
        let mut selected_slot_signal = selected_slot;
        let mut selected_from_research_signal = selected_from_research;
        let mut selected_from_uprooted_signal = selected_from_uprooted;
        if selected_slot_signal.peek().is_some() {
            selected_slot_signal.set(None);
        }
        if *selected_from_research_signal.peek() {
            selected_from_research_signal.set(false);
        }
        if *selected_from_uprooted_signal.peek() {
            selected_from_uprooted_signal.set(false);
        }
        // `open_unit` derives the race and the unit mode from the unit and
        // pushes a navigation snapshot, so scrolling keeps the race theme, the
        // `unit` query parameter and the history entry in step with the card.
        navigation.open_unit(settled_unit_id);
    });

    // A unit chosen anywhere but the pager — a race tab, the search dialog, back
    // and forward — re-centres the window straight on it and aligns its page.
    // Only that one window is ever mounted, so the pager never renders its way
    // through the units in between.
    let jump_unit_ids_memo = unit_ids_memo;
    let mut jump_window = window;
    let mut jump_pending_anchor = pending_anchor;
    let jump_settled_slot = settled_slot_cell.clone();
    let jump_paging = paging_cell.clone();
    use_effect(move || {
        let unit_ids = jump_unit_ids_memo();
        let races = unit_races_memo();
        let paging = PagerPaging::new(*laptop_viewport.read());
        let roster_len = paging.page_count(&races);
        let Some(target_unit_id) = *selected_unit_id.read() else {
            return;
        };
        let Some(target_index) = unit_ids
            .iter()
            .position(|unit_id| *unit_id == target_unit_id)
        else {
            return;
        };
        let target_page = paging.page_of(target_index, &races);
        let current_window = *jump_window.peek();
        // New pages number their slots afresh, so a change of paging lays the
        // window out from scratch rather than from the old slot numbers.
        let paging_changed = jump_paging.get() != paging;
        jump_paging.set(paging);
        let target_slot = if paging_changed {
            isize::try_from(target_page).unwrap_or(0)
        } else {
            current_window.nearest_slot_of(target_page, roster_len)
        };
        // A settle commits the page it rested on, which is already on screen.
        if !paging_changed && jump_settled_slot.get() == Some(target_slot) {
            return;
        }
        jump_settled_slot.set(Some(target_slot));
        let jumped = PagerWindow::centered_on(target_slot, CARD_WINDOW_BUFFER, roster_len);
        jump_window.set(jumped);
        jump_pending_anchor.set(Some(PagerAnchor::aligned(target_slot)));
    });

    // Applies the pending anchor once the window it belongs to is in the DOM.
    let anchor_window = window;
    let mut anchor_pending = pending_anchor;
    let anchor_snap_pager = snap_pager.clone();
    use_effect(move || {
        let _ = pager_mounted.read();
        let current_window = *anchor_window.read();
        let Some(anchor) = *anchor_pending.read() else {
            return;
        };
        let slots_above = anchor.slot() - current_window.first_slot();
        let Ok(page_index) = usize::try_from(slots_above) else {
            return;
        };
        let placed = anchor_snap_pager
            .borrow()
            .as_ref()
            .is_some_and(|pager| pager.place(page_index, anchor.offset_from_top()));
        if placed {
            anchor_pending.set(None);
        }
    });

    // Mounting more pages near an end, while the scrolling is under way. One
    // extension at a time: the next waits until the pages of this one are in.
    let extension_pending = use_hook(|| Rc::new(Cell::new(false)));
    let extended_window = window;
    let extended_snap_pager = snap_pager.clone();
    let extended_pending = extension_pending.clone();
    use_effect(move || {
        let _ = extended_window.read();
        extended_pending.set(false);
        if let Some(pager) = extended_snap_pager.borrow().as_ref() {
            pager.end_mount_above();
        }
    });
    let extend_snap_pager = snap_pager.clone();
    let on_near_edge = move |edge: PagerEdge| {
        if extension_pending.get() {
            return;
        }
        let current_window = *window.peek();
        let paging = PagerPaging::new(*laptop_viewport.peek());
        let roster_len = paging.page_count(&unit_races_memo.peek());
        let extended = match edge {
            PagerEdge::Start => current_window.extended_backward(EXTENSION_STEP, roster_len),
            PagerEdge::End => current_window.extended_forward(EXTENSION_STEP, roster_len),
        };
        if extended == current_window {
            return;
        }
        if edge == PagerEdge::Start
            && let Some(pager) = extend_snap_pager.borrow().as_ref()
        {
            pager.begin_mount_above();
        }
        extension_pending.set(true);
        let mut window_signal = window;
        window_signal.set(extended);
    };

    // Scrolling on to later pages tucks the footer away for good; scrolling
    // back leaves it tucked away.
    let footer_intent = use_hook(|| Rc::new(Cell::new(FooterIntent::default())));
    let mut mounted_pager = pager_mounted;
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(element) = event.data().try_as_web_event() else {
            return;
        };
        let settle_window = window;
        let scroll_footer_intent = footer_intent.clone();
        let near_edge_handler = on_near_edge.clone();
        let callbacks = PagerCallbacks::new(
            move |page_index| {
                let slot = settle_window.peek().slot_at(page_index);
                let mut settled_slot = rested_slot;
                settled_slot.set(Some(slot));
            },
            move |moved_distance| {
                let mut footer_hidden_signal = footer_hidden;
                let mut intent = scroll_footer_intent.get();
                let decision = intent.observe(moved_distance);
                scroll_footer_intent.set(intent);
                let Some(hidden) = decision else {
                    return;
                };
                if *footer_hidden_signal.peek() != hidden {
                    footer_hidden_signal.set(hidden);
                }
            },
            near_edge_handler,
        );
        let pager = SnapPager::attach(element, CARD_SELECTOR, callbacks);
        *snap_pager.borrow_mut() = Some(pager);
        mounted_pager.set(true);
    });

    let unit_ids = unit_ids_memo();
    let races = unit_races_memo();
    let paging = PagerPaging::new(*laptop_viewport.read());
    let page_count = paging.page_count(&races);
    let current_window = *window.read();
    let window_pages = (0..current_window.slot_count())
        .filter_map(|offset| {
            let slot = current_window.slot_at(offset);
            let page_index = RosterSlot::unit_index(slot, page_count)?;
            let unit_ids = paging.units_of(page_index, &unit_ids, &races);
            Some(PagerPageView { unit_ids })
        })
        .collect();

    MobileEditorPresentation {
        onmounted,
        window_pages,
    }
}
