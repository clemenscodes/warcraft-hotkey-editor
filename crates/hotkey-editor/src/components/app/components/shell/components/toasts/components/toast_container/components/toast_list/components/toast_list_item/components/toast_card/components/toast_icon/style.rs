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
        "bg-[color-mix(in_oklab,var(--color-warcraft-success)_18%,transparent)]",
        "text-warcraft-success",
    ],
    Error => tw![
        "bg-[color-mix(in_oklab,var(--color-race-orc)_20%,transparent)]",
        "text-race-orc",
    ],
    Warning => tw![
        "bg-[color-mix(in_oklab,var(--color-warcraft-gold)_20%,transparent)]",
        "text-warcraft-gold",
    ],
    Info => tw![
        "bg-[color-mix(in_oklab,var(--color-race-human)_18%,transparent)]",
        "text-race-human",
    ],
}
