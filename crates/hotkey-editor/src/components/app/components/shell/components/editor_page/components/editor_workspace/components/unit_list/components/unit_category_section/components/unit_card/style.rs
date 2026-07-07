use tw_macro::tw;
use warcraft_api::Race;

classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-4",
        "p-4",
        "w-full",
        "min-w-0",
        "text-left",
        "text-lg",
        "tracking-[0.02em]",
        "border",
        "rounded-tile",
        "transition-all",
        "duration-fast",
        "bg-warcraft-bg-mid/55",
        "border-warcraft-blue-deep",
        "text-warcraft-text-primary",
        "hover:bg-warcraft-blue-deep/70",
        "hover:text-white",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:bg-warcraft-blue/85",
        "kb-focus:shadow-focus",
        "data-[selected=true]:bg-panel-blue",
    ],
    mobile: tw![
        "mobile:flex-[1_0_auto]",
        "mobile:flex-row",
        "mobile:justify-start",
        "mobile:w-[min(54cqi,260px)]",
        "mobile:h-full",
        "mobile:@container-size",
        "mobile:py-2", "mobile:px-2.5",
        "mobile:gap-2.5",
        "mobile:snap-start",
        "mobile:box-border",
        "mobile:overflow-hidden",
        "mobile:bg-panel-dark",
        "mobile:border-warcraft-blue/60",
        "mobile:hover:border-warcraft-gold/35",
        "mobile:data-[selected=true]:bg-panel-blue",
        "mobile:group-[[data-search-active=false][data-active-category=hero]]:[&:not([data-unit-kind=hero])]:hidden",
        "mobile:group-[[data-search-active=false][data-active-category=soldier]]:[&:not([data-unit-kind=soldier])]:hidden",
        "mobile:group-[[data-search-active=false][data-active-category=worker]]:[&:not([data-unit-kind=worker])]:hidden",
        "mobile:group-[[data-search-active=false][data-active-category=building]]:[&:not([data-unit-kind=building])]:hidden",
    ],
    tablet: tw![
        "tablet:flex-[1_0_auto]",
        "tablet:flex-row",
        "tablet:justify-start",
        "tablet:w-[min(54cqi,260px)]",
        "tablet:h-full",
        "tablet:@container-size",
        "tablet:py-2", "tablet:px-2.5",
        "tablet:gap-2.5",
        "tablet:snap-start",
        "tablet:box-border",
        "tablet:overflow-hidden",
        "tablet:bg-panel-dark",
        "tablet:border-warcraft-blue/60",
        "tablet:hover:border-warcraft-gold/35",
        "tablet:data-[selected=true]:bg-panel-blue",
        "tablet:group-[[data-search-active=false][data-active-category=hero]]:[&:not([data-unit-kind=hero])]:hidden",
        "tablet:group-[[data-search-active=false][data-active-category=soldier]]:[&:not([data-unit-kind=soldier])]:hidden",
        "tablet:group-[[data-search-active=false][data-active-category=worker]]:[&:not([data-unit-kind=worker])]:hidden",
        "tablet:group-[[data-search-active=false][data-active-category=building]]:[&:not([data-unit-kind=building])]:hidden",
    ],
}

// The active-race accent: the hover border (desktop; mobile/tablet keep their own
// gold hover from the bands) and the selected border/text/glow all take the race
// colour, chosen directly from the race rather than a cascaded var. The selected
// glow is one blur for every width (the former 10px mobile/tablet variant folds into
// this 8px — a 2px difference that no longer justifies a band-specific race style).

states! {
    Race,
    Human => tw![
        "hover:border-race-human",
        "data-[selected=true]:border-race-human",
        "data-[selected=true]:text-race-human",
        "data-[selected=true]:[--glow-color:var(--color-race-human)]", "data-[selected=true]:shadow-glow-soft",
    ],
    Nightelf => tw![
        "hover:border-race-nightelf",
        "data-[selected=true]:border-race-nightelf",
        "data-[selected=true]:text-race-nightelf",
        "data-[selected=true]:[--glow-color:var(--color-race-nightelf)]", "data-[selected=true]:shadow-glow-soft",
    ],
    Orc => tw![
        "hover:border-race-orc",
        "data-[selected=true]:border-race-orc",
        "data-[selected=true]:text-race-orc",
        "data-[selected=true]:[--glow-color:var(--color-race-orc)]", "data-[selected=true]:shadow-glow-soft",
    ],
    Undead => tw![
        "hover:border-race-undead",
        "data-[selected=true]:border-race-undead",
        "data-[selected=true]:text-race-undead",
        "data-[selected=true]:[--glow-color:var(--color-race-undead)]", "data-[selected=true]:shadow-glow-soft",
    ],
    Neutral => tw![
        "hover:border-warcraft-gold",
        "data-[selected=true]:border-warcraft-gold",
        "data-[selected=true]:text-warcraft-gold",
        "data-[selected=true]:shadow-glow-soft",
    ],
}
