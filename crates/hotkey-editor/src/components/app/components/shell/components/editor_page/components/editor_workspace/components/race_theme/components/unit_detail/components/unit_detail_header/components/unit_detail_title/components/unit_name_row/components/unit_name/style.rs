use tw_macro::tw;
classes! {
    base: tw![
        "text-3xl",
        "leading-flush",
        "font-normal",
        "text-warcraft-gold",
        "text-shadow-drop",
        "m-0",
        "tracking-snug",
    ],
    mobile: tw![
        "mobile:flex-auto",
        "mobile:min-w-0",
        "mobile:text-lg",
        "mobile:leading-title",
        "mobile:text-left",
        "mobile:wrap-break-word",
        "mobile:[word-break:break-word]",
    ],
    tablet: tw![
        "tablet:flex-auto",
        "tablet:min-w-0",
        "tablet:text-lg",
        "tablet:leading-title",
        "tablet:text-left",
        "tablet:wrap-break-word",
        "tablet:[word-break:break-word]",
    ],
}
