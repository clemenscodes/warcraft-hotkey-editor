use super::state::UnitCardIdState;
use tw_macro::tw;

classes! {
    base: tw![
        "text-base",
        "leading-title",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
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

// Selected: the id text takes the card's race accent (chosen off the `data-race`
// attribute the component renders), at reduced opacity.

states! {
    UnitCardIdState,
    Normal => tw!["text-warcraft-text-faint"],
    Selected => tw![
        "text-warcraft-gold",
        "opacity-70",
        "data-[race=human]:text-race-human",
        "data-[race=orc]:text-race-orc",
        "data-[race=nightelf]:text-race-nightelf",
        "data-[race=undead]:text-race-undead",
        "data-[race=neutral]:text-warcraft-gold",
    ],
}
