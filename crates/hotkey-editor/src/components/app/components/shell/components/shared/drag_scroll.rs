use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

const DRAG_SCROLL_THRESHOLD_PIXELS: f64 = 5.0;
const MOMENTUM_DECAY_TAU_MILLISECONDS: f64 = 325.0;
const MOMENTUM_MINIMUM_VELOCITY: f64 = 0.02;
const MOMENTUM_MAXIMUM_VELOCITY: f64 = 4.0;
const MOMENTUM_VELOCITY_SMOOTHING: f64 = 0.7;
const MOMENTUM_MINIMUM_LAUNCH_VELOCITY: f64 = 0.05;

type MomentumFrameLoop = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct PendingDragScroll {
    start_client_horizontal: f64,
    start_client_vertical: f64,
    start_scroll_left: f64,
    start_scroll_top: f64,
    pointer_id: i32,
}

#[derive(Default)]
struct DragScrollState {
    viewport: Option<web_sys::Element>,
    pending: Option<PendingDragScroll>,
    dragging: bool,
    suppress_next_click: bool,
    click_listener: Option<Closure<dyn FnMut(web_sys::Event)>>,
    velocity_left: f64,
    velocity_top: f64,
    last_sample_time: f64,
    last_sample_scroll_left: f64,
    last_sample_scroll_top: f64,
    momentum_last_time: f64,
    momentum_frame: Option<i32>,
    momentum_loop: Option<MomentumFrameLoop>,
}

impl DragScrollState {
    fn begin_velocity_tracking(&mut self, timestamp: f64, scroll_left: f64, scroll_top: f64) {
        self.velocity_left = 0.0;
        self.velocity_top = 0.0;
        self.last_sample_time = timestamp;
        self.last_sample_scroll_left = scroll_left;
        self.last_sample_scroll_top = scroll_top;
    }

    fn record_scroll_velocity(&mut self, timestamp: f64, scroll_left: f64, scroll_top: f64) {
        let elapsed = timestamp - self.last_sample_time;
        if elapsed <= 0.0 {
            return;
        }
        let instant_left = (scroll_left - self.last_sample_scroll_left) / elapsed;
        let instant_top = (scroll_top - self.last_sample_scroll_top) / elapsed;
        let previous_weight = MOMENTUM_VELOCITY_SMOOTHING;
        let instant_weight = 1.0 - MOMENTUM_VELOCITY_SMOOTHING;
        let blended_left = self.velocity_left * previous_weight + instant_left * instant_weight;
        let blended_top = self.velocity_top * previous_weight + instant_top * instant_weight;
        self.velocity_left =
            blended_left.clamp(-MOMENTUM_MAXIMUM_VELOCITY, MOMENTUM_MAXIMUM_VELOCITY);
        self.velocity_top =
            blended_top.clamp(-MOMENTUM_MAXIMUM_VELOCITY, MOMENTUM_MAXIMUM_VELOCITY);
        self.last_sample_time = timestamp;
        self.last_sample_scroll_left = scroll_left;
        self.last_sample_scroll_top = scroll_top;
    }

    fn launch_velocity(&self) -> f64 {
        self.velocity_left.hypot(self.velocity_top)
    }

    fn step_momentum(&mut self, timestamp: f64) -> bool {
        let Some(viewport) = self.viewport.clone() else {
            return false;
        };
        let elapsed = timestamp - self.momentum_last_time;
        self.momentum_last_time = timestamp;
        if elapsed <= 0.0 {
            return true;
        }
        let current_left = f64::from(viewport.scroll_left());
        let current_top = f64::from(viewport.scroll_top());
        let next_left = current_left + self.velocity_left * elapsed;
        let next_top = current_top + self.velocity_top * elapsed;
        viewport.scroll_to_with_x_and_y(next_left, next_top);
        let decay = (-elapsed / MOMENTUM_DECAY_TAU_MILLISECONDS).exp();
        self.velocity_left *= decay;
        self.velocity_top *= decay;
        let actual_left = f64::from(viewport.scroll_left());
        let actual_top = f64::from(viewport.scroll_top());
        let horizontal_progress = (actual_left - current_left).abs();
        let vertical_progress = (actual_top - current_top).abs();
        let progress = horizontal_progress + vertical_progress;
        let speed = self.launch_velocity();
        if speed < MOMENTUM_MINIMUM_VELOCITY || progress < 0.5 {
            return false;
        }
        true
    }

