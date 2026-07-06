use tw_macro::tw;
use warcraft_api::Race;

classes! {
    base: tw![
        "flex-1",
        "min-w-0",
        "min-h-[44px]",
        "px-[0.5rem]",
        "bg-warcraft-bg-mid/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-[8px]",
        "text-warcraft-text-secondary",
        "text-[0.95rem]",
        "tracking-[0.04em]",
        "uppercase",
        "text-center",
        "cursor-pointer",
        "transition-all",
        "duration-[0.12s]",
        "whitespace-nowrap",
        "overflow-hidden",
        "text-ellipsis",
        "hover:bg-warcraft-blue-deep/70",
        "hover:text-white",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:shadow-ring-hl-2",
        "data-[active=true]:bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-blue)_95%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-panel)_95%,transparent)_100%)]",
    ],
    mobile: tw![
        "mobile:text-[clamp(11px,2.8vw,14px)]",
        "mobile:px-[0.35rem]",
        "mobile:h-[44px]",
        "mobile:leading-none",
    ],
    tablet: tw![
        "tablet:text-[clamp(11px,2.8vw,14px)]",
        "tablet:px-[0.35rem]",
        "tablet:h-[44px]",
        "tablet:leading-none",
    ],
}

// The active-race accent: the tab's hover border and its active border/text/glow all
// take the race colour, chosen directly from the race rather than a cascaded var.

states! {
    Race,
    Human => tw![
        "hover:border-[color:var(--color-race-human)]",
        "data-[active=true]:border-[color:var(--color-race-human)]",
        "data-[active=true]:text-[color:var(--color-race-human)]",
        "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-human)_30%,transparent)]",
    ],
    Nightelf => tw![
        "hover:border-[color:var(--color-race-nightelf)]",
        "data-[active=true]:border-[color:var(--color-race-nightelf)]",
        "data-[active=true]:text-[color:var(--color-race-nightelf)]",
        "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-nightelf)_30%,transparent)]",
    ],
    Orc => tw![
        "hover:border-[color:var(--color-race-orc)]",
        "data-[active=true]:border-[color:var(--color-race-orc)]",
        "data-[active=true]:text-[color:var(--color-race-orc)]",
        "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-orc)_30%,transparent)]",
    ],
    Undead => tw![
        "hover:border-[color:var(--color-race-undead)]",
        "data-[active=true]:border-[color:var(--color-race-undead)]",
        "data-[active=true]:text-[color:var(--color-race-undead)]",
        "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-undead)_30%,transparent)]",
    ],
    Neutral => tw![
        "hover:border-[color:var(--color-warcraft-gold)]",
        "data-[active=true]:border-[color:var(--color-warcraft-gold)]",
        "data-[active=true]:text-[color:var(--color-warcraft-gold)]",
        "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
    ],
}
