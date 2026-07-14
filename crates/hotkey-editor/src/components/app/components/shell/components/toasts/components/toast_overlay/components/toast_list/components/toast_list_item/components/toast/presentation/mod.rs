use super::model::ToastModel;
use dioxus::prelude::*;

pub fn use_toast_auto_dismiss(props: &ToastModel) {
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
