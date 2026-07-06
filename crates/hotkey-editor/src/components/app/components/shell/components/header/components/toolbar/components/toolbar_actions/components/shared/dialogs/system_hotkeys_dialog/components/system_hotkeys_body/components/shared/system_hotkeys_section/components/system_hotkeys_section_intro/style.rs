use tw_macro::tw;
classes! {
    base: tw![
        "m-0",
        "max-w-[90rem]",
        "text-center",
        "uppercase",
        "tracking-[0.1em]",
        "text-[2rem]",
        "leading-snug",
        "text-warcraft-gold/75",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:max-w-full",
        "mobile:px-[0.25rem]",
        "mobile:text-[clamp(11px,3vw,14px)]",
        "mobile:tracking-[0.04em]",
        "mobile:leading-[1.35]",
    ],
    tablet: tw![
        "tablet:max-w-full",
        "tablet:px-[0.25rem]",
        "tablet:text-[clamp(11px,3vw,14px)]",
        "tablet:tracking-[0.04em]",
        "tablet:leading-[1.35]",
    ],
}
