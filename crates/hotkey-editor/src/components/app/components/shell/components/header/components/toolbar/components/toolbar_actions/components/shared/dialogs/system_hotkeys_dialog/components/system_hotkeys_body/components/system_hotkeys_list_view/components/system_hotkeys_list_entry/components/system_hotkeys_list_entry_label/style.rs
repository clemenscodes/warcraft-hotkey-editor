use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-[0.08em]",
        "text-3xl",
        "leading-tight",
        "text-warcraft-text-secondary",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:flex-[1_1_auto]",
        "mobile:min-w-0",
        "mobile:text-xs",
        "mobile:tracking-[0.04em]",
        "mobile:leading-title",
        "mobile:whitespace-normal",
        "mobile:wrap-break-word",
        "mobile:[word-break:break-word]",
    ],
    tablet: tw![
        "tablet:flex-[1_1_auto]",
        "tablet:min-w-0",
        "tablet:text-xs",
        "tablet:tracking-[0.04em]",
        "tablet:leading-title",
        "tablet:whitespace-normal",
        "tablet:wrap-break-word",
        "tablet:[word-break:break-word]",
    ],
}