    fn cancel_momentum(&mut self) {
        if let Some(handle) = self.momentum_frame.take()
            && let Some(window) = web_sys::window()
        {
            let _ = window.cancel_animation_frame(handle);
        }
        if let Some(holder) = self.momentum_loop.take() {
            let _ = holder.borrow_mut().take();
        }
    }
}

fn request_momentum_frame(state: &Rc<RefCell<DragScrollState>>, frame_loop: &MomentumFrameLoop) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let borrowed_loop = frame_loop.borrow();
    let Some(frame_closure) = borrowed_loop.as_ref() else {
        return;
    };
    let callback = frame_closure.as_ref().unchecked_ref();
    let scheduled = window.request_animation_frame(callback);
    drop(borrowed_loop);
    if let Ok(handle) = scheduled {
        state.borrow_mut().momentum_frame = Some(handle);
    }
}

pub(crate) struct DragScrollBindings {
    pub(crate) onmounted: EventHandler<MountedEvent>,
    pub(crate) onpointerdown: EventHandler<PointerEvent>,
    pub(crate) onpointermove: EventHandler<PointerEvent>,
    pub(crate) onpointerup: EventHandler<PointerEvent>,
    pub(crate) onpointercancel: EventHandler<PointerEvent>,
    pub(crate) onlostpointercapture: EventHandler<PointerEvent>,
}

