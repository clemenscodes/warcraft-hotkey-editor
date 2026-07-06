use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-[0.08em]",
        "text-[2.8rem]",
        "leading-tight",
        "text-warcraft-text-secondary",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:[flex:1_1_auto]",
        "mobile:min-w-0",
        "mobile:text-[clamp(12px,3.4vw,15px)]",
        "mobile:tracking-[0.04em]",
        "mobile:leading-[1.25]",
        "mobile:whitespace-normal",
        "mobile:[overflow-wrap:break-word]",
        "mobile:[word-break:break-word]",
    ],
    tablet: tw![
        "tablet:[flex:1_1_auto]",
        "tablet:min-w-0",
        "tablet:text-[clamp(12px,3.4vw,15px)]",
        "tablet:tracking-[0.04em]",
        "tablet:leading-[1.25]",
        "tablet:whitespace-normal",
        "tablet:[overflow-wrap:break-word]",
        "tablet:[word-break:break-word]",
    ],
}
