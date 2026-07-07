use super::state::KeyCaptureState;
use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "uppercase",
        "tracking-[0.04em]",
        "text-3xl",
        "leading-none",
        "px-6",
        "py-3.5",
        "min-w-[18rem]",
        "cursor-pointer",
        "whitespace-nowrap",
        "border",
        "rounded-hairline",
        "bg-panel-dark",
        "transition-[filter,border-color]",
        "hover:filter-[brightness(1.18)_drop-shadow(0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_40%,transparent))]",
        "hover:border-warcraft-gold/85",
        "kb-focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:filter-[drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent))]",
        "relative",
        "group/tooltip",
    ],
    mobile: tw![
        "mobile:min-w-22",
        "mobile:max-w-56",
        "mobile:px-3",
        "mobile:py-2",
        "mobile:text-sm",
        "mobile:flex-[0_0_auto]",
        "mobile:overflow-hidden",
        "mobile:text-ellipsis",
        "mobile:touch-manipulation",
    ],
    tablet: tw![
        "tablet:min-w-22",
        "tablet:max-w-56",
        "tablet:px-3",
        "tablet:py-2",
        "tablet:text-sm",
        "tablet:flex-[0_0_auto]",
        "tablet:overflow-hidden",
        "tablet:text-ellipsis",
        "tablet:touch-manipulation",
    ],
}

states! {
    KeyCaptureState,
    Normal => tw![
        "text-warcraft-gold",
        "border-warcraft-gold/45",
        "text-shadow-drop",
    ],
    Conflict => tw![
        "text-warcraft-danger",
        "border-warcraft-danger/65",
        "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_10px_color-mix(in_oklab,var(--color-warcraft-danger)_50%,transparent)]",
    ],
}
