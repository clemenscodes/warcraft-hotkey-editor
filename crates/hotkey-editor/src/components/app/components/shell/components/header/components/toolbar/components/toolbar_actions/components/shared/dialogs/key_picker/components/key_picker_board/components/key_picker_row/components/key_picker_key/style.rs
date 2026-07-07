use super::state::KeyPickerKeyState;
use tw_macro::tw;

classes! {
    base: tw![
        "size-28",
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-1",
        "p-0",
        "border",
        "rounded-tile",
        "text-4xl",
        "leading-none",
        "cursor-pointer",
        "[transition:border-color_0.12s_ease,background_0.12s_ease,box-shadow_0.12s_ease]",
        "text-shadow-outline",
        "[&:hover:not(:disabled)]:border-warcraft-gold",
        "[&:hover:not(:disabled)]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_12%,transparent)]",
        "[&:hover:not(:disabled)]:shadow-glow-8",
        "focus:outline-none",
        "kb-focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:focus-ring",
        "data-[special=true]:w-auto",
        "data-[special=true]:min-w-28",
        "data-[special=true]:px-3.5",
        "data-[special=true]:text-xl",
        "data-[special=true]:whitespace-nowrap",
    ],
    mobile: tw![
        "mobile:size-16",
        "mobile:text-xl",
        "mobile:data-[special=true]:min-w-16",
        "mobile:data-[special=true]:px-2",
        "mobile:data-[special=true]:text-base",
    ],
}

states! {
    KeyPickerKeyState,
    Available => tw![
        "[background:color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)]",
        "border-warcraft-gold-border",
        "text-warcraft-gold",
    ],
    Current => tw![
        "bg-panel-gold-diag-32",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "[box-shadow:var(--shadow-glow-gold-current)]",
    ],
    Conflict => tw![
        "[background:color-mix(in_oklab,var(--color-race-orc-strong)_50%,transparent)]",
        "border-race-orc-strong",
        "text-race-orc",
        "cursor-not-allowed",
        "opacity-85",
    ],
}
