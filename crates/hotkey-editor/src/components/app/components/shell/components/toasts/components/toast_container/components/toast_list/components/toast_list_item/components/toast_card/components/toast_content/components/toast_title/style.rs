use crate::components::app::components::shell::components::toasts::ToastType;
use tw_macro::tw;

classes! {
    base: tw![
        "text-2xl",
        "leading-title",
        "text-warcraft-gold",
    ],
}

states! {
    ToastType,
    Success => tw!["text-warcraft-success"],
    Error => tw!["text-race-orc"],
    Warning => tw!["text-warcraft-gold"],
    Info => tw![],
}
