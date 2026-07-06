use tw_macro::tw;
classes! {
    base: tw![
        "m-0",
        "uppercase",
        "tracking-[0.1em]",
        "text-[2.1rem]/[1.35]",
        "text-warcraft-gold/85",
    ],
    mobile: tw![
        "mobile:text-[clamp(13px,3.5vw,16px)]",
        "mobile:tracking-[0.05em]",
    ],
    tablet: tw![
        "tablet:text-[clamp(13px,3.5vw,16px)]",
        "tablet:tracking-[0.05em]",
    ],
}
