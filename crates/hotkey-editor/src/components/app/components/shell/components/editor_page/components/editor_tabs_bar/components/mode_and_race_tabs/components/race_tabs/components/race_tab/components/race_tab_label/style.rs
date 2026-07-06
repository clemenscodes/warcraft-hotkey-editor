use tw_macro::tw;
use warcraft_api::Race;

classes! {
    base: tw![
        "relative",
        "z-[2]",
        "py-[0.4rem]",
        "px-[0.6rem]",
        "pb-[0.5rem]",
        "w-full",
        "text-white",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:pt-[0.3rem]",
        "mobile:px-[0.15rem]",
        "mobile:pb-[0.45rem]",
        "mobile:text-[clamp(9px,2.4vw,13px)]",
        "mobile:tracking-[0.03em]",
    ],
    tablet: tw![
        "tablet:pt-[0.3rem]",
        "tablet:px-[0.15rem]",
        "tablet:pb-[0.45rem]",
        "tablet:text-[clamp(9px,2.4vw,13px)]",
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
