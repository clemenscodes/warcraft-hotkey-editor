use tw_macro::tw;

// The selected id look: the shared id typography plus the race accent at reduced
// opacity, chosen off the `data-race` attribute the component renders.

classes! {
    base: tw![
        "text-base",
        "leading-title",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
        "text-warcraft-gold",
        "opacity-70",
        "data-[race=human]:text-race-human",
        "data-[race=orc]:text-race-orc",
        "data-[race=nightelf]:text-race-nightelf",
        "data-[race=undead]:text-race-undead",
        "data-[race=neutral]:text-warcraft-gold",
    ],
    mobile: tw![
        "mobile:block",
        "mobile:w-full",
        "mobile:text-xs",
        "mobile:leading-title",
    ],
    tablet: tw![
        "tablet:block",
        "tablet:w-full",
        "tablet:text-xs",
        "tablet:leading-title",
    ],
}
