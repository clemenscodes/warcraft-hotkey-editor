use tw_macro::tw;
use warcraft_api::Race;

classes! {
    base: tw![
        "flex-1",
        "min-w-0",
        "min-h-11",
        "px-2",
        "bg-warcraft-bg-mid/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "text-warcraft-text-secondary",
        "text-sm",
        "tracking-label",
        "uppercase",
        "text-center",
        "cursor-pointer",
        "transition-all",
        "duration-fast",
        "whitespace-nowrap",
        "overflow-hidden",
        "text-ellipsis",
        "hover:bg-warcraft-blue-deep/70",
        "hover:text-white",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]", "kb-focus:shadow-focus",
        "data-[active=true]:bg-panel-blue",
    ],
    mobile: tw![
        "mobile:text-xs",
        "mobile:px-1.5",
        "mobile:h-11",
        "mobile:leading-none",
    ],
    tablet: tw![
        "tablet:text-xs",
        "tablet:px-1.5",
        "tablet:h-11",
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
        "data-[active=true]:text-race-human",
        "data-[active=true]:[--glow-color:var(--color-race-human)]", "data-[active=true]:shadow-glow-soft",
    ],
    Nightelf => tw![
        "hover:border-race-nightelf",
        "data-[active=true]:border-race-nightelf",
        "data-[active=true]:text-race-nightelf",
        "data-[active=true]:[--glow-color:var(--color-race-nightelf)]", "data-[active=true]:shadow-glow-soft",
    ],
    Orc => tw![
        "hover:border-race-orc",
        "data-[active=true]:border-race-orc",
        "data-[active=true]:text-race-orc",
        "data-[active=true]:[--glow-color:var(--color-race-orc)]", "data-[active=true]:shadow-glow-soft",
    ],
    Undead => tw![
        "hover:border-race-undead",
        "data-[active=true]:border-race-undead",
        "data-[active=true]:text-race-undead",
        "data-[active=true]:[--glow-color:var(--color-race-undead)]", "data-[active=true]:shadow-glow-soft",
    ],
    Neutral => tw![
        "hover:border-warcraft-gold",
        "data-[active=true]:border-warcraft-gold",
        "data-[active=true]:text-warcraft-gold",
        "data-[active=true]:shadow-glow-soft",
    ],
}
