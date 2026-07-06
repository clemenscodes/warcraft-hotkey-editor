use tw_macro::tw;
classes! {
    base: tw![
        "m-0",
        "text-[2.75rem]",
        "uppercase",
        "tracking-[0.08em]",
        "text-inherit",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-[clamp(17px,5vw,24px)]",
        "mobile:tracking-[0.06em]",
        "mobile:text-warcraft-gold",
    ],
    tablet: tw![
        "tablet:text-[clamp(17px,5vw,24px)]",
        "tablet:tracking-[0.06em]",
        "tablet:text-warcraft-gold",
    ],
}
