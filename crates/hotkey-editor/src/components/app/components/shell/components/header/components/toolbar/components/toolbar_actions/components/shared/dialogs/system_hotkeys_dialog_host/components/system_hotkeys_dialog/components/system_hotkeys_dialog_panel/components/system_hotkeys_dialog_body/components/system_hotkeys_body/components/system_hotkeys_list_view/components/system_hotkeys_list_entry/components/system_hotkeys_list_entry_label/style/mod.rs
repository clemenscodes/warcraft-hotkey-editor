use tw_macro::tw;
classes! {
    base: tw![
        "uppercase",
        "tracking-caps",
        "text-3xl",
        "leading-title",
        "text-warcraft-text-secondary",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:flex-[1_1_auto]",
        "mobile:min-w-0",
        "mobile:text-xs",
        "mobile:tracking-label",
        "mobile:leading-title",
        "mobile:whitespace-normal",
        "mobile:wrap-break-word",
        "mobile:[word-break:break-word]",
    ],
    tablet: tw![
        "tablet:flex-[1_1_auto]",
        "tablet:min-w-0",
        "tablet:text-xs",
        "tablet:tracking-label",
        "tablet:leading-title",
        "tablet:whitespace-normal",
        "tablet:wrap-break-word",
        "tablet:[word-break:break-word]",
    ],
}
