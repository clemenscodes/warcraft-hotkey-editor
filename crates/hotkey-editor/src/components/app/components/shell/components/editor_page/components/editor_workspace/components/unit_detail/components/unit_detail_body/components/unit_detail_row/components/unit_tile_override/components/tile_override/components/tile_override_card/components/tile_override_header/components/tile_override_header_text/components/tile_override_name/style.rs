use tw_macro::tw;
// The ability / unit name heading in the override panel. Gold display face, ellipsized
// on one line; smaller on the mobile panel.

classes! {
    base: tw![
        "m-0",
        "max-w-full",
        "overflow-hidden",
        "whitespace-nowrap",
        "text-ellipsis",
        "font-normal",
        "text-2xl",
        "leading-title",
        "text-warcraft-gold",
        "text-shadow-drop-92",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:[word-break:normal]",
    ],
    tablet: tw![
        "tablet:text-sm",
        "tablet:[word-break:normal]",
    ],
}
