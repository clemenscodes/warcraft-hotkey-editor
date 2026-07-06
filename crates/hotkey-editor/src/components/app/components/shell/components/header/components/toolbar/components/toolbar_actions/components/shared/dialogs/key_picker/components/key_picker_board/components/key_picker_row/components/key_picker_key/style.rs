use super::state::KeyPickerKeyState;
use tw_macro::tw;

classes! {
    base: tw![
        "w-[clamp(5rem,7.5vw,11rem)]",
        "h-[clamp(5rem,7.5vw,11rem)]",
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-[0.2rem]",
        "p-0",
        "border",
        "rounded-[6px]",
        "text-[clamp(2rem,3.5vw,5rem)]",
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
        "data-[special=true]:min-w-[clamp(5rem,7.5vw,11rem)]",
        "data-[special=true]:px-[0.85rem]",
        "data-[special=true]:text-[clamp(1rem,1.6vw,2rem)]",
        "data-[special=true]:whitespace-nowrap",
    ],
    mobile: tw![
        "mobile:w-[clamp(2.5rem,8.5vw,5rem)]",
        "mobile:h-[clamp(2.5rem,8.5vw,5rem)]",
        "mobile:text-[clamp(1.1rem,3.5vw,2.2rem)]",
        "mobile:data-[special=true]:min-w-[clamp(2.5rem,8.5vw,5rem)]",
        "mobile:data-[special=true]:px-[0.55rem]",
        "mobile:data-[special=true]:text-[clamp(0.75rem,2.6vw,1.25rem)]",
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
        "[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent),inset_0_0_10px_color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)]",
    ],
    Conflict => tw![
        "[background:color-mix(in_oklab,var(--color-race-orc-strong)_50%,transparent)]",
        "border-race-orc-strong",
        "text-race-orc",
        "cursor-not-allowed",
        "opacity-85",
    ],
}
