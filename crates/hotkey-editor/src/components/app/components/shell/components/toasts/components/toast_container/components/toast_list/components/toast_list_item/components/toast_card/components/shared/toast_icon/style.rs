use crate::components::app::components::shell::components::toasts::ToastType;
use tw_macro::tw;

classes! {
    base: tw![
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "w-12",
        "h-12",
        "rounded-full",
        "self-center",
        "[&>svg]:w-8",
        "[&>svg]:h-8",
    ],
}

states! {
    ToastType,
    Success => tw![
        "bg-warcraft-success/18",
        "text-warcraft-success",
    ],
    Error => tw![
        "bg-race-orc/20",
        "text-race-orc",
    ],
    Warning => tw![
        "bg-warcraft-gold/20",
        "text-warcraft-gold",
    ],
    Info => tw![
        "bg-race-human/18",
        "text-race-human",
    ],
}
