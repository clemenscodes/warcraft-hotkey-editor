use super::model::ToastCardModel;
use dioxus::prelude::*;

/// Schedule the auto-dismissal of a non-permanent toast: once mounted, wait its
/// resolved duration, then ask the provider to remove it. Permanent toasts and
/// toasts without a duration are left in place.
pub fn use_toast_auto_dismiss(props: &ToastCardModel) {
    let id = props.record.id();
    let duration = props.record.duration();
    let permanent = props.record.permanent();
    let on_remove = props.on_remove;
    use_effect(move || {
        if permanent {
            return;
        }
        let Some(duration) = duration else {
            return;
        };
        let millis = u32::try_from(duration.as_millis()).unwrap_or(u32::MAX);
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(millis).await;
            on_remove.call(id);
        });
    });
}
