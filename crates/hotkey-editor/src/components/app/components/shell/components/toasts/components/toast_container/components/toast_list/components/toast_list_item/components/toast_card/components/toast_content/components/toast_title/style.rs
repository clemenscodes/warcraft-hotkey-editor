use crate::components::app::components::shell::components::toasts::ToastType;
use tw_macro::tw;

classes! {
    base: tw![
        "text-warcraft-gold",
        "uppercase",
        "tracking-[0.06em]",
        "text-[1.9rem]",
        "leading-[1.2]",
        "text-shadow-drop",
    ],
}

states! {
    ToastType,
    Success => tw!["text-warcraft-success"],
    Error => tw!["text-race-orc"],
    Warning => tw!["text-warcraft-gold"],
    Info => tw![],
}
