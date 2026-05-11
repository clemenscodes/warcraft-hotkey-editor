use std::cell::{Cell, RefCell};

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(super) const DRAG_MOVEMENT_THRESHOLD_PIXELS: f64 = 4.0;
pub(super) const TOUCH_CANCEL_THRESHOLD_PIXELS: f64 = 12.0;
pub(super) const LONG_PRESS_MS: i32 = 300;

#[derive(Clone, Copy)]
pub(super) struct DragOrigin {
    pub(super) cursor_horizontal_position: f64,
    pub(super) cursor_vertical_position: f64,
}

use crate::model::grid::{DragFollowerVisual, GridSlotId};

pub(super) struct PendingDragData {
    pub(super) source_slot: GridSlotId,
    pub(super) section: &'static str,
    pub(super) column: u8,
    pub(super) row: u8,
    pub(super) visual: DragFollowerVisual,
    pub(super) click_offset_horizontal: f64,
    pub(super) click_offset_vertical: f64,
    pub(super) tile_width: f64,
    pub(super) tile_height: f64,
    pub(super) tile_element: web_sys::Element,
    pub(super) pointer_id: i32,
    pub(super) last_cursor_horizontal_position: f64,
    pub(super) last_cursor_vertical_position: f64,
    pub(super) is_touch: bool,
}

pub(super) type TouchScrollLock = Closure<dyn FnMut(web_sys::Event)>;

thread_local! {
    /// Set on a successful drag-end so the synthetic `click` that fires after
    /// `pointerup` does not also re-select the source tile.
    pub(super) static SUPPRESS_NEXT_CLICK: Cell<bool> = const { Cell::new(false) };

    /// Cursor position at `pointerdown`. Used to decide whether the user
    /// actually dragged (vs. just clicked) so we know whether to suppress the
    /// trailing click.
    pub(super) static DRAG_ORIGIN: Cell<Option<DragOrigin>> = const { Cell::new(None) };

    /// Set true once the cursor has travelled past the movement threshold.
    pub(super) static DID_DRAG_MOVE: Cell<bool> = const { Cell::new(false) };

    /// Drag setup data captured at `pointerdown`, not yet committed to signals.
    pub(super) static PENDING_DRAG: RefCell<Option<PendingDragData>> = const { RefCell::new(None) };

    /// Set when a touch/pen `pointerdown` fires so the compatibility `mouse`
    /// `pointerdown` that browsers synthesise afterward is discarded.
    pub(super) static TOUCH_STARTED: Cell<bool> = const { Cell::new(false) };

    /// ID returned by `setTimeout` for the touch long-press timer.
    pub(super) static TOUCH_LONG_PRESS_TIMER_ID: Cell<Option<i32>> = const { Cell::new(None) };

    /// Non-passive `touchmove` listener installed only while a touch drag is active.
    pub(super) static TOUCH_SCROLL_LOCK: RefCell<Option<TouchScrollLock>> = const { RefCell::new(None) };
}

pub(super) struct DragThreadState;

impl DragThreadState {
    pub(super) fn cancel_long_press() {
        if let Some(id) = TOUCH_LONG_PRESS_TIMER_ID.with(|cell| cell.replace(None))
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(id);
        }
    }

    pub(super) fn install_scroll_lock() {
        TOUCH_SCROLL_LOCK.with(|cell| {
            if cell.borrow().is_some() {
                return;
            }
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let cb = Closure::<dyn FnMut(web_sys::Event)>::new(|event: web_sys::Event| {
                event.prevent_default();
            });
            let options = web_sys::AddEventListenerOptions::new();
            options.set_capture(true);
            options.set_passive(false);
            if document
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "touchmove",
                    cb.as_ref().unchecked_ref(),
                    &options,
                )
                .is_ok()
            {
                *cell.borrow_mut() = Some(cb);
            }
        });
    }

    pub(super) fn remove_scroll_lock() {
        let cb_option = TOUCH_SCROLL_LOCK.with(|cell| cell.borrow_mut().take());
        let Some(cb) = cb_option else {
            return;
        };
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            let _ = document.remove_event_listener_with_callback_and_bool(
                "touchmove",
                cb.as_ref().unchecked_ref(),
                true,
            );
        }
    }

    pub(super) fn reset() {
        Self::cancel_long_press();
        Self::remove_scroll_lock();
        TOUCH_STARTED.with(|cell| cell.set(false));
        DID_DRAG_MOVE.with(|cell| cell.set(false));
        DRAG_ORIGIN.with(|cell| cell.set(None));
        PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
        SUPPRESS_NEXT_CLICK.with(|cell| cell.set(false));
    }
}