pub(crate) fn use_drag_scroll() -> DragScrollBindings {
    let state = use_hook(|| Rc::new(RefCell::new(DragScrollState::default())));

    let drop_state = state.clone();
    use_drop(move || {
        let mut borrowed = drop_state.borrow_mut();
        borrowed.cancel_momentum();
        let viewport = borrowed.viewport.clone();
        let listener = borrowed.click_listener.take();
        if let Some(viewport) = viewport
            && let Some(listener) = listener
        {
            let callback = listener.as_ref().unchecked_ref();
            let _ = viewport.remove_event_listener_with_callback_and_bool("click", callback, true);
        }
    });

    let mounted_state = state.clone();
    let onmounted = EventHandler::new(move |event: MountedEvent| {
        let Some(viewport) = event.data().try_as_web_event() else {
            return;
        };
        let listener_state = mounted_state.clone();
        let click_closure =
            Closure::<dyn FnMut(web_sys::Event)>::new(move |click_event: web_sys::Event| {
                let mut borrowed = listener_state.borrow_mut();
                if !borrowed.suppress_next_click {
                    return;
                }
                borrowed.suppress_next_click = false;
                click_event.stop_propagation();
                click_event.prevent_default();
            });
        let listener_options = web_sys::AddEventListenerOptions::new();
        listener_options.set_capture(true);
        let callback = click_closure.as_ref().unchecked_ref();
        let installed = viewport.add_event_listener_with_callback_and_add_event_listener_options(
            "click",
            callback,
            &listener_options,
        );
        if installed.is_err() {
            return;
        }
        let mut borrowed = mounted_state.borrow_mut();
        borrowed.viewport = Some(viewport);
        borrowed.click_listener = Some(click_closure);
    });

    let down_state = state.clone();
    let onpointerdown = EventHandler::new(move |event: PointerEvent| {
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let pointer_type = web_event.pointer_type();
        if pointer_type != "mouse" {
            return;
        }
        let trigger_button = event.data().trigger_button();
        if trigger_button != Some(MouseButton::Primary) {
            return;
        }
        let mut borrowed = down_state.borrow_mut();
        borrowed.cancel_momentum();
        let Some(viewport) = borrowed.viewport.clone() else {
            return;
        };
        let start_client_horizontal = f64::from(web_event.client_x());
        let start_client_vertical = f64::from(web_event.client_y());
        let start_scroll_left = f64::from(viewport.scroll_left());
        let start_scroll_top = f64::from(viewport.scroll_top());
        let pointer_id = web_event.pointer_id();
        let pending = PendingDragScroll {
            start_client_horizontal,
            start_client_vertical,
            start_scroll_left,
            start_scroll_top,
            pointer_id,
        };
        borrowed.pending = Some(pending);
        borrowed.dragging = false;
        borrowed.suppress_next_click = false;
    });

    let move_state = state.clone();
    let onpointermove = EventHandler::new(move |event: PointerEvent| {
        let mut borrowed = move_state.borrow_mut();
        let Some(pending) = borrowed.pending else {
            return;
        };
        let Some(viewport) = borrowed.viewport.clone() else {
            return;
        };
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let current_pointer_id = web_event.pointer_id();
        if current_pointer_id != pending.pointer_id {
            return;
        }
        let event_timestamp = web_event.time_stamp();
        let cursor_client_horizontal = f64::from(web_event.client_x());
        let cursor_client_vertical = f64::from(web_event.client_y());
        let horizontal_delta = cursor_client_horizontal - pending.start_client_horizontal;
        let vertical_delta = cursor_client_vertical - pending.start_client_vertical;
        if !borrowed.dragging {
            let distance_squared =
                horizontal_delta * horizontal_delta + vertical_delta * vertical_delta;
            let threshold_squared = DRAG_SCROLL_THRESHOLD_PIXELS * DRAG_SCROLL_THRESHOLD_PIXELS;
            if distance_squared <= threshold_squared {
                return;
            }
            let pointer_id = pending.pointer_id;
            if viewport.set_pointer_capture(pointer_id).is_err() {
                return;
            }
            borrowed.dragging = true;
            borrowed.suppress_next_click = true;
            let start_scroll_left = pending.start_scroll_left;
            let start_scroll_top = pending.start_scroll_top;
            borrowed.begin_velocity_tracking(event_timestamp, start_scroll_left, start_scroll_top);
        }
        let target_scroll_left = pending.start_scroll_left - horizontal_delta;
        let target_scroll_top = pending.start_scroll_top - vertical_delta;
        viewport.scroll_to_with_x_and_y(target_scroll_left, target_scroll_top);
        borrowed.record_scroll_velocity(event_timestamp, target_scroll_left, target_scroll_top);
    });

    let up_state = state.clone();
    let onpointerup = EventHandler::new(move |event: PointerEvent| {
        let mut borrowed = up_state.borrow_mut();
        let was_dragging = borrowed.dragging;
        let viewport = borrowed.viewport.clone();
        let pending = borrowed.pending;
        if was_dragging
            && let Some(viewport) = viewport
            && let Some(pending) = pending
        {
            let pointer_id = pending.pointer_id;
            let _ = viewport.release_pointer_capture(pointer_id);
        }
        borrowed.pending = None;
        borrowed.dragging = false;
        if !was_dragging {
            return;
        }
        let launch_velocity = borrowed.launch_velocity();
        if launch_velocity < MOMENTUM_MINIMUM_LAUNCH_VELOCITY {
            return;
        }
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let release_timestamp = web_event.time_stamp();
        borrowed.momentum_last_time = release_timestamp;
        drop(borrowed);

        let frame_loop: MomentumFrameLoop = Rc::new(RefCell::new(None));
        let closure_state = up_state.clone();
        let closure_loop = frame_loop.clone();
        let frame_closure = Closure::<dyn FnMut(f64)>::new(move |timestamp: f64| {
            let should_continue = closure_state.borrow_mut().step_momentum(timestamp);
            if !should_continue {
                closure_state.borrow_mut().cancel_momentum();
                return;
            }
            request_momentum_frame(&closure_state, &closure_loop);
        });
        *frame_loop.borrow_mut() = Some(frame_closure);
        request_momentum_frame(&up_state, &frame_loop);
        up_state.borrow_mut().momentum_loop = Some(frame_loop);
    });

    let cancel_state = state.clone();
    let onpointercancel = EventHandler::new(move |_event: PointerEvent| {
        let mut borrowed = cancel_state.borrow_mut();
        borrowed.pending = None;
        borrowed.dragging = false;
        borrowed.suppress_next_click = false;
    });

    let lost_state = state.clone();
    let onlostpointercapture = EventHandler::new(move |_event: PointerEvent| {
        let mut borrowed = lost_state.borrow_mut();
        borrowed.pending = None;
        borrowed.dragging = false;
    });

    DragScrollBindings {
        onmounted,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
    }
}
