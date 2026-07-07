use tw_macro::tw;
use warcraft_api::Race;

classes! {
    base: tw![
        "flex-1",
        "min-w-0",
        "min-h-[44px]",
        "px-2",
        "bg-warcraft-bg-mid/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "text-warcraft-text-secondary",
        "text-sm",
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
        "mobile:text-xs",
        "mobile:px-1.5",
        "mobile:h-[44px]",
        "mobile:leading-none",
    ],
    tablet: tw![
        "tablet:text-xs",
        "tablet:px-1.5",
        "tablet:h-[44px]",
        "tablet:leading-none",
    ],
}

// The active-race accent: the tab's hover border and its active border/text/glow all
// take the race colour, chosen directly from the race rather than a cascaded var.

states! {
    Race,
    Human => tw![
        "hover:border-race-human",
        "data-[active=true]:border-race-human",
        "data-[active=true]:text-[color:var(--color-race-human)]",
        "data-[active=true]:shadow-glow-human-soft",
    ],
    Nightelf => tw![
        "hover:border-race-nightelf",
        "data-[active=true]:border-race-nightelf",
        "data-[active=true]:text-[color:var(--color-race-nightelf)]",
        "data-[active=true]:shadow-glow-nightelf-soft",
    ],
    Orc => tw![
        "hover:border-race-orc",
        "data-[active=true]:border-race-orc",
        "data-[active=true]:text-[color:var(--color-race-orc)]",
        "data-[active=true]:shadow-glow-orc-soft",
    ],
    Undead => tw![
        "hover:border-race-undead",
        "data-[active=true]:border-race-undead",
        "data-[active=true]:text-[color:var(--color-race-undead)]",
        "data-[active=true]:shadow-glow-undead-soft",
    ],
    Neutral => tw![
        "hover:border-warcraft-gold",
        "data-[active=true]:border-warcraft-gold",
        "data-[active=true]:text-[color:var(--color-warcraft-gold)]",
        "data-[active=true]:shadow-glow-8-2",
    ],
}
