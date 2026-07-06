use super::context::{AddToastRequest, ToastRecord, Toasts};
use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The upper bound on live toasts; the oldest is dropped once exceeded.
const MAX_TOASTS: usize = 10;

/// Monotonic source of unique toast ids across the whole app.
static NEXT_TOAST_ID: AtomicUsize = AtomicUsize::new(0);

/// The shaped result of [`use_toast_provider`]: the current queue for rendering
/// and the callback that removes a toast by id.
pub struct ToastProviderModel {
    records: Vec<ToastRecord>,
    on_remove: Callback<usize>,
}

impl ToastProviderModel {
    pub fn records(&self) -> Vec<ToastRecord> {
        self.records.clone()
    }

    pub fn on_remove(&self) -> Callback<usize> {
        self.on_remove
    }
}

/// Wire up the toast queue: create the backing signal, register the add and
/// remove callbacks, provide the [`Toasts`] dispatch handle as context, and hand
/// the body the current queue plus the remove callback. Render the queue with
/// your own visuals.
pub fn use_toast_provider() -> ToastProviderModel {
    let mut records = use_signal(Vec::<ToastRecord>::new);
    let on_remove = use_callback(move |id: usize| {
        records.write().retain(|record| record.id() != id);
    });
    let add_toast = use_callback(move |request: AddToastRequest| {
        let id = NEXT_TOAST_ID.fetch_add(1, Ordering::SeqCst);
        let record = ToastRecord::new(id, request);
        let mut queue = records.write();
        queue.push(record);
        while queue.len() > MAX_TOASTS {
            queue.remove(0);
        }
    });
    let toasts = Toasts::new(add_toast);
    use_context_provider(|| toasts);
    let current = records.read().clone();
    ToastProviderModel {
        records: current,
        on_remove,
    }
}
