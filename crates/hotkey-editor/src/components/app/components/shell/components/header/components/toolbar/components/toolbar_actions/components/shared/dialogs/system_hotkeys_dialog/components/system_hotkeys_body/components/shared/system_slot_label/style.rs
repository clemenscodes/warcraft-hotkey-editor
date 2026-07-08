use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-eyebrow",
        "text-3xl",
        "leading-none",
        "text-warcraft-gold/55",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-xs",
        "mobile:tracking-caps",
        "mobile:data-[compact=true]:text-xs",
    ],
    tablet: tw![
        "tablet:text-xs",
        "tablet:tracking-caps",
        "tablet:data-[compact=true]:text-xs",
    ],
}
