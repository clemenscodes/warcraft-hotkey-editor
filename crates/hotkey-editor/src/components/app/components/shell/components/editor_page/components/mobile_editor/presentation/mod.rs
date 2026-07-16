use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;
use warcraft_api::{Race, WarcraftApi, WarcraftObjectId};

const CARD_WINDOW_BUFFER: usize = 1;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct UnitOrder {
    race_rank: u8,
    name: &'static str,
    unit_id: WarcraftObjectId,
}

impl UnitOrder {
    fn rank_of(race: Option<Race>) -> u8 {
        let rank: u8 = match race {
            Some(Race::Human) => 0,
            Some(Race::Orc) => 1,
            Some(Race::Nightelf) => 2,
            Some(Race::Undead) => 3,
            Some(Race::Neutral) => 4,
            None => 5,
        };
        rank
    }
}

pub(super) struct MobileEditorPresentation {
    pub(super) onmounted: EventHandler<MountedEvent>,
    pub(super) onscrollend: EventHandler<ScrollEvent>,
    pub(super) top_spacer_px: i32,
    pub(super) bottom_spacer_px: i32,
    pub(super) window_unit_ids: Vec<WarcraftObjectId>,
}

pub(super) fn use_mobile_editor() -> MobileEditorPresentation {
    let unit_ids_memo = use_memo(|| {
        let api = WarcraftApi::default();
        let mut ordered: Vec<UnitOrder> = api
            .unit()
            .all()
            .map(|unit| {
                let race_rank = UnitOrder::rank_of(unit.race());
                let name = unit.name().unwrap_or("(unnamed)");
                let unit_id = unit.id();
                UnitOrder {
                    race_rank,
                    name,
                    unit_id,
                }
            })
            .collect();
        ordered.sort();
        let ids: Rc<[WarcraftObjectId]> = ordered.into_iter().map(|order| order.unit_id).collect();
        ids
    });

    let viewport_px = use_signal::<i32>(|| 0);
    let active_index = use_signal::<usize>(|| 0);
    let element_ref = use_hook(|| Rc::new(RefCell::new(None::<web_sys::Element>)));

    let unit_ids = unit_ids_memo();
    let unit_count = unit_ids.len();

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

    let scroll_element_ref = element_ref.clone();
    let mut scroll_viewport_px = viewport_px;
    let mut scroll_active_index = active_index;
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
        let rounded_index = (scroll_top + measured_height / 2) / measured_height;
        let last_index = unit_count.saturating_sub(1);
        let clamped_index = usize::try_from(rounded_index).unwrap_or(0).min(last_index);
        if *scroll_viewport_px.peek() != measured_height {
            scroll_viewport_px.set(measured_height);
        }
        if *scroll_active_index.peek() != clamped_index {
            scroll_active_index.set(clamped_index);
        }
    });

    let current_viewport_px = *viewport_px.read();
    let current_active_index = *active_index.read();

    let window_start;
    let window_end;
    if current_viewport_px <= 0 {
        window_start = 0;
        window_end = unit_count.min(2 * CARD_WINDOW_BUFFER + 1);
    } else {
        window_start = current_active_index.saturating_sub(CARD_WINDOW_BUFFER);
        window_end = (current_active_index + CARD_WINDOW_BUFFER + 1).min(unit_count);
    }

    let window_unit_ids = unit_ids[window_start..window_end].to_vec();
    let leading_cards = i32::try_from(window_start).unwrap_or(0);
    let trailing_cards = i32::try_from(unit_count - window_end).unwrap_or(0);
    let top_spacer_px = leading_cards * current_viewport_px;
    let bottom_spacer_px = trailing_cards * current_viewport_px;

    MobileEditorPresentation {
        onmounted,
        onscrollend,
        top_spacer_px,
        bottom_spacer_px,
        window_unit_ids,
    }
}
