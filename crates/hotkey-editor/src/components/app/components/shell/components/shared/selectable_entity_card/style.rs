use super::state::CardAccent;
use tw_macro::tw;

// The shared selectable-entity-card surface: the button shell behind an editor unit
// card and a collision-sidebar card. `classes!` holds the look and interior shared
// verbatim by both — the resting panel fill, the hairline blue border, the rounded
// tile, the hover and keyboard-focus treatment, the selected `bg-panel-blue`, and the
// carousel interior for the mobile/tablet horizontal scroller. `group` is set so a
// descendant (the island coordinate) can read `group-data-[selected=true]`. The
// per-accent resting/hover/selected border, text, and glow are layered on by
// `states!`; each card's own wrapper owns its placement box and its data attributes.

classes! {
    base: tw![
        "group",
        "flex",
        "items-center",
        "gap-4",
        "p-4",
        "w-full",
        "min-w-0",
        "text-left",
        "text-lg",
        "tracking-snug",
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
        "mobile:h-full",
        "mobile:py-2",
        "mobile:px-2.5",
        "mobile:gap-2.5",
        "mobile:box-border",
        "mobile:overflow-hidden",
        "mobile:bg-panel-dark",
        "mobile:border-warcraft-blue/60",
    ],
    tablet: tw![
        "tablet:h-full",
        "tablet:py-2",
        "tablet:px-2.5",
        "tablet:gap-2.5",
        "tablet:box-border",
        "tablet:overflow-hidden",
        "tablet:bg-panel-dark",
        "tablet:border-warcraft-blue/60",
    ],
}

states! {
    CardAccent,
    Human => tw![
        "hover:border-race-human",
        "data-[selected=true]:border-race-human",
        "data-[selected=true]:text-race-human",
        "data-[selected=true]:[--glow-color:var(--color-race-human)]",
        "data-[selected=true]:shadow-glow-soft",
    ],
    Orc => tw![
        "hover:border-race-orc",
        "data-[selected=true]:border-race-orc",
        "data-[selected=true]:text-race-orc",
        "data-[selected=true]:[--glow-color:var(--color-race-orc)]",
        "data-[selected=true]:shadow-glow-soft",
    ],
    Undead => tw![
        "hover:border-race-undead",
        "data-[selected=true]:border-race-undead",
        "data-[selected=true]:text-race-undead",
        "data-[selected=true]:[--glow-color:var(--color-race-undead)]",
        "data-[selected=true]:shadow-glow-soft",
    ],
    Nightelf => tw![
        "hover:border-race-nightelf",
        "data-[selected=true]:border-race-nightelf",
        "data-[selected=true]:text-race-nightelf",
        "data-[selected=true]:[--glow-color:var(--color-race-nightelf)]",
        "data-[selected=true]:shadow-glow-soft",
    ],
    Neutral => tw![
        "hover:border-warcraft-gold",
        "data-[selected=true]:border-warcraft-gold",
        "data-[selected=true]:text-warcraft-gold",
        "data-[selected=true]:shadow-glow-soft",
    ],
    CollisionGold => tw![
        "hover:border-warcraft-blue",
        "data-[selected=true]:border-warcraft-gold",
        "data-[selected=true]:text-warcraft-gold",
        "data-[selected=true]:shadow-glow-soft",
    ],
}
