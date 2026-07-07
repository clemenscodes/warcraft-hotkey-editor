use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-[0.18em]",
        "text-3xl",
        "leading-none",
        "text-warcraft-gold/55",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-xs",
        "mobile:tracking-[0.08em]",
        "mobile:data-[compact=true]:text-xs",
    ],
    tablet: tw![
        "tablet:text-xs",
        "tablet:tracking-[0.08em]",
        "tablet:data-[compact=true]:text-xs",
    ],
}
