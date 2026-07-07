use tw_macro::tw;
use warcraft_api::Race;

classes! {
    base: tw![
        "relative",
        "z-2",
        "py-1.5",
        "px-2.5",
        "pb-2",
        "w-full",
        "text-white",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:pt-1",
        "mobile:px-0.5",
        "mobile:pb-2",
        "mobile:text-xs",
        "mobile:tracking-[0.03em]",
    ],
    tablet: tw![
        "tablet:pt-1",
        "tablet:px-0.5",
        "tablet:pb-2",
        "tablet:text-xs",
        "tablet:tracking-[0.03em]",
    ],
}

states! {
    Race,
    Human => tw!["group-data-[active=true]:text-race-human"],
    Orc => tw!["group-data-[active=true]:text-race-orc"],
    Nightelf => tw!["group-data-[active=true]:text-race-nightelf"],
    Undead => tw!["group-data-[active=true]:text-race-undead"],
    Neutral => tw!["group-data-[active=true]:text-warcraft-gold"],
}
