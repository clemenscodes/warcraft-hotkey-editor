use std::cell::{Cell, RefCell};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(crate) const DRAG_MOVEMENT_THRESHOLD_PIXELS: f64 = 4.0;
pub(crate) const TOUCH_CANCEL_THRESHOLD_PIXELS: f64 = 12.0;
pub(crate) const LONG_PRESS_MS: i32 = 300;

#[derive(Clone, Copy)]
pub(crate) struct DragOrigin {
    pub(crate) cursor_horizontal_position: f64,
    pub(crate) cursor_vertical_position: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DragMovePoint {
    pub(crate) client_horizontal: f64,
    pub(crate) client_vertical: f64,
}

use crate::services::drag_state::DragFollowerVisual;
use warcraft_keybinds::GridCoordinate;

pub(crate) struct PendingDragData {
    pub(crate) grid_id: &'static str,
    pub(crate) coordinate: GridCoordinate,
    pub(crate) visual: DragFollowerVisual,
    pub(crate) click_offset_horizontal: f64,
    pub(crate) click_offset_vertical: f64,
    pub(crate) tile_width: f64,
    pub(crate) tile_height: f64,
    pub(crate) tile_element: web_sys::Element,
    pub(crate) grid_element: web_sys::Element,
    pub(crate) pointer_id: i32,
    pub(crate) last_cursor_horizontal_position: f64,
    pub(crate) last_cursor_vertical_position: f64,
    pub(crate) is_touch: bool,
}

pub(crate) type TouchScrollLock = Closure<dyn FnMut(web_sys::Event)>;
pub(crate) type DragRafClosure = Closure<dyn FnMut(f64)>;
thread_local! {
    pub(crate) static SUPPRESS_NEXT_CLICK: Cell<bool> = const { Cell::new(false) };

    pub(crate) static SUPPRESS_NEXT_DOUBLE_CLICK: Cell<bool> = const {
        Cell::new(false)
    };

    pub(crate) static DRAG_ORIGIN: Cell<Option<DragOrigin>> = const { Cell::new(None) };

    pub(crate) static DID_DRAG_MOVE: Cell<bool> = const { Cell::new(false) };

    pub(crate) static PENDING_DRAG: RefCell<Option<PendingDragData>> = const {
        RefCell::new(None)
    };

    pub(crate) static DRAG_SOURCE_GRID_ELEMENT: RefCell<Option<web_sys::Element>> = const {
        RefCell::new(None)
    };

    pub(crate) static TOUCH_STARTED: Cell<bool> = const { Cell::new(false) };

    pub(crate) static TOUCH_LONG_PRESS_TIMER_ID: Cell<Option<i32>> = const {
        Cell::new(None)
    };

    pub(crate) static TOUCH_LONG_PRESS_CLOSURE: RefCell<Option<Closure<dyn FnMut()>>> = const {
        RefCell::new(None)
    };

    pub(crate) static TOUCH_SCROLL_LOCK: RefCell<Option<TouchScrollLock>> = const {
        RefCell::new(None)
    };

    pub(crate) static LATEST_DRAG_MOVE: Cell<Option<DragMovePoint>> = const { Cell::new(None) };

    pub(crate) static DRAG_RAF_HANDLE: Cell<Option<i32>> = const { Cell::new(None) };

    pub(crate) static DRAG_RAF_CLOSURE: RefCell<Option<DragRafClosure>> = const {
        RefCell::new(None)
    };
}

pub(crate) fn cancel_long_press() {
    if let Some(id) = TOUCH_LONG_PRESS_TIMER_ID.with(|cell| cell.replace(None))
        && let Some(window) = web_sys::window()
    {
        window.clear_timeout_with_handle(id);
    }
    TOUCH_LONG_PRESS_CLOSURE.with(|cell| cell.borrow_mut().take());
}

pub(crate) fn install_scroll_lock() {
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

pub(crate) fn remove_scroll_lock() {
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

pub(crate) fn cancel_drag_raf() {
    if let Some(handle) = DRAG_RAF_HANDLE.with(|cell| cell.replace(None))
        && let Some(window) = web_sys::window()
    {
        let _ = window.cancel_animation_frame(handle);
    }
    LATEST_DRAG_MOVE.with(|cell| cell.set(None));
    DRAG_RAF_CLOSURE.with(|cell| cell.borrow_mut().take());
}

pub(crate) fn reset() {
    cancel_long_press();
    remove_scroll_lock();
    cancel_drag_raf();
    TOUCH_STARTED.with(|cell| cell.set(false));
    DID_DRAG_MOVE.with(|cell| cell.set(false));
    DRAG_ORIGIN.with(|cell| cell.set(None));
    PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
    DRAG_SOURCE_GRID_ELEMENT.with(|cell| *cell.borrow_mut() = None);
    SUPPRESS_NEXT_CLICK.with(|cell| cell.set(false));
}
